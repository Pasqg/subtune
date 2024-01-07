use crate::signals::SignalSample;
use crate::math::{complex_mul, complex_sum, ComplexNum, scalar_complex_mul};
use crate::wavelets::fourier::{fast_complex_fourier_transform, fast_fourier_transform, inverse_fast_fourier_transform};

/// wavelet_factory: from (frequency, sample rate) to a SignalSample lasting 1/frequency
pub(crate) fn wavelet_transform(signal: &SignalSample<f64>,
                                wavelet_factory: &impl Fn(f64, u32) -> SignalSample<ComplexNum>,
                                frequencies: &Vec<f64>) -> Vec<Vec<ComplexNum>> {
    let mut result = Vec::with_capacity(frequencies.len());
    for frequency_hz in frequencies {
        let wavelet_samples = wavelet_factory(*frequency_hz, signal.sample_rate);

        let convolution = if wavelet_samples.samples.len() >= signal.samples.len() / 20 {
            fourier_convolution(&signal.samples, &wavelet_samples.samples)
        } else {
            complex_convolution(&signal.samples, &wavelet_samples.samples)
        };
        let convolution: Vec<ComplexNum> = convolution.iter().map(|c| scalar_complex_mul(1.0 / (wavelet_samples.samples.len() as f64), *c)).collect();
        result.push(convolution[(wavelet_samples.samples.len() - 1)..(signal.samples.len() + wavelet_samples.samples.len() - 1)].to_vec());
    }
    return result;
}

fn complex_convolution(signal: &Vec<f64>, kernel: &Vec<ComplexNum>) -> Vec<ComplexNum> {
    let signal_len = signal.len() as i64;
    let kernel_len = kernel.len() as i64;

    let mut convolution_result = Vec::with_capacity(signal_len as usize);
    for signal_i in -(kernel_len - 1)..signal_len {
        let mut convolution = (0.0, 0.0);
        for kernel_i in 0..kernel_len {
            if signal_i + kernel_i >= 0 && signal_i + kernel_i < signal_len {
                let signal_at = signal[(signal_i + kernel_i) as usize];
                let kernel_at = kernel[(kernel_len - kernel_i - 1) as usize];
                convolution = complex_sum(convolution, scalar_complex_mul(signal_at, kernel_at));
            }
        }
        convolution_result.push(convolution);
    }
    return convolution_result;
}

fn round_to_power_2(n: i64) -> i64 {
    let power = n.ilog2();
    let smaller = 2i64.pow(power);
    if n == smaller {
        return smaller;
    }
    return smaller * 2;
}

fn fourier_convolution(signal: &Vec<f64>, kernel: &Vec<ComplexNum>) -> Vec<ComplexNum> {
    let signal_len = signal.len();
    let kernel_len = kernel.len();
    let convolution_len = signal_len + kernel_len - 1;

    let convolution_len = round_to_power_2(convolution_len as i64) as usize;

    let padded_signal = pad(signal, convolution_len, 0.0);
    let padded_kernel = pad(kernel, convolution_len, (0.0, 0.0));

    let mut signal_transform = fast_fourier_transform(&padded_signal);
    let kernel_transform = fast_complex_fourier_transform(&padded_kernel);
    for i in 0..convolution_len {
        signal_transform[i] = complex_mul(signal_transform[i], kernel_transform[i]);
    }
    return inverse_fast_fourier_transform(&signal_transform);
}

fn pad<T: Copy>(vector: &Vec<T>, new_length: usize, default: T) -> Vec<T> {
    let mut padded_kernel = vec![default; new_length];
    for i in 0..vector.len() {
        padded_kernel[i] = vector[i];
    }
    padded_kernel
}

#[cfg(test)]
mod tests {
    use crate::math::assert_complex_vec;
    use crate::signals::SignalSample;
    use crate::wavelets::transform::{complex_convolution, fourier_convolution, pad, round_to_power_2, wavelet_transform};

    #[test]
    fn test_convolution_real_part() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];
        let wavelet = vec![(1.0, 0.0), (-2.0, 0.0), (0.5, 0.0)];

        let convolution = complex_convolution(&signal, &wavelet);
        assert_complex_vec(convolution, vec![(0.3, 0.0), (-0.1, 0.0), (-1.85, 0.0),
                                             (2.95, 0.0), (-1.9, 0.0), (0.35, 0.0)]);
    }

    #[test]
    fn test_convolution_img_part() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];
        let wavelet = vec![(0.0, 1.0), (0.0, -2.0), (0.0, 0.5)];

        let convolution = complex_convolution(&signal, &wavelet);
        assert_complex_vec(convolution, vec![(0.0, 0.3), (0.0, -0.1), (0.0, -1.85),
                                             (0.0, 2.95), (0.0, -1.9), (0.0, 0.35)]);
    }

    #[test]
    fn test_convolution_both_parts() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];
        let wavelet = vec![(0.4, 1.0), (0.6, -2.0), (-0.2, 0.5)];

        let convolution = complex_convolution(&signal, &wavelet);
        assert_complex_vec(convolution, vec![(0.12, 0.3), (0.38, -0.1), (-0.16, -1.85),
                                             (-0.42, 2.95), (0.62, -1.9), (-0.14, 0.35)]);
    }

    #[test]
    fn test_transform() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];

        let signal_sample = SignalSample {
            sample_rate: 3,
            samples: signal,
        };
        let frequencies = vec![1.0];
        let transform = wavelet_transform(&signal_sample,
                                          &|f, _| SignalSample {
                                              sample_rate: 3,
                                              samples: vec![(0.4, 1.0), (0.6, -2.0), (-0.2, 0.5)],
                                          },
                                          &frequencies);
        assert_complex_vec(transform[0].to_vec(), vec![(-0.16 / 3.0, -1.85 * 3.0), (-0.42 / 3.0, 2.95 / 3.0),
                                                       (0.62 / 3.0, -1.9 / 3.0), (-0.14 / 3.0, 0.35 / 3.0)]);
    }

    #[test]
    fn test_power_rounding() {
        assert_eq!(round_to_power_2(1), 1);
        assert_eq!(round_to_power_2(2), 2);
        assert_eq!(round_to_power_2(3), 4);
        assert_eq!(round_to_power_2(4), 4);
        assert_eq!(round_to_power_2(13), 16);
        assert_eq!(round_to_power_2(16), 16);
        assert_eq!(round_to_power_2(1023), 1024);
        assert_eq!(round_to_power_2(1024), 1024);
        assert_eq!(round_to_power_2(1025), 2048);
        assert_eq!(round_to_power_2(1237), 2048);
        assert_eq!(round_to_power_2(13972497247), 17179869184);
    }

    #[test]
    fn test_fourier_convolution() {
        let signal = vec![0.3, 0.5, -1.0, 0.0];
        let wavelet = vec![(0.2, 0.0), (-0.7, 0.0), (0.4, 0.0)];

        let fourier_convolution = fourier_convolution(&signal, &wavelet);

        let convolution = complex_convolution(&signal, &wavelet);
        let convolution = pad(&convolution, 8, (0.0, 0.0));

        assert_complex_vec(convolution, fourier_convolution);
    }
}