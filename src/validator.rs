//! Module for validating large images.
//! Everything in this module is written with very very large images in mind (multiple gigabytes
//! compressed)

use std::{fs::File, ops::BitAnd};

use bitflags::{bitflags, Flags};

use crate::structs::Config;

bitflags! {

    /// A struct that holds the configuration of which checks to do.
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ValidationLevel: u32 {
        const CheckChunks               = 0b00000001;
        const RevalidateCrc             = 0b00000010;
        const CheckHeader               = 0b00000100;
        const ValidateConfigChunk       = 0b00001000;
        const ValidateDataChunkCheckSum = 0b00010000;
    }

}

pub fn check_chunks(validation: ValidationLevel, config: &Config) -> std::io::Result<()> {
    let file = File::open(&config.filename)?;

    let mut decoder = png::StreamingDecoder::new();
    let info = decoder.info();

    decoder.set_ignore_adler32(ValidationLevel::ValidateDataChunkCheckSum.contains(validation));

    return Ok(());

}
