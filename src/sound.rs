use rodio;
use std::fs::File;
use std::io::BufReader;
use rodio::Source;
use std::collections::HashMap;

pub struct Sound_Manager {
    
    device: rodio::Device,
    sound_sink_1: rodio::Sink,
    sound_sink_2: rodio::Sink,
}

impl Sound_Manager {

    pub fn new(device: rodio::Device) -> Sound_Manager {

        let sound_sink_1 = rodio::Sink::new(&device);
        let sound_sink_2 = rodio::Sink::new(&device);

        Sound_Manager { device, sound_sink_1, sound_sink_2 }
    }

    pub fn play_sound(&self, name: &str, channel: u32) {

        let mut path = "assets/sounds/".to_string();
        path.push_str(name);
        path.push_str(".wav");

        let file = File::open(path).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

        if channel == 0 {
            self.sound_sink_1.append(source);
        }else if channel == 1 {
            self.sound_sink_2.append(source);
        }
    }

    pub fn set_volume(&self, volume: f32, channel: u32)  {

        if channel == 0 {
            self.sound_sink_1.set_volume(volume);
        }else if channel == 1 {
            self.sound_sink_2.set_volume(volume);
        }
    }
}