extern crate vulkano;
extern crate image;
extern crate shaderc;

use crate::structs::MathFrame;

use ahash::HashMapExt;
use shaderc::CompilationArtifact;
use core::panic;
use std::{
    collections::HashSet, error::Error, time::Instant
};

use vulkano::{
    descriptor_set::layout::{DescriptorSetLayout, DescriptorSetLayoutCreateFlags, DescriptorSetLayoutCreateInfo, DescriptorType}, device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Features, Queue, QueueCreateInfo, QueueFlags
    }, instance::{
        Instance,
        InstanceCreateFlags,
        InstanceCreateInfo,
    }, pipeline::{
        compute::ComputePipelineCreateInfo, layout::{PipelineLayoutCreateFlags, PipelineLayoutCreateInfo, PushConstantRange}, ComputePipeline, PipelineLayout, PipelineShaderStageCreateInfo
    }, shader::{
        DescriptorBindingRequirements, DescriptorRequirements, ShaderModule, ShaderModuleCreateInfo, ShaderStages
    }, VulkanError, VulkanLibrary
};

use std::sync::Arc;

const MINIMAL_FEATURES: Features = Features {
    geometry_shader: true,
    ..Features::empty()
};

fn compile_to_spirv(glsl: String, kind: shaderc::ShaderKind, entry_point_name: &str) -> CompilationArtifact {

    let compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();

    options.add_macro_definition("EP", Some(entry_point_name));

    let filename = "comp.glsl";

    match compiler
        .compile_into_spirv(&glsl, kind, filename, entry_point_name, Some(&options)) {
            Ok(v) => v,
            Err(e) => {
                let lines = glsl
                    .split("\n")
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    ;

                // Gets the string length of the length of the lines array
                // let v = 0..100;
                // max_str_length of v would be 3
                let max_str_length = lines
                    .len()
                    .to_string()
                    .len()
                    ;

                println!("Generated GLSL (filename: {filename})");

                for (i, line) in lines.iter().enumerate() {
                    println!(
                        "{}{} {}",
                        " ".repeat(max_str_length - (i + 1).to_string().len()),
                        i + 1,
                        line,
                    );
                }
                panic!(
                    "{}\n{}",
                    e,
                    "SPIR-V GPU Compiler Error! Try checking the GLSL code above and the line number at the top for more information.",
                );
            }
    }
}

pub fn run_glsl(now: &Instant, glsl: String) -> Result<(Arc<Device>, Arc<ComputePipeline>, impl ExactSizeIterator<Item = Arc<Queue>>), Box<dyn Error>> {

    // Boilerplate Initialization
    let library = match VulkanLibrary::new() {
        Ok(v) => v,
        Err(e) => {
            println!("Can't initialize Vulkan Library with error: '{:?}'. (is vulkan installed?)", e.to_string());
            std::process::exit(1);
        }
    };
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
        // InstanceCreateInfo::application_from_cargo_toml(),
    )?;

    let device_extensions = DeviceExtensions {
        khr_storage_buffer_storage_class: true,
        ..DeviceExtensions::empty()
    };

    // Getting the Device
    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()?
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .position(|q| q.queue_flags.intersects(QueueFlags::COMPUTE))
                .map(|i| (p, i.try_into().unwrap()))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .ok_or(Box::new(VulkanError::InitializationFailed))?;

    log::info!(
        "{:.2?}: Detected Device: {} (Type: {:?})",
        now.elapsed(),
        physical_device.properties().device_name,
        physical_device.properties().device_type,
    );

    if !physical_device.supported_features().contains(&MINIMAL_FEATURES) {
        return Err(Box::new(VulkanError::FeatureNotPresent));
    }

    let (device, queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        }
    )?;

    let entry_point = "main";

    let cs = {
        unsafe {
            ShaderModule::new(
                device.clone(),
                ShaderModuleCreateInfo::new(
                    compile_to_spirv(
                        glsl,
                        shaderc::ShaderKind::Compute,
                        entry_point)
                        .as_binary(),
                ),
            )?
        }
    };

    let pipeline = {

        let stage = PipelineShaderStageCreateInfo::new(
            cs.entry_point(entry_point).ok_or(Box::new(VulkanError::ValidationFailed))?
            );

        let layout = PipelineLayout::new(device.clone(), PipelineLayoutCreateInfo {
            flags: PipelineLayoutCreateFlags::empty(),
            set_layouts: vec![
                DescriptorSetLayout::new(device.clone(), DescriptorSetLayoutCreateInfo {
                    flags: DescriptorSetLayoutCreateFlags::empty(),
                    bindings: [(0, (&DescriptorBindingRequirements {
                        descriptor_types: vec![DescriptorType::StorageImage],
                        descriptor_count: Some(1), // This implies that the size gets decided at runtime
                        stages: ShaderStages::COMPUTE,
                        descriptors: {
                            let mut map = ahash::HashMap::new();
                            map.insert(Some(0), DescriptorRequirements {
                                memory_read: ShaderStages::COMPUTE,
                                memory_write: ShaderStages::COMPUTE,
                                sampler_compare: false,
                                sampler_no_unnormalized_coordinates: false,
                                sampler_no_ycbcr_conversion: false,
                                sampler_with_images: HashSet::default(),
                                storage_image_atomic: false,
                            });
                            map
                        },
                        ..Default::default()
                    }).into())].into(),
                    ..Default::default()
                })?,
            ],
            push_constant_ranges: vec![
                PushConstantRange {
                    stages: ShaderStages::COMPUTE,
                    offset: 0,
                    size: std::mem::size_of::<MathFrame>() as u32,
                }
            ],
            ..Default::default()
        })?;

        ComputePipeline::new(
            device.clone(),
            None,
            ComputePipelineCreateInfo::stage_layout(stage, layout),
        )?
    };

    log::info!("{:.2?}: Compiled Shaders", now.elapsed());

    return Ok((device, pipeline, queues));

}
