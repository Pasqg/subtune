use std::str::FromStr;
use image::{ImageFormat, save_buffer_with_format};
use num_complex::ComplexFloat;
use num_complex::Complex;
use crate::utils::math::FloatType;

pub(crate) enum ResamplingStrategy {
    Map,
    Avg,
}

impl ResamplingStrategy {
    pub fn sample(&self, previous: FloatType, value: Complex<FloatType>, partition_size: usize) -> FloatType {
        match self {
            ResamplingStrategy::Map => value.abs().max(previous),
            ResamplingStrategy::Avg => previous + value.abs() / (partition_size as FloatType),
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
    pub fn color(&self, value: FloatType) -> (u8, u8, u8) {
        match self {
            ColorScheme::HeatMap => Self::hsl_to_rgb((1.0 - value) * 240.0, 1.0, 0.5),
            ColorScheme::Grayscale => {
                let b = (value * 255.0).round() as u8;
                (b, b, b)
            }
        }
    }

    fn hsl_to_rgb(h: FloatType, s: FloatType, l: FloatType) -> (u8, u8, u8) {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let h_prime = h / 60.0;
        let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
        let (r1, g1, b1) = if h_prime < 0.0 {
            (0.0, 0.0, 0.0)
        } else if (0.0..=1.0).contains(&h_prime) {
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
        (((r1 + m) * 255.0).round() as u8,
         ((g1 + m) * 255.0).round() as u8,
         ((b1 + m) * 255.0).round() as u8)
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
    pub frequencies: Vec<FloatType>,
    pub sample_rate: u32,
    pub resampling_strategy: ResamplingStrategy,
    pub color_scheme: ColorScheme,
    pub pixels_per_second: u32,
    pub pixels_per_frequency: u32,
    pub add_piano_roll: bool,
    pub image_format: ImageFormat,
}

pub(crate) fn output_image(wavelet_transform: &[Vec<Complex<FloatType>>],
                           visualization_parameters: &VisualizationParameters) {
    let (image_data, width, height) =
        transform_to_image(wavelet_transform, visualization_parameters);

    save_buffer_with_format(visualization_parameters.file_name.as_str(),
                            &image_data,
                            width as u32,
                            height as u32,
                            image::ColorType::Rgb8,
                            visualization_parameters.image_format).unwrap();
}

fn transform_to_image(transform: &[Vec<Complex<FloatType>>],
                      visualization_parameters: &VisualizationParameters) -> (Vec<u8>, usize, usize) {
    let piano_roll_length = if visualization_parameters.add_piano_roll {
        24.max(visualization_parameters.pixels_per_second / 2).min(128) as usize
    } else { 0 };
    let chunk_size = (visualization_parameters.sample_rate / visualization_parameters.pixels_per_second) as usize;
    let new_width = piano_roll_length + transform[0].len() / chunk_size;
    let new_height = transform.len() * visualization_parameters.pixels_per_frequency as usize;

    let mut sampled = Vec::with_capacity(new_height);
    for vec in transform {
        let mut row = Vec::with_capacity(new_width - piano_roll_length);
        for chunk_index in 0..(new_width - piano_roll_length) {
            let chunk_offset = chunk_index * chunk_size;
            let mut value = 0.0;
            for k in 0..chunk_size {
                value = visualization_parameters.resampling_strategy.sample(value, vec[chunk_offset + k], chunk_size);
            }
            row.push(value);
        }
        for _ in 0..visualization_parameters.pixels_per_frequency {
            sampled.push(row.clone());
        }
    }

    let frequencies = &visualization_parameters.frequencies;
    let max = find_max(&sampled, &std::convert::identity);
    let mut resized_data = Vec::with_capacity(new_height * new_width * 3);
    for i in 0..new_height {
        (0..piano_roll_length).for_each(|k| {
            let frequency = frequencies[frequencies.len() - 1 - (i / (visualization_parameters.pixels_per_frequency as usize))];
            let note = ((12.0 * (frequency / 16.35).log2()) % 12.0).round() as i32;
            if k < (piano_roll_length as f32 * 0.8) as usize && (note == 1 || note == 3 || note == 6 || note == 8 || note == 10) {
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
            let (r, g, b) = visualization_parameters.color_scheme.color(sampled[i][k] / max);
            resized_data.push(r);
            resized_data.push(g);
            resized_data.push(b);
        }
    }
    (resized_data, new_width, new_height)
}

fn find_max<T: Copy>(result: &Vec<Vec<T>>, transform_fn: &impl Fn(T) -> FloatType) -> FloatType {
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