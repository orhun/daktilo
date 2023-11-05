use clap::Parser;
use colored::Colorize;
use daktilo::args::Args;
use daktilo_lib::config::{Config, KeyEvent, SoundPreset, DEFAULT_CONFIG};
use daktilo_lib::embed::EmbeddedConfig;
use daktilo_lib::error::Result;
use daktilo_lib::logger;
use std::{fs, process};
use tracing::Level;

/// Prints the available presets to stdout as a table.
fn list_presets(presets: Vec<SoundPreset>) {
    tracing::info!("Available presets:");
    presets.iter().for_each(|preset| {
        println!("[{}]", preset.name.white().bold());
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
        for key_config in &preset.key_config {
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
        println!("{}", table)
    });
}

/// Entry-point of the application.
#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments.
    let args = Args::parse();

    // Initialize the logger.
    logger::init(
        Some(match args.verbose {
            0 => Level::INFO,
            1 => Level::DEBUG,
            _ => Level::TRACE,
        }),
        vec![env!("CARGO_PKG_NAME").to_string()],
    )?;
    tracing::info!("Starting...");

    // Parse the config file.
    if args.init {
        tracing::info!("Saving the configuration file to {:?}", DEFAULT_CONFIG);
        fs::write(DEFAULT_CONFIG, EmbeddedConfig::get_config()?)?;
        return Ok(());
    }
    let config_path = args.config.or(Config::get_default_location());
    let mut config = if config_path.as_ref().is_some_and(|v| v.exists()) {
        // unwrap is checked above.
        Config::parse(&config_path.unwrap())?
    } else {
        tracing::warn!("Using the default configuration (run with `--init` to save it to a file).");
        EmbeddedConfig::parse()?
    };

    if args.no_surprises {
        tracing::debug!("I bet you're fun at parties.");
        config.disable_easter_eggs = true;
    }

    tracing::debug!("{:#?}", config);

    // Start the typewriter.
    if args.list_presets {
        tracing::info!("Available presets:");
        list_presets(config.sound_presets);
        return Ok(());
    } else if args.list_devices {
        tracing::info!("Available devices:");
        daktilo_lib::audio::get_devices()?
            .iter()
            .try_for_each::<_, Result<()>>(|v| {
                println!("• {}", v.0.white().bold());
                Ok(())
            })?;
        return Ok(());
    }

    let presets = if args.preset.is_empty() {
        tracing::warn!("No preset specified, using the default preset.");
        vec![String::from("default")]
    } else {
        args.preset
    }
    .iter()
    .map(|name| config.select_preset(name))
    .collect::<Result<Vec<_>>>()?;

    match daktilo_lib::run(
        presets,
        args.sound_variation_args.map(|v| v.into()),
        args.device,
    )
    .await
    {
        Ok(_) => process::exit(0),
        Err(e) => {
            tracing::error!("error occurred: {e}");
            process::exit(1)
        }
    }
}
