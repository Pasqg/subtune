pub(crate) type ComplexNum = (f64, f64);

pub(crate) fn modulo(num: ComplexNum) -> f64 {
    (num.0 * num.0 + num.1 * num.1).sqrt()
}

pub(crate) fn scalar_complex_mul(scalar: f64, num: ComplexNum) -> ComplexNum {
    (scalar * num.0, scalar * num.1)
}

pub(crate) fn complex_sum(first: ComplexNum, second: ComplexNum) -> ComplexNum {
    (first.0 + second.0, first.1 + second.1)
}

pub(crate) fn complex_mul(first: ComplexNum, second: ComplexNum) -> ComplexNum {
    (first.0 * second.0 - first.1 * second.1, first.0 * second.1 + first.1 * second.0)
}

pub(crate) fn assert_epsilon(actual: f64, expected: f64) {
    if (expected - actual).abs() > 1e-6 {
        panic!("Expected {:?} to be equal to {:?} with an epsilon of 1e-6", actual, expected);
    }
}

pub(crate) fn assert_complex_vec(actual: Vec<ComplexNum>, expected: Vec<ComplexNum>) {
    if actual.len() != expected.len() {
        panic!("Expected size {:?} but got {:?}", expected.len(), actual.len());
    }
    for i in 0..actual.len() {
        assert_complex(actual[i], expected[i]);
    }
}

pub(crate) fn assert_complex(actual: ComplexNum, expected: ComplexNum) {
    if (expected.0 - actual.0).abs() > 1e-6 || (expected.1 - actual.1).abs() > 1e-6 {
        panic!("Expected {:?} to be equal to {:?} with an epsilon of 1e-6", actual, expected);
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{modulo, scalar_complex_mul};

    #[test]
    fn real_number_modulo_is_square() {
        assert_eq!(modulo((2.0, 0.0)), 2.0);
        assert_eq!(modulo((-2.0, 0.0)), 2.0);
        assert_eq!(modulo((0.0, 2.0)), 2.0);
        assert_eq!(modulo((0.0, -2.0)), 2.0);
    }

    #[test]
    fn scalar_complex_mul_scales_complex_and_real_part() {
        assert_eq!(scalar_complex_mul(3.0, (2.0, 3.0)), (6.0, 9.0));
        assert_eq!(scalar_complex_mul(3.0, (-2.0, -3.0)), (-6.0, -9.0));
    }
}