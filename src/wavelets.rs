pub mod transform;

use std::f32::consts::PI;
use crate::math::ComplexNum;

const PI2: f64 = (2.0 * PI) as f64;
pub(crate) const MORLET_HALF_LENGTH: f64 = 4.0;

pub(crate) fn morlet(frequency_hz: f64) -> impl Fn(f64) -> ComplexNum {
    move |t| {
        let x = frequency_hz * t - MORLET_HALF_LENGTH;
        let exp = (-x * x).exp();
        let x2pi = PI2 * t * frequency_hz;
        (x2pi.cos() * exp, x2pi.sin() * exp)
    }
}

#[cfg(test)]
mod tests {
    use crate::wavelets;
    use crate::wavelets::MORLET_HALF_LENGTH;

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

    fn assert_epsilon(actual: f64, expected: f64) {
        if (expected - actual).abs() > 1e-6 {
            panic!("Expected {:?} to be equal to {:?} with an epsilon of 1e-6", actual, expected);
        }
    }
}