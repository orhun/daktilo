use std::path::PathBuf;

use clap::Parser;

use daktilo_lib::config::SoundVariation;

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
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// Sets the name of the sound preset to use.
    ///
    /// Can be specified multiple times to play multiple presets at once.
    #[arg(short, long, env, num_args(0..))]
    pub preset: Vec<String>,
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
    /// Disables the easter eggs.
    #[arg(long, hide = true)]
    pub no_surprises: bool,
    /// Variate pitch/volume/tempo.
    #[command(flatten)]
    pub sound_variation_args: Option<SoundVariationArgs>,
}

/// Variate pitch/volume/tempo.
#[derive(clap::Args, Default, Debug)]
pub struct SoundVariationArgs {
    /// Variate volume +/- in percent.
    #[arg(
        long,
        env = "DAKTILO_VOLUME",
        value_name = "PERCENT_UP[,PERCENT_DOWN]",
        value_delimiter = ',',
        num_args(1..2)
    )]
    pub variate_volume: Option<Vec<f32>>,
    /// Variate tempo +/- in percent.
    #[arg(
        long,
        env = "DAKTILO_TEMPO",
        value_name = "PERCENT_UP[,PERCENT_DOWN]",
        value_delimiter = ',',
        num_args(1..2)
    )]
    pub variate_tempo: Option<Vec<f32>>,
}

impl From<SoundVariationArgs> for SoundVariation {
    fn from(args: SoundVariationArgs) -> Self {
        Self {
            volume: args.variate_volume.map(|v| {
                (
                    v.first().cloned().unwrap_or(1.0),
                    v.get(1).or(v.first()).cloned().unwrap_or(1.0),
                )
            }),
            tempo: args.variate_tempo.map(|t| {
                (
                    t.first().cloned().unwrap_or(1.0),
                    t.get(1).or(t.first()).cloned().unwrap_or(1.0),
                )
            }),
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
