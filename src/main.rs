use crate::utils::save_wav;

mod wavelets;
mod signals;
mod utils;

fn main() {
    let sample = signals::SignalSample::from_function(5.0, 96000,
                                                      |t| signals::sine_signal(t, 440.0));

    save_wav("sine440hz.wav", &sample);
}