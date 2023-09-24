use crate::error::Result;
use rdev::Key;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;

/// Default configuration file.
pub const DEFAULT_CONFIG: &str = concat!(env!("CARGO_PKG_NAME"), ".toml");

/// Configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Sound presets.
    #[serde(rename = "sound_preset")]
    pub sound_presets: Vec<SoundPreset>,
}

impl Config {
    /// Checks the possible locations for the configuration file.
    ///
    /// - `<config_dir>/<project>.toml`
    /// - `<config_dir>/<project>/<project>.toml`
    /// - `<config_dir>/<project>/config`
    ///
    /// Returns the path if the configuration file is found.
    pub fn get_default_location() -> Option<PathBuf> {
        if let Some(config_dirs) = dirs::config_dir().map(|config_dir| {
            vec![
                config_dir.join(DEFAULT_CONFIG),
                config_dir.join(env!("CARGO_PKG_NAME")).join(DEFAULT_CONFIG),
                config_dir.join(env!("CARGO_PKG_NAME")).join("config"),
            ]
        }) {
            for config_dir in config_dirs {
                if config_dir.exists() {
                    return Some(config_dir);
                }
            }
        }
        None
    }

    /// Parses the configuration file.
    pub fn parse(file: &Path) -> Result<Config> {
        let contents = fs::read_to_string(file)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

/// Sound preset.
#[derive(Debug, Serialize, Deserialize)]
pub struct SoundPreset {
    /// Name of the preset.
    pub name: String,
    /// Key configuration.
    pub key_config: Vec<KeyConfig>,
    /// List of disabled keys.
    pub disabled_keys: Option<Vec<Key>>,
}

/// Key configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyConfig {
    /// Event.
    pub event: KeyEventConfig,
    /// Keys regex.
    #[serde(with = "serde_regex")]
    pub keys: Regex,
    /// MP3 file.
    pub file: String,
    /// Volume.
    pub volume: Option<f32>,
    /// Whether if the file is embedded.
    pub embed: Option<bool>,
}

/// Key configuration.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum KeyEventConfig {
    /// Key press.
    KeyPress,
    /// Key release.
    KeyRelease,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_config() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("config")
            .join(format!("{}.toml", env!("CARGO_PKG_NAME")));
        if let Some(global_path) = Config::get_default_location() {
            path = global_path;
        }
        Config::parse(&path)?;
        Ok(())
    }
}
