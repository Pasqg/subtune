mod transform;

use std::f32::consts::PI;
use crate::math::ComplexNum;

const PI2: f64 = (2.0 * PI) as f64;

pub(crate) fn morlet(duration_s: f64, frequency_hz: f64) -> impl Fn(f64) -> ComplexNum {
    move |t| {
        let k = 2.0;
        let x = 4.0 * t / duration_s - 2.0;
        let exp = (-k * x * x).exp();
        let x2pi = PI2 * x * frequency_hz;
        (x2pi.cos() * exp, x2pi.sin() * exp)
    }
}

#[cfg(test)]
mod tests {
    use crate::wavelets;
    use crate::wavelets::PI2;

    #[test]
    fn morlet_wavelet() {
        for duration in 1..100 {
            let wavelet = wavelets::morlet(duration as f64, 1.0);
            assert_eq!(wavelet(0.0), (0.0003354626279024913, -1.1730830207436842e-10));
            assert_eq!(wavelet(duration as f64 / 2.0), (1.0, 0.0));
            assert_eq!(wavelet(duration as f64), (0.0003354626279024913, 1.1730830207436842e-10));
        }
    }

    #[test]
    fn morlet_kk() {
        let signal = (0..1000).map(|i| ((i as f64) * 0.01 * PI2).sin()).collect::<Vec<f64>>();

        assert_epsilon(signal[0], 0.0);
        assert_epsilon(signal[25], 1.0);
        assert_epsilon(signal[50], 0.0);
        assert_epsilon(signal[75], -1.0);
        assert_epsilon(signal[100], 0.0);
    }

    fn assert_epsilon(actual: f64, expected: f64) {
        if (expected - actual).abs() > 1e-6 {
            panic!("Expected {:?} to be equal to {:?} with an epsilon of 1e6", actual, expected);
        }
    }
}