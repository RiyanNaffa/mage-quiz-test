use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

fn process(sink: &Sink){
    let file = BufReader::new(File::open("songs/frieren_2.mp3").unwrap());
    let source = Decoder::new(file).unwrap().repeat_infinite();
    sink.append(source);
}

pub fn play(){
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        loop {
            process(&sink);
            thread::sleep(Duration::from_millis(100));
        }
    });
}