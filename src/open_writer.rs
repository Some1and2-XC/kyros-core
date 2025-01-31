use std::io::Write;
use async_channel::Sender;

/// This struct is for being a type that implements `write` while also being able to access the
/// underlying written data as soon as its written in an async context.
pub struct OpenWriter<T> {
    channel: Sender<T>,
}

impl <T> OpenWriter <T> {

    /// Function for creating a new instance of the writer
    pub fn new(channel: Sender<T>) -> Self {
        return Self {
            channel,
        };
    }

}

impl Write for OpenWriter<Vec<u8>> {

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {

        let buf_lef = buf.len();

        // This is intended to be used with an unbound channel so blocking isn't really a bad
        // thing.
        if let Err(_e) = self.channel.send_blocking(buf.to_vec()) {
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
