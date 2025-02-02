use gzp::deflate::Zlib;
use gzp::par::compress::{ParCompress, ParCompressBuilder};
use gzp::{Compression, ZWriter};
use indicatif::ProgressBar;
use open_writer::OpenWriter;
use png::chunk::IDAT;
use png::Filter;
use tokio::sync::mpsc::{channel, unbounded_channel, Receiver, UnboundedReceiver};

use super::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use log::{self, debug, info};

/// Method for running on the chunk writing thread.
pub async fn handle_compression_thread_instructions(config: Config, mut compression_bar: ProgressBar, data_write_bar: ProgressBar, generation_count: u32, mut rx: Receiver<Vec<u8>>) -> Option<()> {

    info!("Started compression thread...");

    // A channel for compressed data
    let (comp_tx, comp_rx) = unbounded_channel::<Vec<u8>>();
    let (data_write_tx, data_write_rx) = channel::<()>(2);

    let handle_data_thread = tokio::spawn(handle_data_thread_instructions(config.clone(), data_write_bar, comp_rx, data_write_rx));

    let compressed_data_writer = OpenWriter::new(comp_tx);

    let mut zlib_encoder: ParCompress<Zlib> = ParCompressBuilder::new()
        .num_threads(config.compression_threads as usize).unwrap()
        .compression_level(Compression::new(config.compression))
        .from_writer(compressed_data_writer)
        ;

    // Then we go through the amount of chunks we are going to make.
    for _i in 0..generation_count {
        // The `.recv()` method waits until either no chunks can be passed
        let data = rx.recv().await?;

        // Compresses the data
        zlib_encoder.write_all(&data).unwrap();
        // Takes the data from the buffer and writes it to disk
        zlib_encoder.flush().unwrap();

        data_write_tx.send(()).await.unwrap();

        let elapsed = compression_bar.elapsed();
        compression_bar = compression_bar.with_elapsed(elapsed);
        compression_bar.inc(1);

    }

    zlib_encoder.finish().unwrap();

    compression_bar.finish();
    handle_data_thread.await.ok()?;

    return Some(());

}

pub async fn handle_data_thread_instructions(config: Config, mut data_write_bar: ProgressBar, mut data_rx: UnboundedReceiver<Vec<u8>>, mut write_flag_rx: Receiver<()>) -> Option<()> {

    // Here we setup out file streaming
    let filename = format!("{}.png", config.filename);
    let path = Path::new(&filename);
    let file = File::create(path).expect(&format!("Failed to create file..? Filename: `{}`", config.filename));
    let ref mut w = BufWriter::new(file);

    // Here we add info to the file
    let mut info = png::Info::with_size(config.size_x, config.size_y);
    info.bit_depth = png::BitDepth::Eight;
    info.color_type = png::ColorType::Rgba;

    // Here we setup the encoder for the image
    let mut encoder = png::Encoder::with_info(w, info.clone()).ok()?;
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::High);
    encoder.set_filter(Filter::NoFilter);

    let config_information = serde_json::to_string_pretty(&config).unwrap_or("FAILED TO SERIALIZE CONFIG! (Serde Error)".to_string());
    debug!("{}", config_information);
    encoder.add_ztxt_chunk("kyros_config".to_string(), config_information).ok()?;

    // Now we write the header to file.
    let mut writer = encoder.write_header().ok()?;

    while !data_rx.is_closed() {

        write_flag_rx.recv().await;

        let mut compressed_data: Vec<u8> = Vec::new();
        while !data_rx.is_empty() {
            // This unwrap should be fine as we are checking if we have values before this happens
            compressed_data.extend_from_slice(&data_rx.recv().await.unwrap());
        }
        writer.write_chunk(IDAT, &compressed_data).unwrap();

        let elapsed = data_write_bar.elapsed();
        data_write_bar = data_write_bar.with_elapsed(elapsed);
        data_write_bar.inc(1);

    }

    while let Some(comp_data) = data_rx.recv().await {
        writer.write_chunk(IDAT, &comp_data).unwrap();
    }

    writer.finish().unwrap();

    data_write_bar.finish();

    return Some(());

}
