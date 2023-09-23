/// Error implementation.
mod error;

/// Logger.
mod logger;

use rdev::{listen, EventType};
use rodio::{OutputStream, Sink};
use std::fs::File;
use std::{io::BufReader, thread};

use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init(None);
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
                    let file = File::open("sounds/keydown.mp3").unwrap();
                    controller.add(rodio::Decoder::new(BufReader::new(file)).unwrap());
                }
                EventType::KeyRelease(_) => {
                    let file = File::open("sounds/keyup.mp3").unwrap();
                    controller.add(rodio::Decoder::new(BufReader::new(file)).unwrap());
                }
                _ => {}
            };
        }
        sink.stop();
        sink.append(mixer);
    }
}
