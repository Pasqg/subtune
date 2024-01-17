use image::{ImageFormat, save_buffer_with_format};
use show_image::{create_window, ImageInfo, ImageView};
use crate::utils::math::{ComplexNum, modulo};

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let (r1, g1, b1) = match h_prime {
        0.0..=1.0 => (c, x, 0.0),
        1.0..=2.0 => (x, c, 0.0),
        2.0..=3.0 => (0.0, c, x),
        3.0..=4.0 => (0.0, x, c),
        4.0..=5.0 => (x, 0.0, c),
        5.0..=6.0 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
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

fn to_rgb8(result: &Vec<Vec<ComplexNum>>, value_to_rgb: &impl Fn(f64) -> (u8, u8, u8)) -> Vec<u8> {
    let max_modulo = find_max(&result, &modulo);
    let mut image_data = Vec::new();
    for row in result {
        for value in row {
            let (r, g, b) = value_to_rgb(modulo(*value) / max_modulo);
            image_data.push(r);
            image_data.push(g);
            image_data.push(b);
        }
    }
    image_data
}

pub(crate) fn open_window(width: u32, heigth: u32, image_data: &Vec<u8>) {
    let image = ImageView::new(ImageInfo::rgb8(width, heigth), &image_data);
    let window = create_window("Wavelet transform", Default::default()).unwrap();
    window.set_image("image", image).unwrap();

    loop {};
}

pub(crate) fn output_image(file_name: &str, frequencies: u32, sample_rate: u32, samples: u32,
                           wavelet_transform: &Vec<Vec<ComplexNum>>) -> Vec<u8> {
    let (samples, image_data) =
        sample_transform(frequencies, sample_rate, samples, &wavelet_transform, &avg_fn);

    save_buffer_with_format(file_name, &image_data, samples as u32, frequencies,
                            image::ColorType::Rgb8, ImageFormat::Png).unwrap();

    return image_data;
}

fn sample_transform(frequencies: u32, sample_rate: u32, samples: u32, transform: &Vec<Vec<ComplexNum>>,
                    aggregation_fn: &impl Fn(f64, ComplexNum, usize) -> f64) -> (usize, Vec<u8>) {
    let chunk_size = (sample_rate / 32) as usize;
    let new_width = samples as usize / chunk_size;

    let frequencies = frequencies as usize;
    let mut sampled = Vec::with_capacity(frequencies);
    for i in 0..frequencies {
        let mut row = Vec::with_capacity(new_width);
        for chunk_index in 0..new_width {
            let chunk_offset = chunk_index * chunk_size;
            let mut value = 0.0;
            for k in 0..chunk_size {
                value = aggregation_fn(value, transform[i][chunk_offset + k], chunk_size);
            }
            row.push(value);
        }
        sampled.push(row);
    }

    let max = find_max(&sampled, &std::convert::identity);
    let mut resized_data = Vec::with_capacity(frequencies * new_width * 3);
    for i in 0..frequencies {
        for k in 0..new_width {
            let (r, g, b) = heat_map_color(sampled[i][k] / max);
            resized_data.push(r);
            resized_data.push(g);
            resized_data.push(b);
        }
    }
    (new_width, resized_data)
}

fn max_fn(previous: f64, value: ComplexNum, partition_size: usize) -> f64 {
    modulo(value).max(previous)
}

fn avg_fn(previous: f64, value: ComplexNum, partition_size: usize) -> f64 {
    previous + modulo(value) / (partition_size as f64)
}