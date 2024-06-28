use num_complex::Complex;
use crate::utils::math::FloatType;

pub mod fourier;
pub mod transform;
pub mod wavelets;

//todo: opposed to SignalStream
pub struct SignalSample<T> {
    pub sample_rate: u32,
    pub samples: Vec<T>,
}

impl SignalSample<Complex<FloatType>> {
    pub fn from_wavelet(length_t: FloatType, sample_rate: u32, signal_fn: &impl Fn(FloatType) -> Complex<FloatType>) -> Self {
        let sample_rate_FloatType = sample_rate as FloatType;
        let samples = (length_t * sample_rate_FloatType) as usize;
        let mut result = Vec::with_capacity(samples);

        for i in 0..samples {
            result.push(signal_fn(i as FloatType / sample_rate_FloatType));
        }
        Self {
            sample_rate,
            samples: result,
        }
    }
}
