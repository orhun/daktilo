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
    let sink = Sink::try_new(&handle)?;

    // Handle events loop.
    let mut key_released = true;
    loop {
        // Create a 2-channel mixer.
        let (controller, mixer) = rodio::dynamic_mixer::mixer::<i16>(2, 44_100);

        // Handle events - i.e. add data to the mixer controller.
        if let Some(event) = receiver.recv().await {
            tracing::debug!("{:?}", event);
            match event.event_type {
                EventType::KeyPress(_) => {
                    if key_released {
                        let sound = Sounds::get_sound(Sound::Keydown)?;
                        controller.add(rodio::Decoder::new(BufReader::new(sound))?);
                        sink.stop();
                        sink.append(mixer);
                    }
                    key_released = false;
                }
                EventType::KeyRelease(_) => {
                    let sound = Sounds::get_sound(Sound::Keyup)?;
                    controller.add(rodio::Decoder::new(BufReader::new(sound))?);
                    sink.stop();
                    sink.append(mixer);
                    key_released = true;
                }
                _ => {}
            };
        }
    }
}
