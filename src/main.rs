use std::time::Instant;
use crate::utils::read_wav;
use crate::visualization::save_image;
use crate::wavelets::MORLET_HALF_LENGTH;

mod wavelets;
mod signals;
mod utils;
mod math;
mod visualization;
mod utils;

fn main() {
    let time = Instant::now();
    let base_frequency = 61.74;

    let sine_samples = read_wav("guitar.wav");

    let frequencies = (0..60).into_iter().map(|i| base_frequency * (i as f64 / 12.0).exp2()).collect();
    let transform = wavelets::transform::wavelet_transform(&sine_samples, &|frequency, sample_rate| {
        let wavelet = wavelets::morlet(frequency);
        signals::SignalSample::from_wavelet(2.0 * MORLET_HALF_LENGTH / frequency, sample_rate, &|x| wavelet(x))
    }, &frequencies);

    println!("Elapsed {:?}", time.elapsed());

    save_image("guitar.png", frequencies.len() as u32,
               sine_samples.sample_rate, sine_samples.samples.len() as u32, &transform);
}