use crate::{
    config::{AudioFile, KeyConfig, KeyEvent, PlaybackStrategy, SoundPreset, SoundVariation},
    embed::EmbeddedSound,
    error::{Error, Result},
};
use rdev::{Event, EventType};
use rodio::{cpal::traits::HostTrait, Decoder, DeviceTrait, OutputStream, Sink};
use std::{fs::File, io::BufReader};

/// Application state controller.
pub struct App {
    /// Sound preset.
    preset: SoundPreset,
    /// Output stream.
    _stream: OutputStream,
    /// Sink for key presses.
    key_press_sink: Sink,
    /// Sink for key releases.
    key_release_sink: Sink,
    /// Whether if the key is released.
    key_released: bool,
    /// Index of the file to play.
    file_index: usize,
    /// Sound variations.
    variation: Option<SoundVariation>,
}

impl App {
    /// Initializes a new instance.
    pub fn init(
        preset: SoundPreset,
        variation: Option<SoundVariation>,
        device: Option<String>,
    ) -> Result<Self> {
        let device = match device {
            Some(ref device) => rodio::cpal::default_host()
                .output_devices()?
                .find(|v| v.name().unwrap_or_default().to_lowercase() == *device),
            None => rodio::cpal::default_host().default_output_device(),
        }
        .ok_or_else(|| {
            Error::DeviceNotFound(
                device
                    .unwrap_or_else(|| String::from("default"))
                    .to_string(),
            )
        })?;
        tracing::debug!("Using output device: {}", device.name()?);
        let (stream, handle) = OutputStream::try_from_device(&device)?;
        let key_press_sink = Sink::try_new(&handle)?;
        let key_release_sink = Sink::try_new(&handle)?;
        Ok(Self {
            preset,
            _stream: stream,
            key_press_sink,
            key_release_sink,
            key_released: true,
            file_index: 0,
            variation,
        })
    }

    /// Handle the key events.
    pub fn handle_key_event(&mut self, event: Event) -> Result<()> {
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
                    EventType::KeyPress(_) => KeyEvent::KeyPress,
                    EventType::KeyRelease(_) => KeyEvent::KeyRelease,
                    _ => unreachable!(),
                };
                let key_config = self
                    .preset
                    .key_config
                    .clone()
                    .into_iter()
                    .find(|v| v.event == event_type && v.keys.is_match(&format!("{:?}", key)));
                tracing::debug!("Key config: {:?}", key_config);
                if event_type == KeyEvent::KeyPress {
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
    fn handle_key_press(&mut self, key_config: &Option<KeyConfig>) -> Result<()> {
        if self.key_released {
            if let Some(key_config) = key_config {
                let file = self.pick_sound_file(key_config)?;
                self.play_sound(&file, self.get_variation(key_config), &self.key_press_sink)?;
            }
        }
        self.key_released = false;
        Ok(())
    }

    /// Handle the key release event.
    fn handle_key_release(&mut self, key_config: &Option<KeyConfig>) -> Result<()> {
        if let Some(key_config) = key_config {
            let file = self.pick_sound_file(key_config)?;
            self.play_sound(
                &file,
                self.get_variation(key_config),
                &self.key_release_sink,
            )?;
        }
        self.key_released = true;
        Ok(())
    }

    /// Returns the sound file to play.
    fn pick_sound_file(&mut self, key_config: &KeyConfig) -> Result<AudioFile> {
        match key_config.strategy {
            Some(PlaybackStrategy::Random) => {
                Ok(key_config.files[fastrand::usize(..key_config.files.len())].clone())
            }
            Some(PlaybackStrategy::Sequential) => {
                if self.file_index >= key_config.files.len() {
                    self.file_index = 0;
                }
                let file = key_config.files[self.file_index].clone();
                self.file_index += 1;
                Ok(file)
            }
            None => key_config
                .files
                .first()
                .cloned()
                .ok_or_else(|| Error::NoAudioFiles),
        }
    }

    /// Play the sound from embedded/file for the given sink.
    fn play_sound(
        &self,
        file: &AudioFile,
        variation: Option<SoundVariation>,
        sink: &Sink,
    ) -> Result<()> {
        tracing::debug!("Playing: {:?}", file);

        let volume = file.volume.unwrap_or(1.0)
            * self.generate_variation_factor(variation.as_ref().and_then(|v| v.volume));
        let tempo = self.generate_variation_factor(variation.as_ref().and_then(|v| v.tempo));
        tracing::debug!("Volume: {}, Tempo: {}", volume, tempo);

        sink.stop();
        sink.set_volume(volume);
        sink.set_speed(tempo);

        if let Some(embed_data) = EmbeddedSound::get_sound(&file.path) {
            let sound = BufReader::new(Box::new(embed_data));
            sink.append(Decoder::new(sound)?);
        } else {
            let sound = BufReader::new(Box::new(File::open(&file.path)?));
            sink.append(Decoder::new(sound)?);
        };
        Ok(())
    }

    /// Get variation for the given key.
    fn get_variation(&self, key: &KeyConfig) -> Option<SoundVariation> {
        self.variation
            .clone()
            .or(key.variation.clone())
            .or(self.preset.variation.clone())
    }

    /// Generate variation factor
    fn generate_variation_factor(&self, variation: Option<(f32, f32)>) -> f32 {
        let Some((plus, minus)) = variation else {
            return 1.0;
        };

        let variation = fastrand::f32() * (plus + minus) - minus;
        1.0 + variation
    }
}

#[cfg(feature = "audio-tests")]
#[cfg(test)]
mod tests {
    use super::*;
    use rdev::Key;
    use regex::Regex;
    use std::{
        thread,
        time::{Duration, SystemTime},
    };

    #[test]
    fn test_handle_key_event() -> Result<()> {
        let ding_audio = AudioFile {
            path: String::from("ding.mp3"),
            volume: None,
        };
        let quack_audio = AudioFile {
            path: String::from("quack1.mp3"),
            volume: None,
        };
        let key_config = vec![
            KeyConfig {
                event: KeyEvent::KeyPress,
                keys: Regex::new("Space")?,
                files: vec![ding_audio.clone()],
                strategy: None,
            },
            KeyConfig {
                event: KeyEvent::KeyRelease,
                keys: Regex::new(".*")?,
                files: vec![quack_audio.clone()],
                strategy: None,
            },
        ];
        let sound_preset = SoundPreset {
            name: String::new(),
            key_config: key_config.clone(),
            disabled_keys: None,
        };
        let mut app = App::init(sound_preset)?;
        assert_eq!(ding_audio, app.pick_sound_file(&key_config[0])?);
        assert_eq!(quack_audio, app.pick_sound_file(&key_config[1])?);

        app.handle_key_event(Event {
            time: SystemTime::now(),
            name: None,
            event_type: EventType::KeyPress(Key::Space),
        })?;
        assert_eq!(1, app.key_press_sink.len());
        app.handle_key_event(Event {
            time: SystemTime::now(),
            name: None,
            event_type: EventType::KeyRelease(Key::KeyQ),
        })?;
        assert_eq!(1, app.key_press_sink.len());

        thread::sleep(Duration::from_millis(2000));
        assert_eq!(0, app.key_press_sink.len());

        Ok(())
    }
}
