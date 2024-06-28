use crate::signals::SignalSample;
use crate::utils::math::FloatType;

pub mod argument_validation;
pub mod math;
pub mod visualization;

pub(crate) fn read_wav(file_path: &str) -> SignalSample<FloatType> {
    let mut reader = hound::WavReader::open(file_path).unwrap();
    SignalSample {
        sample_rate: reader.spec().sample_rate,
        samples: reader.samples::<i16>()
            .map(|sample| (sample.unwrap() as FloatType) / (i16::MAX as FloatType))
            .collect(),
    }
}
