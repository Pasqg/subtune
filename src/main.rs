use std::time::Instant;
use crate::signals::sine_signal;
use crate::visualization::open_window;
use crate::wavelets::MORLET_HALF_LENGTH;

mod wavelets;
mod signals;
mod utils;
mod math;
mod visualization;

#[show_image::main]
fn main() {
    let time = Instant::now();
    let sample_rate = 2048;
    let signal_duration_s = 1;
    let base_frequency = 110.0;
    let sine_samples =
        signals::SignalSample::from_signal(signal_duration_s as f64, sample_rate,
                                           &|t| 0.7 * sine_signal(t, base_frequency)
                                               + 0.35 * sine_signal(t, 2.0 * base_frequency)
                                               + 0.2 * sine_signal(t, 3.0 * base_frequency)
                                               + 0.15 * sine_signal(t, 4.0 * base_frequency)
                                               + 0.1 * sine_signal(t, 9.0 * base_frequency)
        );

    let frequencies = sample_rate;
    let result = wavelets::transform::wavelet_transform(&sine_samples, &|frequency, sample_rate| {
        let wavelet = wavelets::morlet(frequency);
        signals::SignalSample::from_wavelet(2.0 * MORLET_HALF_LENGTH / frequency, sample_rate, &|x| wavelet(x))
    }, 50.0, 1000.0, frequencies);

    println!("Elapsed {:?}", time.elapsed());

    open_window(&sine_samples, &result);
}