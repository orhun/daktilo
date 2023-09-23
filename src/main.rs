use rdev::{listen, Event, EventType};
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::{io::BufReader, thread, time::Duration};

#[tokio::main]
async fn main() {
    let (schan, mut rchan) = tokio::sync::mpsc::unbounded_channel();
    let _listener = thread::spawn(move || {
        listen(move |event| {
            schan
                .send(event)
                .unwrap_or_else(|e| println!("Could not send event {:?}", e));
        })
        .expect("Could not listen");
    });
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();
    loop {
        let (controller, mixer) = rodio::dynamic_mixer::mixer::<i16>(2, 44_100);
        let event = rchan.recv().await;
        let event = event.unwrap();
        println!("{:?}", event);
        if let EventType::KeyPress(_) = event.event_type {
            let file = File::open("sounds/keydown.mp3").unwrap();
            controller.add(rodio::Decoder::new(BufReader::new(file)).unwrap());
        }
        if let EventType::KeyRelease(_) = event.event_type {
            let file = File::open("sounds/keyup.mp3").unwrap();
            controller.add(rodio::Decoder::new(BufReader::new(file)).unwrap());
        }
        sink.stop();
        sink.append(mixer);
    }
}
