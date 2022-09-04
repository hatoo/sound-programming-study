use rand::prelude::*;

const FS: usize = 44100;
const F0: usize = 440;
const DURATION: usize = 4;

fn main() {
    let mut rng = StdRng::seed_from_u64(114514);
    let mut s: Vec<f64> = vec![0.0; DURATION * FS];

    for frame in &mut s {
        *frame = rng.gen::<f64>() * 2.0 - 1.0;
    }

    let mean = s.iter().sum::<f64>() / s.len() as f64;

    for frame in &mut s {
        *frame -= mean;
    }

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("out.wav", spec).unwrap();

    for frame in s {
        writer
            .write_sample((frame * i64::MAX as f64) as i16)
            .unwrap();
    }
}
