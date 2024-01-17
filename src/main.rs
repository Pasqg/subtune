use std::time::Instant;
use clap::Parser;
use show_image::exit;
use signals::wavelets;
use crate::utils::argument_validation::validate_arguments;
use crate::utils::read_wav;
use crate::utils::visualization::{open_window, output_image};
use crate::signals::wavelets::MORLET_HALF_LENGTH;
use crate::signals::transform::wavelet_transform;

mod signals;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input wav file path
    #[arg(short, long)]
    input: String,

    /// Output image file (png) path
    #[arg(short, long)]
    output: Option<String>,

    /// Number of octaves to analyze, default 9
    #[arg(short, long)]
    num_octaves: Option<u32>,

    /// Index of first octave (default 1 = C1-B1). Negative allowed.
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

    let input_file = cli.input.as_str();
    let output_file_from_input = default_output_file(input_file);
    let output_file = cli.output.unwrap_or(output_file_from_input);
    let output_file = output_file.as_str();

    let validation_result = validate_arguments(input_file, output_file);
    if validation_result.is_err() {
        eprintln!("{}", validation_result.err().unwrap());
        exit(1);
    }

    println!("Will save result to {}", output_file);

    let signal = read_wav(input_file);

    let first_octave = cli.start_octave.unwrap_or(1);
    let octaves = cli.num_octaves.unwrap_or(9) as i32;
    let frequencies = (12 * first_octave..(12 * (first_octave + octaves)))
        .into_iter().map(|i| CO * (i as f64 / 12.0).exp2()).collect();

    let transform = wavelet_transform(&signal, &|frequency, sample_rate| {
        let wavelet = wavelets::morlet(frequency);
        signals::SignalSample::from_wavelet(2.0 * MORLET_HALF_LENGTH / frequency, sample_rate, &|x| wavelet(x))
    }, &frequencies);

    let image_data = output_image(output_file, frequencies.len() as u32,
                                  signal.sample_rate, signal.samples.len() as u32,
                                  &transform);

    println!("Done in {:?}", time.elapsed());

    if cli.display {
        open_window(signal.samples.len() as u32, frequencies.len() as u32, &image_data);
    }
}

fn default_output_file(input_file: &str) -> String {
    let split: Vec<&str> = input_file.split_inclusive('.').collect();
    if split.len() > 1 {
        return split[0..(split.len() - 1)].concat() + "png";
    }
    return input_file.to_string();
}