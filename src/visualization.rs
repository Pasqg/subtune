use show_image::{create_window, ImageInfo, ImageView};
use crate::math::{ComplexNum, modulo};
use crate::signals::SignalSample;

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

fn find_max_modulo(result: &Vec<Vec<ComplexNum>>) -> f64 {
    let mut max_modulo = 0.0;
    for row in result {
        for value in row {
            let value = modulo(*value);
            if value > max_modulo {
                max_modulo = value;
            }
        }
    }
    max_modulo
}

fn to_rgb8(result: &Vec<Vec<ComplexNum>>, value_to_rgb: &impl Fn(f64) -> (u8, u8, u8)) -> Vec<u8> {
    let max_modulo = find_max_modulo(&result);
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

pub(crate) fn open_window(sine_samples: &SignalSample<f64>, wavelet_transform: &Vec<Vec<ComplexNum>>) {
    let image_data = to_rgb8(&wavelet_transform, &heat_map_color);
    let image = ImageView::new(ImageInfo::rgb8(sine_samples.samples.len() as u32, wavelet_transform.len() as u32), &image_data);
    let window = create_window("Wavelet transform", Default::default()).unwrap();
    window.set_image("image", image).unwrap();

    loop {};
}