use std::path::PathBuf;

use clap::Parser;

use crate::config::SoundVariation;

/// Typewriter ASCII banner.
pub const BANNER: &str = r#"
      .-------.
     _|~~ ~~  |_
   =(_|_______|_)=
     |:::::::::|
     |:::::::[]|
     |o=======.|
     `"""""""""`
"#;

/// Argument parser powered by [`clap`].
#[derive(Debug, Default, Parser)]
#[clap(
    version,
    author = clap::crate_authors!("\n"),
    about,
    rename_all_env = "screaming-snake",
    before_help = BANNER,
    help_template = "\
{before-help}-=[ {name} {version} ]=-\n
{about-with-newline}Written by {author-with-newline}
{usage-heading}
  {usage}

{all-args}{after-help}
",
)]
pub struct Args {
    /// Enables verbose logging.
    #[arg(short, long, env)]
    pub verbose: bool,
    /// Sets the name of the sound preset to use.
    #[arg(short, long, env)]
    pub preset: Option<String>,
    /// Lists the available presets.
    #[arg(short, long, aliases = vec!["ls", "list"])]
    pub list_presets: bool,
    /// Lists the available output devices.
    #[arg(long)]
    pub list_devices: bool,
    /// Sets the device for playback.
    #[arg(short, long, env = "DAKTILO_DEVICE", value_name = "DEVICE")]
    pub device: Option<String>,
    /// Sets the configuration file.
    #[arg(short, long, env = "DAKTILO_CONFIG", value_name = "PATH")]
    pub config: Option<PathBuf>,
    /// Writes the default configuration file.
    #[arg(short, long)]
    pub init: bool,
    /// Variate pitch/volume/tempo.
    #[command(flatten)]
    pub sound_variation_args: Option<SoundVariationArgs>,
}

/// Variate pitch/volume/tempo.
#[derive(clap::Args, Default, Debug)]
pub struct SoundVariationArgs {
    /// Variate pitch +/- in percent.
    /// Overrides preset configuration.
    #[arg(long, env = "DAKTILO_PITCH", value_name = "PERCENT")]
    pub variate_pitch: Option<f32>,
    /// Variate volume +/- in percent.
    /// Overrides preset configuration.
    #[arg(long, env = "DAKTILO_VOLUME", value_name = "PERCENT")]
    pub variate_volume: Option<f32>,
    /// Variate tempo +/- in percent.
    /// Overrides preset configuration.
    #[arg(long, env = "DAKTILO_TEMPO", value_name = "PERCENT")]
    pub variate_tempo: Option<f32>,
}

impl Into<SoundVariation> for SoundVariationArgs {
    fn into(self) -> SoundVariation {
        SoundVariation {
            pitch: self.variate_pitch.map(|pitch| (pitch, pitch)),
            volume: self.variate_volume.map(|volume| (volume, volume)),
            tempo: self.variate_tempo.map(|tempo| (tempo, tempo)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    #[test]
    fn test_args() {
        Args::command().debug_assert();
    }
}
