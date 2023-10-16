//! Turn your keyboard into a typewriter! 📇

#![warn(missing_docs)]

/// Error implementation.
pub mod error;

/// Logger.
pub mod logger;

/// File embedder.
pub mod embed;

/// Command-line arguments.
pub mod args;

/// Application state.
pub mod app;

/// Configuration file.
pub mod config;

use app::App;
use config::{SoundPreset, SoundVariation};
use error::Result;
use rdev::listen;
use std::thread;

/// Starts the typewriter.
pub async fn run(
    sound_presets: Vec<SoundPreset>,
    variation: Option<SoundVariation>,
    device: Option<String>,
) -> Result<()> {
    // Create a listener for the keyboard events.
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
    thread::spawn(move || {
        listen(move |event| {
            sender
                .send(event)
                .unwrap_or_else(|e| tracing::error!("could not send event {:?}", e));
        })
        .expect("could not listen events");
    });

    // Create the application state.
    tracing::debug!("{:#?}", sound_presets);
    let mut apps = sound_presets
        .into_iter()
        .map(|sound_preset| App::init(sound_preset, variation.clone(), device.clone()))
        .collect::<Result<Vec<_>>>()?;

    // Handle events.
    loop {
        if let Some(event) = receiver.recv().await {
            for app in apps.iter_mut() {
                app.handle_key_event(event.clone()).unwrap();
            }
        }
    }
}
