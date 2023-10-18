use crate::error::{Error, Result};
use colored::*;
use rdev::Key;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
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
    /// Disable the easter eggs.
    #[serde(rename = "no_surprises", default)]
    pub disable_easter_eggs: bool,
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

    /// Returns a preset by its name if it exists.
    pub fn select_preset(&self, name: &str) -> Result<SoundPreset> {
        if !self.disable_easter_eggs && fastrand::usize(0..1000) == 42 || name == "ak47" {
            return Ok(SoundPreset {
                name: String::new(),
                key_config: vec![
                    KeyConfig {
                        event: KeyEvent::KeyPress,
                        keys: Regex::new("Return")?,
                        files: vec![
                            AudioFile {
                                path: String::from("mbox10.mp3"),
                                volume: None,
                            },
                            AudioFile {
                                path: String::from("mbox11.mp3"),
                                volume: None,
                            },
                        ],
                        strategy: Some(PlaybackStrategy::Random),
                        variation: None,
                    },
                    KeyConfig {
                        event: KeyEvent::KeyPress,
                        keys: Regex::new(".*")?,
                        files: vec![AudioFile {
                            path: String::from("mbox9.mp3"),
                            volume: None,
                        }],
                        strategy: None,
                        variation: Some(SoundVariation {
                            volume: Some((0.1, 0.1)),
                            tempo: Some((0.075, 0.075)),
                        }),
                    },
                ],
                disabled_keys: None,
                variation: None,
            });
        }
        self.sound_presets
            .clone()
            .into_iter()
            .find(|v| v.name == name)
            .ok_or_else(|| Error::PresetNotFound(name.to_string()))
    }
}

/// Sound preset.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoundPreset {
    /// Name of the preset.
    pub name: String,
    /// Key configuration.
    pub key_config: Vec<KeyConfig>,
    /// List of disabled keys.
    pub disabled_keys: Option<Vec<Key>>,
    /// Configure sound variations.
    pub variation: Option<SoundVariation>,
}

impl fmt::Display for SoundPreset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[{}]", self.name.white().bold())?;
        let mut table = format!(
            " {:<20}  {:<20}  {:<20}\n",
            "Event".bold(),
            "Keys".bold(),
            "File".bold()
        );
        table.push_str(&format!(
            " {:<20}  {:<20}  {:<20}\n",
            "-----", "----", "----"
        ));
        for key_config in &self.key_config {
            let event_str = match key_config.event {
                KeyEvent::KeyPress => "Key Press",
                KeyEvent::KeyRelease => "Key Release",
            };
            let keys_str = key_config.keys.as_str();
            let file_str = &key_config
                .files
                .iter()
                .map(|v| v.path.clone())
                .collect::<Vec<String>>()
                .join(",");
            table.push_str(&format!(
                " {:<20}  {:<20}  {:<20}\n",
                event_str,
                keys_str,
                file_str.italic()
            ));
        }
        write!(f, "{}", table)
    }
}

/// Key configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyConfig {
    /// Event.
    pub event: KeyEvent,
    /// Keys regex.
    #[serde(with = "serde_regex")]
    pub keys: Regex,
    /// MP3 files.
    pub files: Vec<AudioFile>,
    /// Playback strategy.
    pub strategy: Option<PlaybackStrategy>,
    /// Sound variations. Overrides the preset sound variations.
    pub variation: Option<SoundVariation>,
}

/// Key event type.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum KeyEvent {
    /// Key press.
    #[serde(rename = "press")]
    KeyPress,
    /// Key release.
    #[serde(rename = "release")]
    KeyRelease,
}

/// Audio file configuration.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AudioFile {
    /// Path of the file.
    pub path: String,
    /// Volume.
    pub volume: Option<f32>,
}

/// Playback strategy.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlaybackStrategy {
    /// Pick random.
    Random,
    /// Play sequentially.
    Sequential,
}

/// Sound variation configuration.
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct SoundVariation {
    /// Volume +/- in percent.
    pub volume: Option<(f32, f32)>,
    /// Tempo +/- in percent.
    pub tempo: Option<(f32, f32)>,
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
