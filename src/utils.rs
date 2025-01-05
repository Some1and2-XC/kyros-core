/*
  File for general utilities
*/

extern crate minijinja;

use flate2::write::ZlibEncoder;
use flate2::{Compression, FlushCompress};
use minijinja::{context, Environment};
use png::chunk::IDAT;
use png::Filter;
use vulkano::buffer::allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo};
use vulkano::buffer::{BufferUsage, Subbuffer};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyImageToBufferInfo};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::{Pipeline, PipelineBindPoint};
use vulkano::sync::{self, GpuFuture};
use vulkano::VulkanError;

use super::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Cursor, Read, Write};
use std::path::Path;
use std::str;
use std::sync::Arc;
use crate::gpu::run_glsl;
use crate::colors::profiles::get_profile;
use log::{self, debug, info};

/// Function for getting image from configuration and generator function.
pub fn cpu_eval(config: &Config) -> Result<(), Box<dyn Error>> {

    let save_method = get_save_method(config.save_method.as_str());

    let color_function = get_color(&config.color_formula.as_str());
    let shadow_function = get_shadow(&config.shadow_formula.as_str());
    let generator_function = get_formula(&config.gen_formula.as_str());

    // Sets Initial 'c' Value (If set)
    let mut c = Complex { real: 0f32, imaginary: 0f32, };
    let is_julia: bool = match config.c_init {
        Some(value) => {
            c = value;
            true
        },
        None => false,
    };

    // Sets Math Values
    let x_math_space_factor = config.math_frame.factor_x;
    let y_math_space_factor = config.math_frame.factor_y;

    let x_math_space_offset = config.math_frame.offset_x;
    let y_math_space_offset = config.math_frame.offset_y;

    let mut z: Complex;
    let mut old_z: Complex;

    let max_i = config.max_i as f64;

    let color_profile = get_profile(&config);

    // Initializes Image Buffer
    let mut img: Vec<u8> = Vec::with_capacity(4 * (config.size_x * config.size_y) as usize);

    // Goes through each pixel
    for i in 0..config.size_y {
        for j in 0..config.size_x {

            // Sets Initial Z Value
            z = Complex {
                real      : x_math_space_factor * j as f32 + x_math_space_offset,
                imaginary : y_math_space_factor * i as f32 + y_math_space_offset,
            };
            old_z = z;

            if is_julia == false { c = z; }

            let mut z_output: f32 = 0.0;

            // Runs Math
            for _iteration in 0..config.max_i {
                if z.is_greater(2.0) { break }
                z = generator_function.method(c, z);

                // Calculates Output
                if !config.travel_distance {
                    z_output += 1.0;
                } else {
                    z_output += (
                        (z.real - old_z.real) * (z.real - old_z.real) +
                        (z.imaginary - old_z.imaginary) * (z.imaginary - old_z.imaginary)
                    ).sqrt();
                    old_z = z;
                }
            }

            // Adds a pixel
            img.extend(
                {
                    let out = match z_output {
                        x if x == 0.0 => color_profile.get_background().to_owned(),
                        x if (x >= max_i as f32 && !config.travel_distance) => color_profile.get_foreground().to_owned(),
                        _ => color_profile.method(
                            color_function.method(z_output as f64, &config).rem_euclid(360.0),
                            shadow_function.method(z_output as f64).rem_euclid(360.0),
                        ),
                    };
                    out[0..(3 + config.rgba as usize)].to_owned().iter()
                }
            );
        }
        if config.logs >= Level::Info {
            print!("\t {:.2}% | {} / {}\r", 100.0 * (i as f64 + 1.0) / config.size_y as f64, i+1, config.size_y);
        }
    }
    if config.logs >= Level::Info {
        println!();
    }

    return save_method.method(img.as_slice(), config);
}

static TEMPLATE: &str = include_str!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/comp.glsl")
);

pub fn gpu_eval(config: &Config) -> Result<(), Box<dyn Error>> {

    /// Takes a Vec<f64> and returns a string that looks like 1.00000, 2.00000, 3.00000
    /// Returns Option<None> if the result isn't the expected length
    fn get_arr_str_with_len(in_arr: Vec<f64>, expected_length: usize) -> Option<String> {
        if in_arr.len() != expected_length {
            log::debug!("Invalid length of array.");
            log::debug!("Expected length: `{expected_length}`.");
            log::debug!("Got Array: `{in_arr:?}`.");
            return None;
        }
        return Some(in_arr
            .iter()
            .map(|v| format!("{v:.5?}"))
            .collect::<Vec<String>>()
            .join(", ")
            .into()
        );
    }

    let color_function = get_color(&config.color_formula.as_str());
    let shadow_function = get_shadow(&config.shadow_formula.as_str());
    let generator_function = get_formula(&config.gen_formula.as_str());

    // Sets value for math constant 'c'
    let c: [f64; 2] = match config.c_init {
        Some(value) => {
            [value.real as f64, value.imaginary as f64]
        },
        None => [0.0, 0.0],
    };

    let _color_profile = get_profile(&config);

    let compiled_shader = {
        let mut env = Environment::new();
        env.add_template(
            "compute_shader",
            TEMPLATE
            ).unwrap();
        let compute_shader = env.get_template("compute_shader").unwrap();

        // Big mess that passes all the values to the Jinja template
        compute_shader.render(context!(
            formula => generator_function.gpu_method(),
            travel_distance => format!("{:?}", config.travel_distance),
            rate_of_color_change => format!("{:.1}", config.rate_of_color_change),
            background => get_arr_str_with_len(config.background.to_array().into(), 4).unwrap(),
            foreground => get_arr_str_with_len(config.foreground.to_array().into(), 4).unwrap(),
            max_i => format!("{:}", config.max_i),
            c_init => get_arr_str_with_len(c.into(), 2).unwrap(),
            colors => color_function.gpu_method(),
            shadows => shadow_function.gpu_method(),
            julia_changes => match config.c_init {
                Some(_) => "", // Is julia settings
                None => "c = z;", // Mandelbrot settings
            }

            ))
            .unwrap()
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\r", "\r")
    };

    log::debug!("{}", compiled_shader);

    let now = Instant::now();

    let (device, pipeline, mut queues) = run_glsl(&now, compiled_shader)?;

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
        device.clone(),
        Default::default(),
    ));

    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
        device.clone(),
        Default::default(),
    ));

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

    let layout = &pipeline.layout().set_layouts();

    let queue = queues.next().unwrap();
    let queue_family_index = device.active_queue_family_indices().first().ok_or(VulkanError::InitializationFailed)?.clone();

    // Gets the amount of lines per chunk
    let mut amnt_of_lines_per_chunk = config.chunk_sizes.unwrap_or((config.size_x * config.size_y) as u64) / (config.size_x as u64);
    let original_amnt_of_lines_per_chunk = amnt_of_lines_per_chunk;
    // Gets the amount of chunks to generate
    let generation_count = (config.size_y as u64).div_ceil(amnt_of_lines_per_chunk);

    let filename = format!("{}.png", config.filename);
    let path = Path::new(&filename);
    let file = File::create(path).expect(&format!("Failed to create file..? Filename: `{}`", config.filename));
    let ref mut w = BufWriter::new(file);

    let mut info = png::Info::with_size(config.size_x, config.size_y);
    info.bit_depth = png::BitDepth::Eight;
    info.color_type = png::ColorType::Rgba;

    let mut encoder = png::Encoder::with_info(w, info.clone())?;
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let config_information = serde_json::to_string_pretty(&config).unwrap_or("FAILED TO SERIALIZE CONFIG! (Serde Error)".to_string());
    debug!("{}", config_information);
    encoder.add_ztxt_chunk("kyros_config".to_string(), config_information)?;
    encoder.set_compression(png::Compression::High);
    encoder.set_filter(Filter::NoFilter);
    // encoder.set_compression(Compression::Best); // This is for the official image
    let mut writer = encoder.write_header()?;

    // Value of the amount of bytes in the buffer.
    // MUST be recalculated if `amnt_of_lines_per_chunk` changes.
    let mut buf_length = (amnt_of_lines_per_chunk * config.size_x as u64 * 4) as usize;
    let original_buf_length = buf_length;

    let mut data_buffer_content;

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [config.size_x, original_amnt_of_lines_per_chunk as u32, 1],
            usage: ImageUsage::STORAGE | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )?;

    let view = ImageView::new_default(image.clone())?;

    // Here we setup the descriptor sets.
    let image_desc_set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        layout[0].clone(),
        [WriteDescriptorSet::image_view(0, view.clone())],
        [],
        )?;

    let mut current_math_frame = MathFrame {
        factor_x: config.math_frame.factor_x * config.size_x as f32,
        factor_y: config.math_frame.factor_y * config.size_y as f32,
        offset_x: config.math_frame.offset_x,
        offset_y: config.math_frame.offset_y,
    };

    let original_factor_y = current_math_frame.factor_y;

    let mut compressor = ZlibEncoder::new(Cursor::new(Vec::new()), Compression::best());
    // This is set to buf_length even though with compression this just almost always be less.
    let mut compressed_output = Vec::with_capacity(buf_length);

    for i in 0..generation_count {

        if i == generation_count - 1 && i != 0 {
            // We can reassign becaues this should be the last iteration
            amnt_of_lines_per_chunk = config.size_y as u64 - (amnt_of_lines_per_chunk * i);
            buf_length = (amnt_of_lines_per_chunk * config.size_x as u64 * 4) as usize;
            debug!("Writing Partial image fragment!");
        }

        current_math_frame.factor_y = original_factor_y * amnt_of_lines_per_chunk as f32 / config.size_y as f32;

        let data_buffer: Subbuffer<[u8]> = buffer_allocator
            .allocate_unsized(original_buf_length as u64)?;

        let mut builder = AutoCommandBufferBuilder::primary(
                &command_buffer_allocator,
                queue_family_index,
                CommandBufferUsage::MultipleSubmit,
            )?;

        println!("Math Frame: {:?}", current_math_frame);

        builder
            .bind_pipeline_compute(pipeline.clone())?
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                pipeline.layout().clone(),
                0,
                image_desc_set.clone(),
            )?
            .push_constants(pipeline.layout().clone(), 0, current_math_frame.clone())?
            .dispatch([config.size_x / 16, original_amnt_of_lines_per_chunk as u32 / 16, 1])?
            .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
                image.clone(),
                data_buffer.clone(),
            ))?;

        let command_buffer = builder.build()?;
        let future = sync::now(device.clone())
            .then_execute(queue.clone(), command_buffer)?
            .then_signal_fence_and_flush()?;
        future.wait(None)?;

        data_buffer_content = {
            let read_values = data_buffer.read()?;
            let (values, _) = read_values.split_at(buf_length);
            let mut out_data = Vec::with_capacity(buf_length as usize + config.size_x as usize);

            for chunk in values.chunks(config.size_x as usize) {

                let mut vec_chunk = chunk.to_vec();
                out_data.push(0u8);
                out_data.append(&mut vec_chunk);

            }

            println!("Out Data: {:?}", out_data);
            println!("Vec_Chunk size: {}", values.len());

            out_data


        };

        // info!("Passed Buffer Data: {:?}", data_buffer_content);
        info!("Passed Buffer Size: {}", data_buffer_content.len());
        info!("Expected: {}", buf_length + config.size_x as usize);

        let bytes_written = compressor.write(&compressed_output)?;
        info!("Compressed Buffer with {} bytes written", bytes_written);

        let _ = compressor.flush()?;
        let _bytes_read = compressor.read(&mut compressed_output)?;
        // info!("Compressed Buffer with {} bytes read", bytes_read);

        writer.write_chunk(IDAT, &compressed_output)?;
        compressed_output.clear();
        // writer.write_image_data(&data_buffer_content)?;

        current_math_frame.offset_y += original_factor_y * amnt_of_lines_per_chunk as f32 / config.size_y as f32;

    }

    debug!("Remaining data (from cursor): {:?}", compressor.flush_finish()?);

    // writer.write_chunk(IDAT, &curs.into_inner())?;


    info!("Validating Data...");
    writer.finish()?;

    log::info!("{:.2?}: Finished GPU Execution", now.elapsed());

    return Ok(());

    // let save_method = save::get_save_method(config.save_method.as_str());
    // return save_method.method(&data_buffer_content[..], config);

}
