use crate::{
    config::{Config, DEFAULT_CONFIG},
    error::{Error, Result},
};
use rust_embed::RustEmbed;
use std::io::Cursor;
use std::str;

/// Embedded sound assets.
#[derive(RustEmbed)]
#[folder = "../../sounds/"]
pub struct EmbeddedSound;

impl EmbeddedSound {
    /// Returns the bytes of the sound.
    pub fn get_sound(name: &str) -> Option<Cursor<Vec<u8>>> {
        Self::get(name).map(|v| Cursor::new(v.data.to_vec()))
    }
}

/// Configuration file embedder/extractor.
///
/// Embeds `config/`[`DEFAULT_CONFIG`] into the binary.
#[derive(Debug, RustEmbed)]
#[folder = "../../config/"]
pub struct EmbeddedConfig;

impl EmbeddedConfig {
    /// Extracts the embedded content.
    pub fn get_config() -> Result<String> {
        match Self::get(DEFAULT_CONFIG) {
            Some(v) => Ok(str::from_utf8(&v.data)?.to_string()),
            None => Err(Error::Embedded(format!(
                "embedded config {} not found",
                DEFAULT_CONFIG,
            ))),
        }
    }

    /// Parses the extracted content into [`Config`].
    ///
    /// [`Config`]: Config
    pub fn parse() -> Result<Config> {
        Ok(toml::from_str(&Self::get_config()?)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_embed_config() {
        assert!(EmbeddedConfig::parse().is_ok());
    }
}
