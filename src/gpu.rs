extern crate vulkano;
extern crate image;
extern crate shaderc;

use shaderc::CompilationArtifact;
use core::panic;
use std::{
    error::Error, time::Instant,
};

use vulkano::{
    buffer::{
        allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo}, BufferUsage, Subbuffer
    }, command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage, CopyImageToBufferInfo
    }, descriptor_set::{
        allocator::StandardDescriptorSetAllocator,
        persistent::PersistentDescriptorSet,
        WriteDescriptorSet,
    }, device::{
        physical::PhysicalDeviceType,
        Device,
        DeviceCreateInfo,
        DeviceExtensions,
        Features,
        QueueCreateInfo,
        QueueFlags,
    }, format::Format, image::{
        view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage
    }, instance::{
        Instance,
        InstanceCreateFlags,
        InstanceCreateInfo,
    }, memory::allocator::{
        AllocationCreateInfo,
        MemoryTypeFilter,
        StandardMemoryAllocator,
    }, pipeline::{
        compute::ComputePipelineCreateInfo, layout::PipelineDescriptorSetLayoutCreateInfo, ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo
    }, shader::{
        ShaderModule, ShaderModuleCreateInfo,
    }, sync::{
        self,
        GpuFuture
    }, VulkanLibrary
};

use std::sync::Arc;

use crate::{save, structs::Config};

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
                        " ".repeat(max_str_length - i.to_string().len()),
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

pub fn run_glsl(glsl: String, config: &Config) -> Result<(), Box<dyn Error>> {

    let save_method = save::get_save_method(config.save_method.as_str());

    let image_width = config.size_x;
    let image_height = config.size_y;

    let now = Instant::now();

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
    )
    .unwrap();

    let device_extensions = DeviceExtensions {
        khr_storage_buffer_storage_class: true,
        ..DeviceExtensions::empty()
    };

    // Getting the Device
    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .unwrap()
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
        .unwrap();

    log::info!(
        "{:.2?}: Detected Device: {} (Type: {:?})",
        now.elapsed(),
        physical_device.properties().device_name,
        physical_device.properties().device_type,
    );

    if !physical_device.supported_features().contains(&MINIMAL_FEATURES) {
        panic!("Physical device has insufficient features for this application.");
    }

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        }
    )
    .unwrap();

    let queue = queues.next().unwrap();

    let pipeline = {
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
                ).unwrap()
            }
        };

        let stage = PipelineShaderStageCreateInfo::new(
            cs.entry_point(entry_point).unwrap()
            );
        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
                .into_pipeline_layout_create_info(device.clone())
                .unwrap(),
            ).unwrap();

        ComputePipeline::new(
            device.clone(),
            None,
            ComputePipelineCreateInfo::stage_layout(stage, layout),
        ).unwrap()
    };
    log::info!("{:.2?}: Compiled Shaders", now.elapsed());

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
        device.clone(),
        Default::default(),
    ));

    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
        device.clone(),
        Default::default(),
    ));

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [image_width, image_height, 1],
            usage: ImageUsage::STORAGE | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                ,
            ..Default::default()
        },
    ).unwrap();

    let view = ImageView::new_default(image.clone()).unwrap();

    let buffer_allocator = SubbufferAllocator::new(
        memory_allocator.clone(),
        SubbufferAllocatorCreateInfo {
            buffer_usage: BufferUsage::TRANSFER_DST,
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_RANDOM_ACCESS
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
                ,
            ..Default::default()
        },
    );

    let buf_length = image_height as u64 * image_width as u64 * 4;

    let data_buffer: Subbuffer<[u8]> = match buffer_allocator
        .allocate_unsized(buf_length) {
            Ok(v) => v,
            Err(_) => {
                panic!("Unable to allocate '{buf_length}'!");
            },
    };

    let layout = &pipeline.layout().set_layouts()[0];
    let set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        layout.clone(),
        [
            WriteDescriptorSet::image_view(0, view.clone()),
        ],
        [],
    )
    .unwrap();

    let mut builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .bind_pipeline_compute(pipeline.clone())
        .unwrap()
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            pipeline.layout().clone(),
            0,
            set,
        )
        .unwrap()
        ;

    builder
        .dispatch([image_height / 16, image_width / 16, 1])
        .unwrap()
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
            image.clone(),
            data_buffer.clone(),
        ))
        .unwrap()
        ;

    let command_buffer = builder.build().unwrap();
    let future = sync::now(device)
        .then_execute(queue, command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();
    future.wait(None).unwrap();

    let data_buffer_content = data_buffer.read().unwrap();
    log::info!("{:.2?}: Finished GPU Execution", now.elapsed());

    return save_method.method(&data_buffer_content[..], config);
}
