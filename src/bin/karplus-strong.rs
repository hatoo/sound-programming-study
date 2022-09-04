use std::f64::consts::PI;

use rand::prelude::*;

const FS: usize = 44100;
const DURATION: usize = 4;
const F0: f64 = 440.0;
const T: f64 = 1.0 / F0;
const DECAY: f64 = 8.0;

struct ADSR {
    attack: usize,
    delay: usize,
    sustain: f64,
    release: usize,
    duration: usize,
}

impl ADSR {
    fn value(&self, i: usize) -> f64 {
        match i {
            i if i < self.attack => i as f64 / self.attack as f64,
            i if i < self.attack + self.delay => {
                let i = i - self.attack;
                1.0 - (1.0 - self.sustain) * (i as f64 / self.delay as f64)
            }
            i if i < self.duration - self.release => self.sustain,
            i => {
                let i = i + self.release - self.duration;
                if i >= self.release {
                    0.0
                } else {
                    self.sustain * (self.release - i) as f64 / self.release as f64
                }
            }
        }
    }
}

fn main() {
    let d: f64 = 0.5;
    let num = 10.0f64.powf(-3.0 * T / DECAY);
    let den =
        ((1.0 - d) * (1.0 - d) + 2.0 * d * (1.0 - d) * ((2.0 * PI * F0) / FS as f64).cos() + d * d)
            .sqrt();
    let c = (num / den).min(1.0);
    let delay = (T * FS as f64 - d) as usize;
    let cycle = (T * FS as f64) as usize;
    let e = T * FS as f64 - d - (T as f64 * FS as f64 - d).floor();
    let g = (1.0 - e) / (1.0 + e);

    let len = DURATION * FS;
    let mut rng = StdRng::seed_from_u64(114514);
    let mut s0: Vec<f64> = vec![0.0; len];

    for frame in &mut s0[..cycle] {
        *frame = rng.gen::<f64>() * 2.0 - 1.0;
    }

    let mean = s0[..cycle].iter().sum::<f64>() / cycle as f64;

    for frame in &mut s0[..cycle] {
        *frame -= mean;
    }

    let mut s1: Vec<f64> = vec![0.0; len];
    // let mut s2: Vec<f64> = vec![0.0; len];

    for i in cycle..len {
        // fractional delay
        s1[i] = -g * s1[i - 1] + g * s0[i - delay] + s0[i - delay - 1];

        // filter
        s0[i] = c * ((1.0 - d) * s1[i] + d * s1[i - 1]);

        /*

        s0[i] += s2[i];
        */
    }

    let adsr = ADSR {
        attack: 0,
        delay: 0,
        sustain: 1.0,
        release: (0.1 * FS as f64) as usize,
        duration: 3 * FS,
    };

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("out.wav", spec).unwrap();

    let max = s0.iter().fold(0.0f64, |a, b| a.max(b.abs()));
    for frame in &mut s0 {
        *frame /= max;
        *frame *= 0.5;
    }
    // mute time
    for _ in 0..FS {
        writer.write_sample(0).unwrap();
    }

    for (i, frame) in s0.into_iter().enumerate() {
        writer
            .write_sample((frame * adsr.value(i) * i16::MAX as f64) as i16)
            .unwrap();
    }
}
