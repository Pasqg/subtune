mod transform;

use std::f32::consts::PI;

const PI2: f64 = (2.0 * PI) as f64;

fn morlet() -> impl Fn(f64) -> (f64, f64) {
    move |x| {
        let exp = (-x * x / 2.0).exp();
        let x2pi = PI2 * x;
        (x2pi.cos() * exp, x2pi.sin() * exp)
    }
}

#[cfg(test)]
mod tests {
    use crate::wavelets;
    use crate::wavelets::PI2;

    #[test]
    fn morlet_wavelet() {
        let wavelet = wavelets::morlet();
        assert_eq!(wavelet(0.0), (1.0, 0.0));
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