use crate::signals::SignalSample;
use crate::math::{complex_sum, ComplexNum, modulo, scalar_complex_mul};

/// wavelet_factory: from (frequency, sample rate) to a SignalSample lasting 1/frequency
pub(crate) fn wavelet_transform(signal: &SignalSample<f64>, wavelet_factory: &impl Fn(f64, u32) -> SignalSample<ComplexNum>,
                                start_frequency: f64, end_frequency: f64, n_frequencies: u32) -> Vec<Vec<ComplexNum>> {
    let mut result = Vec::with_capacity(n_frequencies as usize);
    for frequency_index in 0..n_frequencies {
        let frequency = start_frequency + (end_frequency - start_frequency) * (frequency_index as f64) / (n_frequencies as f64);
        let wavelet_samples = wavelet_factory(frequency, signal.sample_rate);
        let vec = complex_convolution(&signal.samples, &wavelet_samples.samples);
        result.push(vec.split_at(wavelet_samples.samples.len() - 1).1.to_vec());
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
                convolution = complex_sum(convolution, scalar_complex_mul(signal_at / (kernel_len as f64), kernel_at));
            }
        }
        convolution_result.push(convolution);
    }
    return convolution_result;
}

#[cfg(test)]
mod tests {
    use crate::math::assert_complex_vec;
    use crate::signals::SignalSample;
    use crate::wavelets::transform::{complex_convolution, wavelet_transform};

    #[test]
    fn test_convolution_real_part() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];
        let wavelet = vec![(1.0, 0.0), (-2.0, 0.0), (0.5, 0.0)];

        let convolution = complex_convolution(&signal, &wavelet);
        assert_complex_vec(convolution, vec![(0.3 / 3.0, 0.0), (-0.1 / 3.0, 0.0), (-1.85 / 3.0, 0.0),
                                             (2.95 / 3.0, 0.0), (-1.9 / 3.0, 0.0), (0.35 / 3.0, 0.0)]);
    }

    #[test]
    fn test_convolution_img_part() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];
        let wavelet = vec![(0.0, 1.0), (0.0, -2.0), (0.0, 0.5)];

        let convolution = complex_convolution(&signal, &wavelet);
        assert_complex_vec(convolution, vec![(0.0, 0.3 / 3.0), (0.0, -0.1 / 3.0), (0.0, -1.85 / 3.0),
                                             (0.0, 2.95 / 3.0), (0.0, -1.9 / 3.0), (0.0, 0.35 / 3.0)]);
    }

    #[test]
    fn test_convolution_both_parts() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];
        let wavelet = vec![(0.4, 1.0), (0.6, -2.0), (-0.2, 0.5)];

        let convolution = complex_convolution(&signal, &wavelet);
        assert_complex_vec(convolution, vec![(0.12 / 3.0, 0.3 / 3.0), (0.38 / 3.0, -0.1 / 3.0), (-0.16 / 3.0, -1.85 / 3.0),
                                             (-0.42 / 3.0, 2.95 / 3.0), (0.62 / 3.0, -1.9 / 3.0), (-0.14 / 3.0, 0.35 / 3.0)]);
    }

    #[test]
    fn test_transform() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];

        let signal_sample = SignalSample {
            sample_rate: 3,
            samples: signal,
        };
        let transform = wavelet_transform(&signal_sample,
                                          &|f, _| SignalSample {
                                              sample_rate: 3,
                                              samples: vec![(0.4, 1.0), (0.6, -2.0), (-0.2, 0.5)],
                                          },
                                          1.0, 1.0, 1);
        assert_complex_vec(transform[0].to_vec(), vec![(-0.16 / 3.0, -1.85 / 3.0), (-0.42 / 3.0, 2.95 / 3.0),
                                      (0.62 / 3.0, -1.9 / 3.0), (-0.14 / 3.0, 0.35 / 3.0)]);
    }
}