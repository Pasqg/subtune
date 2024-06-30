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

pub(crate) fn read_wav(file_path: &str) -> SignalSample<FloatType> {
    let mut reader = hound::WavReader::open(file_path).unwrap();
    SignalSample {
        sample_rate: reader.spec().sample_rate,
        samples: reader.samples::<i16>()
            .map(|sample| (sample.unwrap() as FloatType) / (i16::MAX as FloatType))
            .collect(),
    }
}
