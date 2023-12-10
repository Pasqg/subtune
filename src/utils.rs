use crate::signals::SignalSample;

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
        writer.write_sample((0.03 * sample * amplitude) as i16).unwrap();
    }
    writer.finalize().unwrap();
}
