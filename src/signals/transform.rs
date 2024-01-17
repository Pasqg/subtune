use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use crate::signals::SignalSample;
use crate::utils::math::{complex_sum, ComplexNum, scalar_complex_mul};

/// wavelet_factory: from (frequency, sample rate) to a SignalSample lasting 1/frequency
pub(crate) fn wavelet_transform(signal: &SignalSample<f64>,
                                wavelet_factory: &(impl Fn(f64, u32) -> SignalSample<ComplexNum> + Sync),
                                frequencies: &Vec<f64>) -> Vec<Vec<ComplexNum>> {
    let sample_rate = signal.sample_rate;
    let signal = &signal.samples;
    let indexed_frequencies: Vec<(usize, f64)> = (0..frequencies.len()).into_iter()
        .map(|index| (index, frequencies[frequencies.len() - index - 1]))
        .collect();
    let planner = Arc::new(Mutex::new(FftPlanner::<f64>::new()));
    let indexed_result: Vec<(usize, Vec<ComplexNum>)> = indexed_frequencies.par_iter()
        .map(|(index, frequency_hz)| {
            let wavelet = wavelet_factory(*frequency_hz, sample_rate);
            let convolution =
                if should_use_fourier(signal.len() as u32, wavelet.samples.len() as u32) {
                    fourier_convolution(&signal, &wavelet.samples, &planner)
                } else {
                    complex_convolution(&signal, &wavelet.samples)
                };
            let convolution: Vec<ComplexNum> = convolution.iter().map(|c| scalar_complex_mul(1.0 / (wavelet.samples.len() as f64), *c)).collect();
            (*index, convolution[(wavelet.samples.len() - 1)..(signal.len() + wavelet.samples.len() - 1)].to_vec())
        })
        .collect();
    let mut transform = vec![Vec::new(); frequencies.len()];
    for result in indexed_result {
        transform[result.0] = result.1;
    }
    return transform;
}

fn should_use_fourier(signal_len: u32, wavelet_len: u32) -> bool {
    let convolution_len = round_to_power_2((signal_len + wavelet_len - 1) as i64) as u32;
    let fourier_complexity = convolution_len.ilog2() * convolution_len;
    let convolution_complexity = signal_len * wavelet_len;
    return fourier_complexity <= convolution_complexity;
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

fn fourier_convolution(signal: &Vec<f64>, kernel: &Vec<ComplexNum>, planner: &Arc<Mutex<FftPlanner<f64>>>) -> Vec<ComplexNum> {
    let signal_len = signal.len();
    let kernel_len = kernel.len();
    let convolution_len = signal_len + kernel_len - 1;
    let convolution_len = round_to_power_2(convolution_len as i64) as usize;

    let signal = pad(signal, convolution_len, 0.0);
    let kernel = pad(kernel, convolution_len, (0.0, 0.0));

    let mut signal_transform = vec![Complex { re: 0.0, im: 0.0 }; convolution_len];
    for i in 0..signal_len {
        signal_transform[i] = Complex { re: signal[i], im: 0.0 };
    }
    let mut kernel_transform = vec![Complex { re: 0.0, im: 0.0 }; convolution_len];
    for i in 0..kernel_len {
        kernel_transform[i] = Complex { re: kernel[i].0, im: kernel[i].1 };
    }

    let fft = planner.lock().unwrap().plan_fft_forward(convolution_len);
    fft.process(&mut signal_transform);

    let fft = planner.lock().unwrap().plan_fft_forward(convolution_len);
    fft.process(&mut kernel_transform);

    for i in 0..convolution_len {
        signal_transform[i] *= kernel_transform[i];
    }

    let fft = planner.lock().unwrap().plan_fft_inverse(convolution_len);
    fft.process(&mut signal_transform);

    return signal_transform.iter().map(|c| (c.re / (convolution_len as f64), c.im / (convolution_len as f64))).collect();
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
    use std::sync::{Arc, Mutex};
    use rustfft::FftPlanner;
    use crate::utils::math::assert_complex_vec;
    use crate::signals::SignalSample;
    use crate::signals::transform::{complex_convolution, fourier_convolution, pad, round_to_power_2, wavelet_transform};

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
        assert_complex_vec(transform[0].to_vec(), vec![(-0.16 / 3.0, -1.85 / 3.0), (-0.42 / 3.0, 2.95 / 3.0),
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

        let planner = Arc::new(Mutex::new(FftPlanner::<f64>::new()));
        let fourier_convolution = fourier_convolution(&signal, &wavelet, &planner);

        let convolution = complex_convolution(&signal, &wavelet);
        let convolution = pad(&convolution, 8, (0.0, 0.0));

        assert_complex_vec(convolution, fourier_convolution);
    }
}