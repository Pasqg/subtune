use std::f64::consts::PI;
use crate::utils::math::ComplexNum;

const PI2: f64 = 2.0 * PI;

const WAVE_NUMBER: f64 = 16.0;
pub(crate) const MORLET_HALF_LENGTH: f64 = WAVE_NUMBER * 2.0;

pub(crate) fn morlet(frequency_hz: f64) -> impl Fn(f64) -> ComplexNum {
    move |t| {
        let d = frequency_hz / WAVE_NUMBER;
        let x = d * t - 2.0;
        let exp = (-x * x).exp();
        let x2pi = PI2 * t * frequency_hz;
        (x2pi.cos() * exp, x2pi.sin() * exp)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::math::assert_epsilon;
    use crate::signals::wavelets;
    use crate::signals::wavelets::MORLET_HALF_LENGTH;

    #[test]
    fn morlet_wavelet() {
        for frequency in 1..100 {
            let frequency_f64 = frequency as f64;
            let wavelet = wavelets::morlet(frequency_f64);
            assert_epsilon(wavelet(0.0).0, 0.0);
            assert_epsilon(wavelet(0.0).1, 0.0);
            assert_epsilon(wavelet(MORLET_HALF_LENGTH / frequency_f64).0, 1.0);
            assert_epsilon(wavelet(MORLET_HALF_LENGTH / frequency_f64).1, 0.0);
            assert_epsilon(wavelet(2.0 * MORLET_HALF_LENGTH / frequency_f64).0, 0.0);
            assert_epsilon(wavelet(2.0 * MORLET_HALF_LENGTH / frequency_f64).1, 0.0);
        }
    }
}