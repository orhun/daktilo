use clap::Parser;
use std::{fs, process};
use tracing::Level;

use daktilo::args::Args;
use daktilo::config::{Config, DEFAULT_CONFIG};
use daktilo::embed::EmbeddedConfig;
use daktilo::error::{Error, Result};
use daktilo::logger;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments.
    let args = Args::parse();

    // Initialize the logger.
    logger::init(args.verbose.then_some(Level::DEBUG))?;
    tracing::info!("Starting...");

    // Parse the config file.
    if args.init {
        tracing::info!("Saving the configuration file to {:?}", DEFAULT_CONFIG);
        fs::write(DEFAULT_CONFIG, EmbeddedConfig::get_config()?)?;
        return Ok(());
    }
    let config_path = args.config.or(Config::get_default_location());
    let config = if config_path.as_ref().is_some_and(|v| v.exists()) {
        // unwrap is checked above.
        Config::parse(&config_path.unwrap())?
    } else {
        tracing::warn!("Using the default configuration (run with `--init` to save it to a file).");
        EmbeddedConfig::parse()?
    };
    tracing::debug!("{:#?}", config);

    // Start the typewriter.
    let preset_name = args.preset.unwrap_or_else(|| String::from("default"));
    let preset = config
        .sound_presets
        .into_iter()
        .find(|v| v.name == preset_name)
        .ok_or_else(|| Error::PresetNotFound(preset_name))?;
    match daktilo::run(preset).await {
        Ok(_) => process::exit(0),
        Err(e) => {
            tracing::error!("error occurred: {e}");
            process::exit(1)
        }
    }
}
