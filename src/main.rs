use std::time::Instant;
use clap::Parser;
use crate::utils::read_wav;
use crate::visualization::output_image;
use crate::wavelets::MORLET_HALF_LENGTH;

mod wavelets;
mod signals;
mod utils;
mod math;
mod visualization;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input wav file path
    #[arg(short, long)]
    input: String,

    /// Output image file (png) path
    #[arg(short, long)]
    output: String,

    /// Number of octaves to analyze, default 10
    #[arg(short, long)]
    num_octaves: Option<u32>,

    /// Index of first octave (default 0 = C0-B0). Negative allowed.
    #[arg(short, long)]
    start_octave: Option<i32>,

    /// If this flag is present, opens a window to show the resulting image
    #[arg(short, long, default_missing_value = "true")]
    display: bool,
}

const CO: f64 = 16.35;

#[show_image::main]
fn main() {
    let time = Instant::now();
    let cli = Cli::parse();
    let signal = read_wav(cli.input.as_str());

    let first_octave = cli.start_octave.unwrap_or(0);
    let frequencies = (12 * first_octave..(12 * (first_octave + cli.num_octaves.unwrap_or(10) as i32)))
        .into_iter().map(|i| CO * (i as f64 / 12.0).exp2()).collect();

    let transform = wavelets::transform::wavelet_transform(&signal, &|frequency, sample_rate| {
        let wavelet = wavelets::morlet(frequency);
        signals::SignalSample::from_wavelet(2.0 * MORLET_HALF_LENGTH / frequency, sample_rate, &|x| wavelet(x))
    }, &frequencies);

    println!("Done in {:?}", time.elapsed());

    output_image(cli.output.as_str(), frequencies.len() as u32,
                 signal.sample_rate, signal.samples.len() as u32,
                 &transform, cli.display);
}