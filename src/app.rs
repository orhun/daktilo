use std::io::BufReader;

use crate::{
    embed::{Sound, Sounds},
    error::Result,
};
use rodio::{OutputStream, Sink};

/// Application state controller.
pub struct App {
    _stream: OutputStream,
    key_press_sink: Sink,
    key_release_sink: Sink,
    key_released: bool,
}

impl App {
    /// Initializes a new instance.
    pub fn init() -> Result<Self> {
        let (stream, handle) = OutputStream::try_default()?;
        let key_press_sink = Sink::try_new(&handle)?;
        let key_release_sink = Sink::try_new(&handle)?;
        Ok(Self {
            _stream: stream,
            key_press_sink,
            key_release_sink,
            key_released: true,
        })
    }

    /// Handle the key press event.
    pub fn handle_key_press(&mut self) -> Result<()> {
        if self.key_released {
            let sound = Sounds::get_sound(Sound::Keydown)?;
            self.key_press_sink.stop();
            self.key_press_sink
                .append(rodio::Decoder::new(BufReader::new(sound))?);
        }
        self.key_released = false;
        Ok(())
    }

    /// Handle the key release event.
    pub fn handle_key_release(&mut self) -> Result<()> {
        let sound = Sounds::get_sound(Sound::Keyup)?;
        self.key_release_sink.stop();
        self.key_release_sink
            .append(rodio::Decoder::new(BufReader::new(sound))?);
        self.key_released = true;
        Ok(())
    }
}
