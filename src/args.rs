use std::path::PathBuf;

use clap::Parser;

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
