use crate::signals::SignalSample;
use crate::math::{complex_sum, ComplexNum, modulo, scalar_complex_mul};

/// wavelet_factory: from (frequency, sample rate) to a SignalSample lasting 1/frequency
pub(crate) fn wavelet_transform(signal: &SignalSample<f64>, wavelet_factory: &impl Fn(f64, u32) -> SignalSample<ComplexNum>,
                                start_frequency: f64, end_frequency: f64, n_frequencies: u32) -> Vec<Vec<ComplexNum>> {
    let mut result = Vec::with_capacity(n_frequencies as usize);
    for frequency_index in 0..n_frequencies {
        let frequency = start_frequency + (end_frequency - start_frequency) * (frequency_index as f64) / (n_frequencies as f64);
        let wavelet_samples = wavelet_factory(frequency, signal.sample_rate);
        result.push(complex_convolution(&signal.samples, &wavelet_samples.samples));
    }
    return result;
}

fn complex_convolution(signal_samples: &Vec<f64>, wavelet_samples: &Vec<ComplexNum>) -> Vec<ComplexNum> {
    let signal_len = signal_samples.len();
    let mut convolution_result = Vec::with_capacity(signal_len);
    let wavelet_len = wavelet_samples.len();
    for signal_index in 0..signal_len {
        let mut convolution = (0.0, 0.0);
        for wavelet_index in 0..wavelet_len {
            //todo: handle boundary conditions better
            //todo: most likely it shouldn't be full convolution (so convolution duration = signal duration - wavelet duration + 1 sample
            let signal_at = if signal_index + wavelet_index >= signal_len { 0.0 } else { signal_samples[signal_index + wavelet_index] };
            convolution = complex_sum(convolution, scalar_complex_mul(signal_at, wavelet_samples[wavelet_index]));
        }
        convolution_result.push(convolution);
    }
    return convolution_result;
}

#[cfg(test)]
mod tests {
    use crate::signals::SignalSample;
    use crate::wavelets::transform::{complex_convolution, wavelet_transform};

    #[test]
    fn test_convolution_real_part() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];
        let wavelet = vec![(1.0, 0.0), (-2.0, 0.0), (0.5, 0.0)];

        let vec1 = complex_convolution(&signal, &wavelet);
        assert_eq!(vec1, vec![(0.3 - 1.0 - 0.5, 0.0), (0.5 + 2.0 + 0.35, 0.0), (-1.0 - 1.4, 0.0), (0.7, 0.0)]);
    }

    #[test]
    fn test_convolution_img_part() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];
        let wavelet = vec![(0.0, 1.0), (0.0, -2.0), (0.0, 0.5)];

        let vec1 = complex_convolution(&signal, &wavelet);
        assert_eq!(vec1, vec![(0.0, 0.3 - 1.0 - 0.5), (0.0, 0.5 + 2.0 + 0.35), (0.0, -1.0 - 1.4), (0.0, 0.7)]);
    }

    #[test]
    fn test_convolution_both_parts() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];
        let wavelet = vec![(0.4, 1.0), (0.6, -2.0), (-0.2, 0.5)];

        let vec1 = complex_convolution(&signal, &wavelet);
        assert_eq!(vec1, vec![(0.12 + 0.3 + 0.2, 0.3 - 1.0 - 0.5), (-0.5399999999999999, 0.5 + 2.0 + 0.35), (-0.4 + 0.42, -1.0 - 1.4), (0.27999999999999997, 0.7)]);
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
        assert_eq!(transform[0], vec![(0.12 + 0.3 + 0.2, 0.3 - 1.0 - 0.5), (-0.5399999999999999, 0.5 + 2.0 + 0.35), (-0.4 + 0.42, -1.0 - 1.4), (0.27999999999999997, 0.7)]);
    }
}