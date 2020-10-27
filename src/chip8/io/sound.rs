use sdl2::Sdl;
use sdl2::audio::{AudioDevice, AudioCallback, AudioSpecDesired};

const PHASE_MAX: f32 = 1.0;
const PHASE_THRESHOLD: f32 = 0.5;
const SOUND_FREQUENCY: f32 = 440.0;
const SOUND_VOLUME: f32 = 0.25;
const SAMPLING_FREQUENCY: i32 = 44_100;

pub struct Sound {
    device: AudioDevice<SquareWave>,
}

impl Sound {
    pub fn new(sdl: &Sdl) -> Result<Self, String> {
        let audio = sdl.audio()?;

        let desired_spec = AudioSpecDesired {
            freq: Some(SAMPLING_FREQUENCY),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        let device = audio.open_playback(None, &desired_spec, |spec| {
            SquareWave {
                phase: 0.0,
                phase_inc: SOUND_FREQUENCY / spec.freq as f32,
                volume: SOUND_VOLUME,
            }
        })?;

        Ok(Sound { device })
    }

    pub fn set(&self, is_on: bool) {
        if is_on {
            self.unmute();
        } else {
            self.mute();
        }
    }

    pub fn mute(&self) {
        self.device.pause();
    }

    pub fn unmute(&self) {
        self.device.resume()
    }
}

struct SquareWave {
    phase: f32,
    phase_inc: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= PHASE_THRESHOLD { self.volume } else { -self.volume };
            self.phase = (self.phase + self.phase_inc) % PHASE_MAX;
        }
    }
}
