use crate::{
    config::{KeyConfig, KeyEventConfig, SoundPreset},
    embed::EmbeddedSound,
    error::Result,
};
use rdev::{Event, EventType};
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};

/// Application state controller.
pub struct App {
    preset: SoundPreset,
    _stream: OutputStream,
    key_press_sink: Sink,
    key_release_sink: Sink,
    key_released: bool,
}

impl App {
    /// Initializes a new instance.
    pub fn init(preset: SoundPreset) -> Result<Self> {
        let (stream, handle) = OutputStream::try_default()?;
        let key_press_sink = Sink::try_new(&handle)?;
        let key_release_sink = Sink::try_new(&handle)?;
        Ok(Self {
            preset,
            _stream: stream,
            key_press_sink,
            key_release_sink,
            key_released: true,
        })
    }

    /// Handle the key events.
    pub fn handle_event(&mut self, event: Event) -> Result<()> {
        match event.event_type {
            EventType::KeyPress(key) | EventType::KeyRelease(key) => {
                tracing::debug!("Event: {:?}", event);
                if self
                    .preset
                    .disabled_keys
                    .as_ref()
                    .cloned()
                    .unwrap_or_default()
                    .contains(&key)
                {
                    tracing::debug!("Skipping: {:?}", key);
                    return Ok(());
                }
                let event_type = match event.event_type {
                    EventType::KeyPress(_) => KeyEventConfig::KeyPress,
                    EventType::KeyRelease(_) => KeyEventConfig::KeyRelease,
                    _ => unreachable!(),
                };
                let key_config = self
                    .preset
                    .key_config
                    .clone()
                    .into_iter()
                    .find(|v| v.event == event_type && v.keys.is_match(&format!("{:?}", key)));
                tracing::debug!("Key config: {:?}", key_config);
                if event_type == KeyEventConfig::KeyPress {
                    self.handle_key_press(&key_config)?;
                } else {
                    self.handle_key_release(&key_config)?;
                }
            }
            _ => {}
        };
        Ok(())
    }

    /// Handle the key press event.
    pub fn handle_key_press(&mut self, key_config: &Option<KeyConfig>) -> Result<()> {
        if self.key_released {
            if let Some(key_config) = key_config {
                self.play_sound(key_config, &self.key_press_sink)?;
            }
        }
        self.key_released = false;
        Ok(())
    }

    /// Handle the key release event.
    pub fn handle_key_release(&mut self, key_config: &Option<KeyConfig>) -> Result<()> {
        if let Some(key_config) = key_config {
            self.play_sound(key_config, &self.key_release_sink)?;
        }
        self.key_released = true;
        Ok(())
    }

    /// Play the sound from embedded/file for the given sink.
    fn play_sound(&self, key_config: &KeyConfig, sink: &Sink) -> Result<()> {
        if key_config.embed.unwrap_or(false) {
            let sound = BufReader::new(Box::new(EmbeddedSound::get_sound(&key_config.file)?));
            sink.stop();
            sink.set_volume(key_config.volume.unwrap_or(1.0));
            sink.append(Decoder::new(sound)?);
        } else {
            let sound = BufReader::new(Box::new(File::open(&key_config.file)?));
            sink.stop();
            sink.set_volume(key_config.volume.unwrap_or(1.0));
            sink.append(Decoder::new(sound)?);
        };
        Ok(())
    }
}
