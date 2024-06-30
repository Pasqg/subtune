use std::fs::File;
use minimp3::{Decoder, Error, Frame};
use crate::signals::SignalSample;
use crate::utils::math::FloatType;

pub mod argument_validation;
pub mod math;
pub mod visualization;

pub(crate) fn file_extension(file_path: &str) -> Option<&str> {
    let split: Vec<&str> = file_path.split('.').collect();
    if split.len() == 1 {
        return None;
    }
    Some(split[split.len() - 1])
}

pub(crate) fn read_audio(file_path: &str) -> SignalSample<FloatType> {
    let extension = file_extension(file_path);
    match extension {
        Some("wav") => read_wav(file_path),
        Some("mp3") => read_mp3(file_path),
        _ => panic!("Unrecognised file format for {}", file_path),
    }
}

fn read_wav(file_path: &str) -> SignalSample<FloatType> {
    let mut reader = hound::WavReader::open(file_path).unwrap();
    SignalSample {
        sample_rate: reader.spec().sample_rate,
        samples: reader.samples::<i16>()
            .map(|sample| to_float_sample(sample.unwrap()))
            .collect(),
    }
}

fn read_mp3(file_path: &str) -> SignalSample<FloatType> {
    let mut decoder = Decoder::new(File::open(file_path).unwrap());

    let mut samples = Vec::new();
    let mut eof_reached = false;
    let mut s_rate = 0;
    while !eof_reached {
        match decoder.next_frame() {
            Ok(Frame { data, sample_rate, channels, .. }) => {
                s_rate = sample_rate;
                for i in 0..(data.len() / channels) {
                    let mut sample = 0.0;
                    for c in 0..channels {
                        sample += to_float_sample(data[i * channels + c]);
                    }
                    samples.push(sample / (channels as FloatType));
                }
            }
            Err(Error::Eof) => eof_reached = true,
            Err(e) => panic!("Error reading {}: {:?}", file_path, e),
        }
    }
    SignalSample {
        sample_rate: s_rate as u32,
        samples,
    }
}

fn to_float_sample(sample: i16) -> FloatType {
    (sample as FloatType) / (i16::MAX as FloatType)
}
