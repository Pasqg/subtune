use std::f64::consts::PI;
use crate::math::{complex_sum, ComplexNum, scalar_complex_mul};

pub(crate) fn fast_complex_fourier_transform(samples: &Vec<ComplexNum>) -> Vec<ComplexNum> {
    let samples_number = samples.len();
    if samples_number <= 4 {
        return complex_fourier_transform(&samples);
    }

    let mut frequencies = vec![(0.0, 0.0); samples_number];
    let mut even_samples = Vec::with_capacity(samples_number / 2);
    let mut odd_samples = Vec::with_capacity(samples_number / 2);

    for i in (0..samples_number).step_by(2) {
        even_samples.push(samples[i]);
        odd_samples.push(samples[i + 1]);
    }

    let even_frequencies = fast_complex_fourier_transform(&even_samples);
    let odd_frequencies = fast_complex_fourier_transform(&odd_samples);

    for k in 0..samples_number / 2 {
        let t = 2.0 * PI * k as f64 / samples_number as f64;
        let sin = t.sin();
        let cos = t.cos();
        let cos_odd = scalar_complex_mul(cos, odd_frequencies[k]);
        let sin_odd = scalar_complex_mul(sin, odd_frequencies[k]);
        let re = cos_odd.0 + sin_odd.1;
        let img = cos_odd.1 - sin_odd.0;
        frequencies[k] =
            (even_frequencies[k].0 + re, even_frequencies[k].1 + img);
        frequencies[k + samples_number / 2] =
            (even_frequencies[k].0 - re, even_frequencies[k].1 - img);
    }

    return frequencies;
}

fn complex_fourier_transform(samples: &Vec<ComplexNum>) -> Vec<ComplexNum> {
    let samples_number = samples.len();
    let mut frequencies = Vec::with_capacity(samples_number);
    for k in 0..samples_number {
        let mut f = (0.0, 0.0);
        for i in 0..samples_number {
            let t = 2.0 * PI * (i * k) as f64 / (samples_number as f64);
            let cos = t.cos();
            let sin = t.sin();
            f = complex_sum(f, (samples[i].0 * cos + samples[i].1 * sin, samples[i].1 * cos - samples[i].0 * sin));
        }
        frequencies.push(f);
    }
    return frequencies;
}

// Fourier of fourier is N * the signal where after the first element, the elements are reversed
// FFT(FFT(X)) = N * X[0, N-1, ..., 1]
// So inverse transform is the transform of the transform, times 1/N, reversing elements after the first
pub(crate) fn inverse_fast_fourier_transform(transform: &Vec<ComplexNum>) -> Vec<ComplexNum> {
    let transform = fast_complex_fourier_transform(&transform);
    let length = transform.len();
    let mut result = vec![scalar_complex_mul(1.0 / (length as f64), transform[0]); length];
    for i in 1..length {
        result[i] = scalar_complex_mul(1.0 / (length as f64), transform[length - i]);
    }
    return result;
}

// This exists because it is faster than generic implementation which would perform extra mul+adds
pub(crate) fn fast_fourier_transform(samples: &Vec<f64>) -> Vec<ComplexNum> {
    let samples_number = samples.len();
    if samples_number <= 4 {
        return fourier_transform(&samples);
    }

    let mut frequencies = vec![(0.0, 0.0); samples_number];
    let mut even_samples = Vec::with_capacity(samples_number / 2);
    let mut odd_samples = Vec::with_capacity(samples_number / 2);

    for i in (0..samples_number).step_by(2) {
        even_samples.push(samples[i]);
        odd_samples.push(samples[i + 1]);
    }

    let even_frequencies = fast_fourier_transform(&even_samples);
    let odd_frequencies = fast_fourier_transform(&odd_samples);

    for k in 0..samples_number / 2 {
        let t = 2.0 * PI * k as f64 / samples_number as f64;
        let sin = t.sin();
        let cos = t.cos();
        let cos_odd = scalar_complex_mul(cos, odd_frequencies[k]);
        let sin_odd = scalar_complex_mul(sin, odd_frequencies[k]);
        let re = cos_odd.0 + sin_odd.1;
        let img = cos_odd.1 - sin_odd.0;
        frequencies[k] =
            (even_frequencies[k].0 + re, even_frequencies[k].1 + img);
        frequencies[k + samples_number / 2] =
            (even_frequencies[k].0 - re, even_frequencies[k].1 - img);
    }

    return frequencies;
}

fn fourier_transform(samples: &Vec<f64>) -> Vec<ComplexNum> {
    let samples_number = samples.len();
    let mut frequencies = Vec::with_capacity(samples_number);
    for k in 0..samples_number {
        let mut f = (0.0, 0.0);
        for i in 0..samples_number {
            let t = 2.0 * PI * (i * k) as f64 / (samples_number as f64);
            let cos = t.cos();
            let sin = t.sin();
            f = complex_sum(f, (samples[i] * cos, -samples[i] * sin));
        }
        frequencies.push(f);
    }
    return frequencies;
}

#[cfg(test)]
mod tests {
    use crate::math::{assert_complex_vec};
    use crate::wavelets::fourier::{complex_fourier_transform, inverse_fast_fourier_transform};

    #[test]
    fn test_fourier_transform() {
        let signal = vec![(-1.0, 0.0), (2.0, 0.0), (3.0, 0.0), (0.0, 0.0)];
        let transform = complex_fourier_transform(&signal);
        assert_complex_vec(transform, vec![(4.0, 0.0), (-4.0, -2.0), (0.0, 0.0), (-4.0, 2.0)]);

        let signal = vec![(2.0, 0.0), (1.0, 0.0), (-1.0, 0.0), (5.0, 0.0), (0.0, 0.0), (3.0, 0.0), (0.0, 0.0), (-4.0, 0.0)];
        let transform = complex_fourier_transform(&signal);
        assert_complex_vec(transform, vec![(6.0, 0.0),
                                           (-5.778174593052022, -3.9497474683058345),
                                           (3.0, -3.0),
                                           (9.778174593052025, -5.94974746830583),
                                           (-4.0, 0.0),
                                           (9.778174593052022, 5.9497474683058345),
                                           (3.0, 3.0),
                                           (-5.778174593052025, 3.94974746830583)]);
    }

    #[test]
    fn test_complex_fourier_transform() {
        let signal = vec![(-1.0, 0.0), (2.0, 0.0), (3.0, 0.0), (0.0, 0.0)];
        let transform = complex_fourier_transform(&signal);
        assert_complex_vec(transform, vec![(4.0, 0.0), (-4.0, -2.0), (0.0, 0.0), (-4.0, 2.0)]);

        let signal = vec![(2.0, 0.0), (1.0, 0.0), (-1.0, 0.0), (5.0, 0.0), (0.0, 0.0), (3.0, 0.0), (0.0, 0.0), (-4.0, 0.0)];
        let transform = complex_fourier_transform(&signal);
        assert_complex_vec(transform, vec![(6.0, 0.0),
                                           (-5.778174593052022, -3.9497474683058345),
                                           (3.0, -3.0),
                                           (9.778174593052025, -5.94974746830583),
                                           (-4.0, 0.0),
                                           (9.778174593052022, 5.9497474683058345),
                                           (3.0, 3.0),
                                           (-5.778174593052025, 3.94974746830583)]);


        let signal = vec![(-1.0, 0.6), (2.0, -0.1), (3.0, -3.0), (0.0, 0.8)];
        let transform = complex_fourier_transform(&signal);
        assert_complex_vec(transform, vec![(4.0, -1.7), (-4.9, 1.6), (0.0, -3.1), (-3.1, 5.6)]);
    }

    #[test]
    fn test_inverse_fft() {
        let signal = vec![(-1.0, 0.6), (2.0, -0.1), (3.0, -3.0), (0.0, 0.8)];
        let transform = complex_fourier_transform(&signal);
        let inverse = inverse_fast_fourier_transform(&transform);
        assert_complex_vec(signal, inverse);
    }
}