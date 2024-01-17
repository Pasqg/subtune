use crate::utils::math::{i, re};
use num_complex::Complex;
use std::f64::consts::PI;

pub(crate) fn fast_complex_fourier_transform(samples: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let samples_number = samples.len();
    if samples_number <= 4 {
        return complex_fourier_transform(&samples);
    }

    let mut frequencies = vec![re(0.0); samples_number];
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
        let cos_odd = cos * odd_frequencies[k];
        let sin_odd = sin * odd_frequencies[k];
        let odd = cos_odd.re + sin_odd.im + i(cos_odd.im - sin_odd.re);
        frequencies[k] = even_frequencies[k] + odd;
        frequencies[k + samples_number / 2] = even_frequencies[k] - odd;
    }

    return frequencies;
}

fn complex_fourier_transform(samples: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let samples_number = samples.len();
    let mut frequencies = Vec::with_capacity(samples_number);
    for k in 0..samples_number {
        let mut f = re(0.0);
        for j in 0..samples_number {
            let t = 2.0 * PI * (j * k) as f64 / (samples_number as f64);
            let cos = t.cos();
            let sin = t.sin();
            f += samples[j] * (cos - i(sin));
        }
        frequencies.push(f);
    }
    return frequencies;
}

// Fourier of fourier is N * the signal where after the first element, the elements are reversed
// FFT(FFT(X)) = N * X[0, N-1, ..., 1]
// So inverse transform is the transform of the transform, times 1/N, reversing elements after the first
pub(crate) fn inverse_fast_fourier_transform(transform: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let transform = fast_complex_fourier_transform(&transform);
    let length = transform.len();
    let mut result = vec![transform[0] / (length as f64); length];
    for i in 1..length {
        result[i] = transform[length - i] / (length as f64);
    }
    return result;
}

// This exists because it is faster than generic implementation which would perform extra mul+adds
pub(crate) fn fast_fourier_transform(samples: &Vec<f64>) -> Vec<Complex<f64>> {
    let samples_number = samples.len();
    if samples_number <= 4 {
        return fourier_transform(&samples);
    }

    let mut frequencies = vec![re(0.0); samples_number];
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
        let cos_odd = cos * odd_frequencies[k];
        let sin_odd = sin * odd_frequencies[k];
        let odd = cos_odd.re + sin_odd.im - i(cos_odd.im - sin_odd.re);
        frequencies[k] = even_frequencies[k] + odd;
        frequencies[k + samples_number / 2] = even_frequencies[k] - odd;
    }

    return frequencies;
}

fn fourier_transform(samples: &Vec<f64>) -> Vec<Complex<f64>> {
    let samples_number = samples.len();
    let mut frequencies = Vec::with_capacity(samples_number);
    for k in 0..samples_number {
        let mut f = re(0.0);
        for j in 0..samples_number {
            let t = 2.0 * PI * (j * k) as f64 / (samples_number as f64);
            let cos = t.cos();
            let sin = t.sin();
            f += samples[j] * cos - i(samples[j] * sin);
        }
        frequencies.push(f);
    }
    return frequencies;
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use num_complex::Complex;
    use rustfft::FftPlanner;

    use crate::signals::fourier::{
        complex_fourier_transform, fast_complex_fourier_transform, inverse_fast_fourier_transform,
    };
    use crate::utils::math::{assert_complex_vec, i, re};

    #[test]
    fn test_fourier_transform() {
        let signal = vec![re(-1.0), re(2.0), re(3.0), re(0.0)];
        let transform = complex_fourier_transform(&signal);
        assert_complex_vec(
            &transform,
            &vec![re(4.0), -4.0 + i(-2.0), re(0.0), -4.0 + i(2.0)],
        );

        let signal = vec![
            re(2.0),
            re(1.0),
            re(-1.0),
            re(5.0),
            re(0.0),
            re(3.0),
            re(0.0),
            re(-4.0),
        ];
        let transform = complex_fourier_transform(&signal);
        assert_complex_vec(
            &transform,
            &vec![
                re(6.0),
                -5.778174593052022 + i(-3.9497474683058345),
                3.0 + i(-3.0),
                9.778174593052025 + i(-5.94974746830583),
                re(-4.0),
                9.778174593052022 + i(5.9497474683058345),
                3.0 + i(3.0),
                -5.778174593052025 + i(3.94974746830583),
            ],
        );
    }

    #[test]
    fn test_inverse_fft() {
        let signal = vec![-1.0 + i(0.6), 2.0 - i(0.1), 3.0 - i(3.0), i(0.8)];
        let transform = complex_fourier_transform(&signal);
        let inverse = inverse_fast_fourier_transform(&transform);
        assert_complex_vec(&signal, &inverse);
    }

    #[test]
    fn test_result_equals_rustfft() {
        let signal = vec![1.0 + i(1.0); 4096];
        let transform = complex_fourier_transform(&signal);
        let fft = fast_complex_fourier_transform(&signal);
        let rust_fft_transform = rust_fft(&signal);

        assert_complex_vec(&rust_fft_transform, &transform);
        assert_complex_vec(&rust_fft_transform, &fft);
    }

    fn rust_fft(signal: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        let mut transform = signal.clone();
        let mut planner = FftPlanner::<f64>::new();
        let fft = planner.plan_fft_forward(transform.len());
        fft.process(&mut transform);
        transform
    }
}
