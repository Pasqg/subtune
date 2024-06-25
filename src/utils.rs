use crate::signals::SignalSample;

pub mod argument_validation;
pub mod math;
pub mod visualization;

pub(crate) fn read_wav(file_path: &str) -> SignalSample<f64> {
    let mut reader = hound::WavReader::open(file_path).unwrap();
    SignalSample {
        sample_rate: reader.spec().sample_rate,
        samples: reader.samples::<i16>()
            .map(|sample| (sample.unwrap() as f64) / (i16::MAX as f64))
            .collect(),
    }
}
