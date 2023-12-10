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