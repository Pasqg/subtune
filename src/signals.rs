use num_complex::Complex;

pub mod fourier;
pub mod transform;
pub mod wavelets;

//todo: opposed to SignalStream
pub struct SignalSample<T> {
    pub sample_rate: u32,
    pub samples: Vec<T>,
}

impl SignalSample<f64> {
    pub fn from_signal(length_t: f64, sample_rate: u32, signal_fn: &impl Fn(f64) -> f64) -> Self {
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

impl SignalSample<Complex<f64>> {
    pub fn from_wavelet(length_t: f64, sample_rate: u32, signal_fn: &impl Fn(f64) -> Complex<f64>) -> Self {
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
