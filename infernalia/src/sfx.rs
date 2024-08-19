use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

pub struct SoundManager {
    _stream: OutputStream,
    stream_handle: Arc<rodio::OutputStreamHandle>,
    footstep_sink: Sink,
    ambient_sink: Sink, // Sink to handle the ambient sound
}

impl SoundManager {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let footstep_sink = Sink::try_new(&stream_handle).unwrap();
        let ambient_sink = Sink::try_new(&stream_handle).unwrap();

        SoundManager {
            _stream,
            stream_handle: Arc::new(stream_handle),
            footstep_sink,
            ambient_sink,
        }
    }

    pub fn play_footsteps(&self, file_path: &str) {
        if self.footstep_sink.empty() {
            let file = File::open(file_path).expect("Failed to open footstep sound file");
            let source = Decoder::new(BufReader::new(file)).expect("Failed to decode footstep sound file");
            self.footstep_sink.append(source.repeat_infinite()); // Repeat the sound infinitely
            self.footstep_sink.play();
        }
    }

    pub fn stop_footsteps(&self) {
        self.footstep_sink.stop();
    }

    pub fn play_ambient(&self, file_path: &str) {
        if self.ambient_sink.empty() {
            let file = File::open(file_path).expect("Failed to open ambient sound file");
            let source = Decoder::new(BufReader::new(file)).expect("Failed to decode ambient sound file");
            self.ambient_sink.append(source.repeat_infinite()); // Repeat the sound infinitely
            self.ambient_sink.set_volume(0.3); // Adjust the volume for ambient sound
            self.ambient_sink.play();
        }
    }

    pub fn stop_ambient(&self) {
        self.ambient_sink.stop();
    }
}