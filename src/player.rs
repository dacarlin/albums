use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};

pub struct Player {
    sink: Sink,
    _stream: OutputStream,
    _stream_handle: rodio::OutputStreamHandle,
}

impl Player {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        Ok(Player {
            sink,
            _stream: stream,
            _stream_handle: stream_handle,
        })
    }

    pub fn play(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file)?;
        self.sink.append(source);
        self.sink.play();
        Ok(())
    }

    pub fn add_song_to_sink(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file)?;
        self.sink.append(source);
        Ok(())
    }

    pub fn pause(&mut self) {
        self.sink.pause();
    }

    pub fn resume(&mut self) {
        self.sink.play();
    }

    pub fn stop(&mut self) {
        self.sink.stop();
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.sink.set_volume(volume);
    }
    
}