use num_complex::{Complex};

pub(crate) fn i(i: f64) -> Complex<f64> {
    Complex { re: 0.0, im: i }
}

pub(crate) fn re(re: f64) -> Complex<f64> {
    Complex { re, im: 0.0 }
}

pub(crate) fn assert_epsilon(actual: f64, expected: f64) {
    if (expected - actual).abs() > 1e-6 {
        panic!("Expected {:?} to be equal to {:?} with an epsilon of 1e-6", actual, expected);
    }
}

pub(crate) fn assert_complex_vec(actual: &Vec<Complex<f64>>, expected: &Vec<Complex<f64>>) {
    if actual.len() != expected.len() {
        panic!("Expected size {:?} but got {:?}", expected.len(), actual.len());
    }
    for i in 0..actual.len() {
        assert_complex(actual[i], expected[i], i);
    }
}

pub(crate) fn assert_complex(actual: Complex<f64>, expected: Complex<f64>, index: usize) {
    if (expected.re - actual.re).abs() > 1e-6 || (expected.im - actual.im).abs() > 1e-6 {
        panic!("Expected {:?} to be equal to {:?} with an epsilon of 1e-6 at index {}", actual, expected, index);
    }
}