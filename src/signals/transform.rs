use std::sync::Arc;
use rayon::prelude::*;
use rustfft::{Fft, FftPlanner};
use rustfft::num_complex::Complex;
use crate::signals::SignalSample;
use crate::signals::wavelets::MORLET_HALF_LENGTH;
use crate::utils::math::re;

/// wavelet_factory: from (frequency, sample rate) to a SignalSample lasting 1/frequency
pub(crate) fn wavelet_transform(signal: &SignalSample<f64>,
                                wavelet_factory: &(impl Fn(f64, u32) -> SignalSample<Complex<f64>> + Sync),
                                frequencies: &[f64],
                                n_threads: u32) -> Vec<Vec<Complex<f64>>> {
    let sample_rate = signal.sample_rate;
    let signal = &signal.samples;
    let frequencies_num = frequencies.len();

    let max_wavelet_samples = (2.0 * MORLET_HALF_LENGTH * (sample_rate as f64) / frequencies[0]).ceil() as usize;
    let max_convolution_len = signal.len() + max_wavelet_samples - 1;

    let signal_fourier = in_place_fourier(signal, max_convolution_len);
    let results: Vec<(usize, f64)> = (0..frequencies.len())
        .map(|index| (index, frequencies[frequencies.len() - index - 1]))
        .collect();

    let mut planner = FftPlanner::<f64>::new();
    let forward_fft = planner.plan_fft_forward(signal_fourier.len());
    let inverse_fft = planner.plan_fft_inverse(signal_fourier.len());
    let results: Vec<(usize, Vec<Complex<f64>>)> = results
        .par_rchunks((frequencies_num as f64 / n_threads as f64).ceil() as usize)
        .flat_map(|elements| {
            elements.iter().map(|(index, frequency_hz)| {
                let wavelet = wavelet_factory(*frequency_hz, sample_rate);
                let convolution =
                    fourier_convolution(&signal_fourier, &wavelet.samples, &forward_fft, &inverse_fft);
                //todo: merge these two
                let convolution: Vec<Complex<f64>> = convolution.iter().map(|c| *c / (wavelet.samples.len() as f64)).collect();
                (*index, convolution[(wavelet.samples.len() - 1)..(signal.len() + wavelet.samples.len() - 1)].to_vec())
            }).collect::<Vec<(usize, Vec<Complex<f64>>)>>()
        })
        .collect();
    let mut transform = vec![Vec::new(); frequencies_num];
    for result in results {
        transform[result.0] = result.1;
    }
    transform
}

fn round_to_power_2(n: i64) -> i64 {
    let power = n.ilog2();
    let smaller = 2i64.pow(power);
    if n == smaller {
        return smaller;
    }
    smaller * 2
}

fn fourier_convolution(signal_fourier: &[Complex<f64>],
                       kernel: &[Complex<f64>],
                       forward_fft: &Arc<dyn Fft<f64>>,
                       inverse_fft: &Arc<dyn Fft<f64>>) -> Vec<Complex<f64>> {
    let convolution_len = signal_fourier.len();

    let mut kernel_transform = pad(kernel, convolution_len, re(0.0));

    forward_fft.process(&mut kernel_transform);

    let mut signal_transform = vec![re(0.0); convolution_len];
    let signal_transform_slice = &mut signal_transform;
    for i in 0..convolution_len {
        signal_transform_slice[i] = signal_fourier[i] * kernel_transform[i] / (convolution_len as f64);
    }

    inverse_fft.process(signal_transform_slice);

    signal_transform
}

fn in_place_fourier(signal: &[f64], length: usize) -> Vec<Complex<f64>> {
    let signal_len = signal.len();
    let convolution_len: usize = round_to_power_2(length as i64) as usize;

    let mut signal_transform = vec![re(0.0); convolution_len];
    for i in 0..signal_len {
        signal_transform[i] = re(signal[i]);
    }

    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(convolution_len);
    fft.process(&mut signal_transform);

    signal_transform
}

fn pad<T: Copy>(vector: &[T], new_length: usize, default: T) -> Vec<T> {
    let mut padded_kernel = vec![default; new_length];
    padded_kernel[..vector.len()].copy_from_slice(vector);
    padded_kernel
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;
    use rustfft::FftPlanner;
    use crate::utils::math::{assert_complex_vec, i, re};
    use crate::signals::SignalSample;
    use crate::signals::transform::{fourier_convolution, in_place_fourier, pad, round_to_power_2, wavelet_transform};

    #[test]
    fn test_convolution_real_part() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];
        let wavelet = vec![re(1.0), -re(2.0), re(0.5)];

        let convolution = complex_convolution(&signal, &wavelet);
        assert_complex_vec(&convolution, &vec![re(0.3), -re(0.1), -re(1.85), re(2.95), -re(1.9), re(0.35)]);
    }

    #[test]
    fn test_convolution_img_part() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];
        let wavelet = vec![i(1.0), -i(2.0), i(0.5)];

        let convolution = complex_convolution(&signal, &wavelet);
        assert_complex_vec(&convolution, &vec![i(0.3), -i(0.1), -i(1.85), i(2.95), -i(1.9), i(0.35)]);
    }

    #[test]
    fn test_convolution_both_parts() {
        let signal = vec![0.3, 0.5, -1.0, 0.7];
        let wavelet = vec![0.4 + i(1.0), 0.6 + i(-2.0), -0.2 + i(0.5)];

        let convolution = complex_convolution(&signal, &wavelet);
        assert_complex_vec(&convolution, &vec![0.12 + i(0.3), 0.38 - i(0.1), -0.16 - i(1.85),
                                               -0.42 + i(2.95), 0.62 - i(1.9), -0.14 + i(0.35)]);
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
                                          &|_, _| SignalSample {
                                              sample_rate: 3,
                                              samples: vec![0.4 + i(1.0), 0.6 - i(2.0), -0.2 + i(0.5)],
                                          },
                                          &frequencies, 1);

        assert_complex_vec(&transform[0].to_vec(), &vec![(-0.16 - i(1.85)) / 3.0, (-0.42 + i(2.95)) / 3.0,
                                                         (0.62 - i(1.9)) / 3.0, (-0.14 + i(0.35)) / 3.0]);
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
        let wavelet = vec![re(0.2), re(-0.7), re(0.4)];

        let signal_fourier = in_place_fourier(&signal, 8);

        let mut planner = FftPlanner::<f64>::new();
        let fourier_convolution = fourier_convolution(&signal_fourier, &wavelet,
                                                      &planner.plan_fft_forward(8), &planner.plan_fft_inverse(8));

        let convolution = complex_convolution(&signal, &wavelet);
        let convolution = pad(&convolution, 8, re(0.0));

        assert_complex_vec(&convolution, &fourier_convolution);
    }

    fn complex_convolution(signal: &[f64], kernel: &[Complex<f64>]) -> Vec<Complex<f64>> {
        let signal_len = signal.len() as i64;
        let kernel_len = kernel.len() as i64;

        let mut convolution_result = Vec::with_capacity(signal_len as usize);
        for signal_i in -(kernel_len - 1)..signal_len {
            let mut convolution = re(0.0);
            for kernel_i in 0..kernel_len {
                if signal_i + kernel_i >= 0 && signal_i + kernel_i < signal_len {
                    let signal_at = signal[(signal_i + kernel_i) as usize];
                    let kernel_at = kernel[(kernel_len - kernel_i - 1) as usize];
                    convolution += signal_at * kernel_at;
                }
            }
            convolution_result.push(convolution);
        }
        convolution_result
    }
}