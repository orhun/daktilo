use crate::error::{Error, Result};
use rust_embed::RustEmbed;
use std::{fmt, io::Cursor};

/// Representation of a typewriter sound.
#[derive(Debug)]
pub enum Sound {
    Ding,
    Keydown,
    Keystroke,
    Keyup,
    Newline,
}

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Sound {
    /// Returns the file name of the sound.
    fn as_file_name(&self) -> String {
        format!("{}.mp3", self.to_string())
    }
}

/// Embedded sound assets.
#[derive(RustEmbed)]
#[folder = "sounds"]
pub struct Sounds;

impl Sounds {
    /// Returns the bytes of the sound.
    pub fn get_sound(sound: Sound) -> Result<Cursor<Vec<u8>>> {
        Self::get(&sound.as_file_name())
            .map(|v| Cursor::new(v.data.to_vec()))
            .ok_or_else(|| Error::AssetNotFound(sound.to_string()))
    }
}
