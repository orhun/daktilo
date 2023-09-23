/// Error implementation.
pub mod error;

/// Logger.
pub mod logger;

/// File embedder.
mod embed;

/// Command-line arguments.
pub mod args;

use embed::{Sound, Sounds};
use error::Result;
use rdev::{listen, EventType};
use rodio::{OutputStream, Sink};
use std::{io::BufReader, thread};

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

    // Create the sink for audio playback.
    let (_stream, handle) = OutputStream::try_default()?;
    let key_press_sink = Sink::try_new(&handle)?;
    let key_release_sink = Sink::try_new(&handle)?;

    // Handle events loop.
    let mut key_released = true;
    loop {
        // Handle events - i.e. add data to the mixer controller.
        if let Some(event) = receiver.recv().await {
            tracing::debug!("{:?}", event);
            match event.event_type {
                EventType::KeyPress(_) => {
                    if key_released {
                        let sound = Sounds::get_sound(Sound::Keydown)?;
                        key_press_sink.stop();
                        key_press_sink.append(rodio::Decoder::new(BufReader::new(sound))?);
                    }
                    key_released = false;
                }
                EventType::KeyRelease(_) => {
                    let sound = Sounds::get_sound(Sound::Keyup)?;
                    key_release_sink.stop();
                    key_release_sink.append(rodio::Decoder::new(BufReader::new(sound))?);
                    key_released = true;
                }
                _ => {}
            };
        }
    }
}
