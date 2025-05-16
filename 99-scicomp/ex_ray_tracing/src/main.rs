mod vector3;

use image::{Rgb, RgbImage};

// use vector3::Vector3;
use crate::vector3::Vector3;

/// Linear interpolation between two values.
///
/// `lerp` calculates a value between `start` and `end` based on parameter `t`:
/// - When t = 0.0, returns `start`
/// - When t = 1.0, returns `end`
/// - When 0.0 < t < 1.0, returns a proportional value between `start` and `end`
///
/// This is commonly used in graphics to blend between two values (colors, positions, etc.)
/// based on a normalized parameter.
pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
    (1.0 - t) * start + t * end
}

fn create_ray_sky(image_width: u32, image_height: u32) -> RgbImage {
    let aspect_ratio = image_width as f64 / image_height as f64;
    let mut image = RgbImage::new(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {
            let x_ndc = (x as f64 + 0.5) / (image_width - 1) as f64;
            let y_ndc = (y as f64 + 0.5) / (image_height - 1) as f64;

            let x_pixel_camera = (2.0 * x_ndc - 1.0) * aspect_ratio;
            let y_pixel_camera = 1.0 - 2.0 * y_ndc;

            let mut ray_direction = Vector3::new(x_pixel_camera, y_pixel_camera, -1.0);
            ray_direction.normalize();

            let t = 0.5 * (ray_direction.y + 1.0);

            let r = (lerp(1.0, 0.5, t) * 255.0) as u8;
            let g = (lerp(1.0, 0.7, t) * 255.0) as u8;
            let b = 255;

            let color = Rgb([r, g, b]);
            image.put_pixel(x, y, color);
        }
    }

    image
}

fn main() {
    let image_width = 800;
    let image_height = 600;
    let image = create_ray_sky(image_width, image_height);

    image.save("results/image_ray.png").unwrap();
}
