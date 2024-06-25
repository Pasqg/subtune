use std::str::FromStr;
use image::{ImageFormat, save_buffer_with_format};
use num_complex::ComplexFloat;
use show_image::{create_window, ImageInfo, ImageView};
use num_complex::Complex;

pub(crate) enum ResamplingStrategy {
    Map,
    Avg,
}

impl ResamplingStrategy {
    pub fn sample(&self, previous: f64, value: Complex<f64>, partition_size: usize) -> f64 {
        match self {
            ResamplingStrategy::Map => value.abs().max(previous),
            ResamplingStrategy::Avg => previous + value.abs() / (partition_size as f64),
        }
    }
}

impl FromStr for ResamplingStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "max" => Ok(ResamplingStrategy::Map),
            "avg" => Ok(ResamplingStrategy::Avg),
            _ => Err(format!("Invalid resampling strategy '{}'", s).to_string()),
        }
    }
}

pub(crate) enum ColorScheme {
    HeatMap,
    Grayscale,
}

impl ColorScheme {
    pub fn color(&self, value: f64) -> (u8, u8, u8) {
        match self {
            ColorScheme::HeatMap => Self::hsl_to_rgb((1.0 - value) * 240.0, 1.0, 0.5),
            ColorScheme::Grayscale => {
                let b = (value * 255.0).round() as u8;
                (b, b, b)
            }
        }
    }

    fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let h_prime = h / 60.0;
        let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
        let (r1, g1, b1) = if h_prime < 0.0 {
            (0.0, 0.0, 0.0)
        } else if h_prime >= 0.0 && h_prime <= 1.0 {
            (c, x, 0.0)
        } else if h_prime <= 2.0 {
            (x, c, 0.0)
        } else if h_prime <= 3.0 {
            (0.0, c, x)
        } else if h_prime <= 4.0 {
            (0.0, x, c)
        } else if h_prime <= 5.0 {
            (x, 0.0, c)
        } else if h_prime <= 6.0 {
            (c, 0.0, x)
        } else {
            (0.0, 0.0, 0.0)
        };

        let m = l - c / 2.0;
        return (
            ((r1 + m) * 255.0).round() as u8,
            ((g1 + m) * 255.0).round() as u8,
            ((b1 + m) * 255.0).round() as u8,
        );
    }
}

impl FromStr for ColorScheme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "heatmap" => Ok(ColorScheme::HeatMap),
            "grayscale" => Ok(ColorScheme::Grayscale),
            _ => Err(format!("Invalid color scheme '{}'", s).to_string()),
        }
    }
}

pub(crate) struct VisualizationParameters {
    pub file_name: String,
    pub frequencies: Vec<f64>,
    pub sample_rate: u32,
    pub resampling_strategy: ResamplingStrategy,
    pub color_scheme: ColorScheme,
    pub pixels_per_second: u32,
    pub pixels_per_frequency: u32,
    pub add_piano_roll: bool,
    pub image_format: ImageFormat,
}

pub(crate) fn open_window(width: u32, heigth: u32, image_data: &Vec<u8>) {
    let image = ImageView::new(ImageInfo::rgb8(width, heigth), &image_data);
    let window = create_window("Wavelet transform", Default::default()).unwrap();
    window.set_image("image", image).unwrap();

    loop {};
}

pub(crate) fn output_image(wavelet_transform: &Vec<Vec<Complex<f64>>>,
                           visualization_parameters: &VisualizationParameters) -> (Vec<u8>, usize, usize) {
    let (image_data, width, height) =
        transform_to_image(&wavelet_transform,
                           &visualization_parameters.resampling_strategy,
                           &visualization_parameters.color_scheme,
                           &visualization_parameters.frequencies,
                           visualization_parameters.sample_rate,
                           visualization_parameters.pixels_per_frequency,
                           visualization_parameters.pixels_per_second,
                           visualization_parameters.add_piano_roll);

    save_buffer_with_format(visualization_parameters.file_name.as_str(),
                            &image_data,
                            width as u32,
                            height as u32,
                            image::ColorType::Rgb8,
                            visualization_parameters.image_format).unwrap();

    return (image_data, width, height);
}

fn transform_to_image(transform: &Vec<Vec<Complex<f64>>>,
                      resampling_strategy: &ResamplingStrategy,
                      color_scheme: &ColorScheme,
                      frequencies: &Vec<f64>,
                      sample_rate: u32,
                      pixels_per_frequency: u32,
                      pixels_per_second: u32,
                      add_piano_roll: bool) -> (Vec<u8>, usize, usize) {
    let piano_roll_length = if add_piano_roll { 24 } else { 0 };
    let chunk_size = (sample_rate / pixels_per_second) as usize;
    let new_width = piano_roll_length + transform[0].len() / chunk_size;
    let new_height = transform.len() * pixels_per_frequency as usize;

    let mut sampled = Vec::with_capacity(new_height);
    for i in 0..transform.len() {
        let mut row = Vec::with_capacity(new_width - piano_roll_length);
        for chunk_index in 0..(new_width - piano_roll_length) {
            let chunk_offset = chunk_index * chunk_size;
            let mut value = 0.0;
            for k in 0..chunk_size {
                value = resampling_strategy.sample(value, transform[i][chunk_offset + k], chunk_size);
            }
            row.push(value);
        }
        for _ in 0..pixels_per_frequency {
            sampled.push(row.clone());
        }
    }

    let max = find_max(&sampled, &std::convert::identity);
    let mut resized_data = Vec::with_capacity(new_height * new_width * 3);
    for i in 0..new_height {
        (0..piano_roll_length).into_iter().for_each(|_| {
            let frequency = frequencies[frequencies.len() - 1 - (i / (pixels_per_frequency as usize))];
            let note = ((12.0 * (frequency / 16.35).log2()) % 12.0).round() as i32;
            if note == 1 || note == 3 || note == 6 || note == 8 || note == 10 {
                resized_data.push(0);
                resized_data.push(0);
                resized_data.push(0);
            } else {
                resized_data.push(255);
                resized_data.push(255);
                resized_data.push(255);
            }
        });
        for k in 0..(new_width - piano_roll_length) {
            let (r, g, b) = color_scheme.color(sampled[i][k] / max);
            resized_data.push(r);
            resized_data.push(g);
            resized_data.push(b);
        }
    }
    (resized_data, new_width, new_height)
}

fn find_max<T: Copy>(result: &Vec<Vec<T>>, transform_fn: &impl Fn(T) -> f64) -> f64 {
    let mut max = 0.0;
    for row in result {
        for value in row {
            let value = transform_fn(*value);
            if value > max {
                max = value;
            }
        }
    }
    max
}