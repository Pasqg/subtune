use crate::utils::save_wav;

mod wavelets;
mod signals;
mod utils;

fn main() {
    let wavelet = wavelets::morlet(3.0, 440.0);
    let sample =
        signals::SignalSample::from_function(3.0, 96000, &|x| wavelet(x).0);

    save_wav("wavelet440hz3s.wav", &sample);
}