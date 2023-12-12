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
        let wavelet = wavelets::morlet(frequency);
        signals::SignalSample::from_wavelet(2.0 * MORLET_HALF_LENGTH / frequency, sample_rate, &|x| wavelet(x))
    }, 1.0, 33.0, 16);

    println!("{:?}", result[15]);
    println!("Elapsed {:?}", time.elapsed());

//    save_wav("wavelet440hz3s.wav", &sample);
}