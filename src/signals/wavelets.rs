use num_complex::Complex;
use crate::utils::math::{FloatType, i};

const PI2: FloatType = 2.0 * std::f64::consts::PI as FloatType;

const WAVE_NUMBER: FloatType = 16.0;
pub(crate) const MORLET_HALF_LENGTH: FloatType = WAVE_NUMBER * 2.0;

pub(crate) fn morlet(frequency_hz: FloatType) -> impl Fn(FloatType) -> Complex<FloatType> {
    move |t| {
        let d = frequency_hz / WAVE_NUMBER;
        let x = d * t - 2.0;
        let exp = (-x * x).exp();
        let x2pi = PI2 * t * frequency_hz;
        x2pi.cos() * exp + i(x2pi.sin() * exp)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::math::{assert_epsilon, FloatType};
    use crate::signals::wavelets;
    use crate::signals::wavelets::MORLET_HALF_LENGTH;

    #[test]
    fn morlet_wavelet() {
        for frequency in 1..100 {
            let frequency_FloatType = frequency as FloatType;
            let wavelet = wavelets::morlet(frequency_FloatType);
            assert_epsilon(wavelet(0.0).re, 0.0);
            assert_epsilon(wavelet(0.0).im, 0.0);
            assert_epsilon(wavelet(MORLET_HALF_LENGTH / frequency_FloatType).re, 1.0);
            assert_epsilon(wavelet(MORLET_HALF_LENGTH / frequency_FloatType).im, 0.0);
            assert_epsilon(wavelet(2.0 * MORLET_HALF_LENGTH / frequency_FloatType).re, 0.0);
            assert_epsilon(wavelet(2.0 * MORLET_HALF_LENGTH / frequency_FloatType).im, 0.0);
        }
    }
}