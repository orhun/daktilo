/// Error implementation.
pub mod error;

/// Logger.
pub mod logger;

/// File embedder.
mod embed;

/// Command-line arguments.
pub mod args;

/// Application state.
pub mod app;

use app::App;
use error::Result;
use rdev::{listen, EventType};
use std::thread;

/// Starts the typewriter.
pub async fn run() -> Result<()> {
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
    let mut app = App::init()?;

    // Handle events loop.
    loop {
        if let Some(event) = receiver.recv().await {
            tracing::debug!("{:?}", event);
            match event.event_type {
                EventType::KeyPress(_) => {
                    app.handle_key_press()?;
                }
                EventType::KeyRelease(_) => {
                    app.handle_key_release()?;
                }
                _ => {}
            };
        }
    }
}
