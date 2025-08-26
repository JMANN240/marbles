use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    time::Duration,
};

use hound::{SampleFormat, WavReader, WavSpec, WavWriter};

#[derive(Debug)]
pub struct Collision {
    pub sound_path: PathBuf,
    pub volume: f32,
}

impl Collision {
    pub fn new(sound_path: PathBuf, volume: f32) -> Self {
        Self { sound_path, volume }
    }
}

pub fn render_collisions<P: AsRef<Path>>(
    output_path: P,
    collisions: &HashMap<usize, Vec<Collision>>,
    duration: Duration,
    sample_rate: u32,
) {
    let sound_paths = collisions
        .values()
        .flat_map(|collisions| {
            collisions
                .iter()
                .map(|collision| collision.sound_path.clone())
        })
        .collect::<HashSet<PathBuf>>();

    let sound_samples = sound_paths
        .into_iter()
        .map(|sound_path| {
            let mut reader = WavReader::open(&sound_path).unwrap();
            (
                sound_path,
                reader
                    .samples::<i16>()
                    .map(|s| s.unwrap() as f32 / i16::MAX as f32)
                    .collect(),
            )
        })
        .collect::<HashMap<PathBuf, Vec<f32>>>();

    let total_samples = (2.0 * duration.as_secs_f64() * sample_rate as f64).ceil() as usize;
    let mut mix = vec![0.0f32; total_samples];

    for (frame, collisions) in collisions {
        let time_sec = *frame as f32 / 60.0;
        let offset_samples = (2.0 * time_sec * sample_rate as f32).round() as usize;

        for collision in collisions {
            let samples = sound_samples.get(&collision.sound_path).unwrap();

            for (index, sample) in samples.iter().enumerate() {
                if offset_samples + index >= mix.len() {
                    break;
                }

                mix[offset_samples + index] += sample * collision.volume;
            }
        }
    }

    let spec = WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create(output_path, spec).unwrap();
    for sample in mix {
        writer
            .write_sample((sample * i16::MAX as f32) as i16)
            .unwrap();
    }
    writer.finalize().unwrap();
}
