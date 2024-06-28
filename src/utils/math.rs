use num_complex::{Complex};

pub(crate) type FloatType = f64;

pub(crate) fn i(i: FloatType) -> Complex<FloatType> {
    Complex { re: 0.0, im: i }
}

pub(crate) fn re(re: FloatType) -> Complex<FloatType> {
    Complex { re, im: 0.0 }
}

pub(crate) fn assert_epsilon(actual: FloatType, expected: FloatType) {
    if (expected - actual).abs() > 1e-6 {
        panic!("Expected {:?} to be equal to {:?} with an epsilon of 1e-6", actual, expected);
    }
}

pub(crate) fn assert_complex_vec(actual: &[Complex<FloatType>], expected: &[Complex<FloatType>]) {
    if actual.len() != expected.len() {
        panic!("Expected size {:?} but got {:?}", expected.len(), actual.len());
    }
    for i in 0..actual.len() {
        assert_complex(actual[i], expected[i], i);
    }
}

pub(crate) fn assert_complex(actual: Complex<FloatType>, expected: Complex<FloatType>, index: usize) {
    if (expected.re - actual.re).abs() > 1e-6 || (expected.im - actual.im).abs() > 1e-6 {
        panic!("Expected {:?} to be equal to {:?} with an epsilon of 1e-6 at index {}", actual, expected, index);
    }
}