use std::str::FromStr;
use std::time::Instant;
use clap::Parser;
use image::ImageFormat;
use show_image::exit;
use signals::wavelets;
use crate::notes::C0;
use crate::utils::argument_validation::validate_arguments;
use crate::utils::read_wav;
use crate::utils::visualization::{ColorScheme, open_window, output_image, ResamplingStrategy, VisualizationParameters};
use crate::signals::wavelets::MORLET_HALF_LENGTH;
use crate::signals::transform::wavelet_transform;

mod notes;
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

    /// Resampling strategy [max, avg] (default max)
    #[arg(short, long)]
    resampling_strategy: Option<String>,

    /// Color scheme [heatmap, grayscale] (default heatmap)
    #[arg(short, long)]
    color_scheme: Option<String>,

    /// Pixels per second on the horizontal axis of the resulting image (default 32)
    #[arg(long)]
    pixels_per_second: Option<u32>,

    /// Pixels per frequency on the vertical axis of the resulting image (default 6)
    #[arg(long)]
    pixels_per_frequency: Option<u32>,

    /// Frequencies per note/pitch, evenly spaced in exponential space (default 1)
    #[arg(short, long)]
    frequencies_per_note: Option<u32>,

    /// Number of threads to use when calculating the wavelet transform (default 16)
    #[arg(short, long)]
    threads: Option<u32>,

    /// If this flag is present, adds a simple piano roll in the resulting image
    #[arg(short, long, default_missing_value = "true")]
    piano_roll: bool,

    /// If this flag is present, opens a window to show the resulting image
    #[arg(short, long, default_missing_value = "true")]
    display: bool,
}

#[show_image::main]
fn main() {
    let time = Instant::now();
    let cli = Cli::parse();

    let input_file = cli.input.as_str();
    let output_file_from_input = default_output_file(input_file);
    let output_file = cli.output.unwrap_or(output_file_from_input);

    let resampling_strategy = cli.resampling_strategy.unwrap_or("max".to_string());
    let resampling_strategy = resampling_strategy.as_str();
    let color_scheme = cli.color_scheme.unwrap_or("heatmap".to_string());
    let color_scheme = color_scheme.as_str();

    validate(input_file, &output_file, resampling_strategy, color_scheme);

    let signal = read_wav(input_file);

    let first_octave = cli.start_octave.unwrap_or(1);
    let octaves = cli.num_octaves.unwrap_or(9) as i32;

    let frequencies_per_note = cli.frequencies_per_note.unwrap_or(1) as i32;
    let frequencies: Vec<f64> = (12 * frequencies_per_note * first_octave..(12 * frequencies_per_note * (first_octave + octaves) + 12))
        .map(|i| C0 * (i as f64 / 12.0 / frequencies_per_note as f64).exp2())
        .collect();

    println!("Transforming {} samples, for {} frequencies. Will save result to {}", signal.samples.len(), frequencies.len(), output_file.as_str());

    let transform = wavelet_transform(&signal, &|frequency, sample_rate| {
        let wavelet = wavelets::morlet(frequency);
        signals::SignalSample::from_wavelet(2.0 * MORLET_HALF_LENGTH / frequency, sample_rate, &wavelet)
    }, &frequencies, cli.threads.unwrap_or(16));

    let parameters = VisualizationParameters {
        file_name: output_file,
        frequencies,
        sample_rate: signal.sample_rate,
        resampling_strategy: ResamplingStrategy::from_str(resampling_strategy).unwrap(),
        color_scheme: ColorScheme::from_str(color_scheme).unwrap(),
        pixels_per_second: cli.pixels_per_second.unwrap_or(32),
        pixels_per_frequency: cli.pixels_per_frequency.unwrap_or(6),
        add_piano_roll: cli.piano_roll,
        image_format: ImageFormat::Png,
    };
    let (image_data, width, height) = output_image(&transform, &parameters);

    println!("Done in {:?}", time.elapsed());

    if cli.display {
        open_window(width as u32, height as u32, &image_data);
    }
}

fn validate(input_file: &str, output_file: &str, resampling_strategy: &str, color_scheme: &str) {
    let validation_result =
        validate_arguments(input_file,
                           output_file,
                           resampling_strategy,
                           color_scheme);
    if validation_result.is_err() {
        eprintln!("{}", validation_result.err().unwrap());
        exit(1);
    }
}

fn default_output_file(input_file: &str) -> String {
    let split: Vec<&str> = input_file.split_inclusive('.').collect();
    if split.len() > 1 {
        return split[0..(split.len() - 1)].concat() + "png";
    }
    input_file.to_string()
}