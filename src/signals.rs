use std::f64::consts::PI;

//todo: opposed to SignalStream
pub struct SignalSample {
    pub sample_rate: u32,
    pub samples: Vec<f64>,
}

impl SignalSample {
    pub fn from_function(length_t: f64, sample_rate: u32, signal_fn: impl Fn(f64) -> f64) -> Self {
        let sample_rate_f64 = sample_rate as f64;
        let samples = (length_t * sample_rate_f64) as usize;
        let mut result = Vec::with_capacity(samples);

        for i in 0..samples {
            result.push(signal_fn(i as f64 / sample_rate_f64));
        }
        return Self {
            sample_rate,
            samples: result,
        };
    }
}

pub(crate) fn sine_signal(t: f64, frequency_hz: f64) -> f64 {
    (t * frequency_hz * 2.0 * PI).sin()
}