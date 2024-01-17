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

pub(crate) fn save_wav(filename: &str, sample: &SignalSample<f64>) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample.sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    let amplitude = i16::MAX as f64;
    for sample in &sample.samples {
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
    writer.finalize().unwrap();
}
