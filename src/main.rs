use std::time::Instant;
use crate::signals::{SignalSample, sine_signal};
use crate::math::{ComplexNum, modulo};
use crate::wavelets::MORLET_HALF_LENGTH;

mod wavelets;
mod signals;
mod utils;
mod math;

fn main() {
    let time = Instant::now();
    let sample_rate = 2048;
    let signal_duration_s = 1;
    let sine_samples =
        signals::SignalSample::from_signal(signal_duration_s as f64, sample_rate, &|t| sine_signal(t, 200.0));

    let frequencies = sample_rate;
    let result = wavelets::transform::wavelet_transform(&sine_samples, &|frequency, sample_rate| {
        let wavelet = wavelets::morlet(frequency);
        signals::SignalSample::from_wavelet(2.0 * MORLET_HALF_LENGTH / frequency, sample_rate, &|x| wavelet(x))
    }, 1.0, 500.0, frequencies);

    println!("Elapsed {:?}", time.elapsed());

//    save_wav("wavelet440hz3s.wav", &sample);
}