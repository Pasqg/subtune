use image::{ImageFormat, save_buffer_with_format};
use num_complex::ComplexFloat;
use show_image::{create_window, ImageInfo, ImageView};
use num_complex::Complex;

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

fn heat_map_color(value: f64) -> (u8, u8, u8) {
    hsl_to_rgb((1.0 - value) * 240.0, 1.0, 0.5)
}

fn grayscale(value: f64) -> (u8, u8, u8) {
    let b = (value * 255.0).round() as u8;
    (b, b, b)
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

pub(crate) fn open_window(width: u32, heigth: u32, image_data: &Vec<u8>) {
    let image = ImageView::new(ImageInfo::rgb8(width, heigth), &image_data);
    let window = create_window("Wavelet transform", Default::default()).unwrap();
    window.set_image("image", image).unwrap();

    loop {};
}

pub(crate) fn output_image(file_name: &str, frequencies: &Vec<f64>,
                           pixels_per_frequency: u32, sample_rate: u32, samples: u32,
                           wavelet_transform: &Vec<Vec<Complex<f64>>>) -> (Vec<u8>, usize, usize) {
    let (image_data, width, height) =
        transform_to_image(&wavelet_transform, &avg_fn, frequencies, sample_rate, samples, pixels_per_frequency, 32);

    save_buffer_with_format(file_name, &image_data, width as u32, height as u32,
                            image::ColorType::Rgb8, ImageFormat::Png).unwrap();

    return (image_data, width, height);
}

const PIANO_ROLL_LENGTH: usize = 24;

fn transform_to_image(transform: &Vec<Vec<Complex<f64>>>,
                      aggregation_fn: &impl Fn(f64, Complex<f64>, usize) -> f64,
                      frequencies: &Vec<f64>,
                      sample_rate: u32, samples: u32,
                      pixels_per_frequency: u32, pixels_per_second: u32, ) -> (Vec<u8>, usize, usize) {
    let chunk_size = (sample_rate / pixels_per_second) as usize;
    let new_width = PIANO_ROLL_LENGTH + samples as usize / chunk_size;
    let new_height = transform.len() * pixels_per_frequency as usize;

    let mut sampled = Vec::with_capacity(new_height);
    for i in 0..transform.len() {
        let mut row = Vec::with_capacity(new_width - PIANO_ROLL_LENGTH);
        for chunk_index in 0..(new_width - PIANO_ROLL_LENGTH) {
            let chunk_offset = chunk_index * chunk_size;
            let mut value = 0.0;
            for k in 0..chunk_size {
                value = aggregation_fn(value, transform[i][chunk_offset + k], chunk_size);
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
        (0..PIANO_ROLL_LENGTH).into_iter().for_each(|_| {
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
        for k in 0..(new_width - PIANO_ROLL_LENGTH) {
            let (r, g, b) = heat_map_color(sampled[i][k] / max);
            resized_data.push(r);
            resized_data.push(g);
            resized_data.push(b);
        }
    }
    (resized_data, new_width, new_height)
}

fn max_fn(previous: f64, value: Complex<f64>, partition_size: usize) -> f64 {
    value.abs().max(previous)
}

fn avg_fn(previous: f64, value: Complex<f64>, partition_size: usize) -> f64 {
    previous + value.abs() / (partition_size as f64)
}