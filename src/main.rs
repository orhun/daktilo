/// Error implementation.
mod error;

/// Logger.
mod logger;

/// File embedder.
mod embed;

/// Command-line arguments.
mod args;

use clap::Parser;
use rdev::{listen, EventType};
use rodio::{OutputStream, Sink};
use std::{io::BufReader, thread};
use tracing::Level;

use crate::args::Args;
use crate::embed::{Sound, Sounds};
use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    logger::init(args.verbose.then_some(Level::DEBUG))?;
    tracing::info!("starting");
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
    thread::spawn(move || {
        listen(move |event| {
            sender
                .send(event)
                .unwrap_or_else(|e| tracing::error!("could not send event {:?}", e));
        })
        .expect("could not listen events");
    });
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;
    loop {
        let (controller, mixer) = rodio::dynamic_mixer::mixer::<i16>(2, 44_100);
        if let Some(event) = receiver.recv().await {
            tracing::debug!("{:?}", event);
            match event.event_type {
                EventType::KeyPress(_) => {
                    let sound = Sounds::get_sound(Sound::Keydown)?;
                    controller.add(rodio::Decoder::new(BufReader::new(sound)).unwrap());
                }
                EventType::KeyRelease(_) => {
                    let sound = Sounds::get_sound(Sound::Keyup)?;
                    controller.add(rodio::Decoder::new(BufReader::new(sound)).unwrap());
                }
                _ => {}
            };
        }
        sink.stop();
        sink.append(mixer);
    }
}
