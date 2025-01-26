use std::io::Write;
use tokio::sync::mpsc::{Sender, UnboundedSender};

/// This struct is for being a type that implements `write` while also being able to access the
/// underlying written data as soon as its written in an async context.
pub struct OpenWriter<T> {
    channel: UnboundedSender<T>,
}

impl <T> OpenWriter <T> {

    /// Function for creating a new instance of the writer
    pub fn new(channel: UnboundedSender<T>) -> Self {
        return Self {
            channel,
        };
    }

}


impl Write for OpenWriter<Vec<u8>> {

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {

        let buf_lef = buf.len();

        if let Err(_e) = self.channel.send(buf.to_vec()) {
            // I'm unsure if this is what is the intended use of the BrokenPipe error type however
            // it seems like a good fit for this usecase.
            return Err(std::io::ErrorKind::BrokenPipe.into());
        }

        return Ok(buf_lef);

    }

    fn flush(&mut self) -> std::io::Result<()> {
        return Ok(());
    }

}
