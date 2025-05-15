use image::{Rgb, RgbImage};

pub fn fill_one_color(img: &mut RgbImage, color: Rgb<u8>) {
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            img.put_pixel(x, y, color);
        }
    }
}

pub fn fill_gradient(img: &mut RgbImage) {
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let x_ndc = x as f64 / (width - 1) as f64;
            let y_ndc = y as f64 / (height - 1) as f64;

            let r = (x_ndc * 255.0) as u8;
            let g = 0;
            let b = (y_ndc * 255.0) as u8;

            let color = Rgb([r, g, b]);
            img.put_pixel(x, y, color);
        }
    }
}

pub fn fill_gradient_radial(img: &mut RgbImage) {
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            // Normalize coordinates to range [-1, 1] for symmetry
            let x_ndc = 2.0 * (x as f64 / (width - 1) as f64) - 1.0;
            let y_ndc = 2.0 * (y as f64 / (height - 1) as f64) - 1.0;

            // Calculate distance from center (creates radial symmetry)
            let distance = (x_ndc * x_ndc + y_ndc * y_ndc).sqrt();

            // Create symmetric color pattern
            let r = ((1.0 - distance).max(0.0) * 255.0) as u8;
            let g = ((x_ndc.abs()) * 255.0) as u8;
            let b = ((y_ndc.abs()) * 255.0) as u8;

            let color = Rgb([r, g, b]);
            img.put_pixel(x, y, color);
        }
    }
}
