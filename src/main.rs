use std::time::Instant;
use crate::signals::sine_signal;

mod wavelets;
mod signals;
mod utils;
mod math;

fn main() {
    let time = Instant::now();
    let sample_rate = 512;
    let sine_samples =
        signals::SignalSample::from_signal(3.0, sample_rate, &|t| sine_signal(t, 17.0));

    let result = wavelets::transform::wavelet_transform(&sine_samples, &|frequency, sample_rate| {
        let duration_s = 1.0 / frequency;
        let wavelet = wavelets::morlet(duration_s, frequency);
        signals::SignalSample::from_wavelet(duration_s, sample_rate, &|x| wavelet(x))
    }, 1.0, 33.0, 16);

    println!("{:?}", result[15]);
    println!("Elapsed {:?}", time.elapsed());

//    save_wav("wavelet440hz3s.wav", &sample);
}