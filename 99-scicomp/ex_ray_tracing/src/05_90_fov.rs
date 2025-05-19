mod ray;
mod sphere;
mod vector3;

use image::{Rgb, RgbImage};

// use vector3::Vector3;
use crate::ray::Ray;
use crate::sphere::Sphere;
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

/// Creates a ray-traced image with a sphere and gradient background.
///
/// This function implements a basic ray tracer that:
/// 1. Creates a sphere in 3D space
/// 2. Casts rays from the camera through each pixel
/// 3. Colors pixels based on ray-sphere intersection or background gradient
fn create_ray(image_width: u32, image_height: u32, field_of_view: f64) -> RgbImage {
    // Calculate aspect ratio to maintain proper proportions
    let aspect_ratio = image_width as f64 / image_height as f64;

    // Create a fox using spheres
    let objects = vec![
        // Head
        Sphere::new(Vector3::new(0.0, 0.15, -1.0), 0.12),
        // Snout
        Sphere::new(Vector3::new(0.0, 0.08, -0.85), 0.04),
        // Left ear
        Sphere::new(Vector3::new(-0.08, 0.28, -1.0), 0.04),
        // Right ear
        Sphere::new(Vector3::new(0.08, 0.28, -1.0), 0.04),
        // Left eye
        Sphere::new(Vector3::new(-0.04, 0.2, -0.9), 0.015),
        // Right eye
        Sphere::new(Vector3::new(0.04, 0.2, -0.9), 0.015),
        // Body
        Sphere::new(Vector3::new(0.0, -0.08, -1.0), 0.16),
        // Tails
        Sphere::new(Vector3::new(0.15, -0.15, -1.0), 0.06),
        Sphere::new(Vector3::new(0.20, -0.20, -1.0), 0.04),
        // Left front leg
        Sphere::new(Vector3::new(-0.08, -0.25, -1.0), 0.04),
        // Right front leg
        Sphere::new(Vector3::new(0.08, -0.25, -1.0), 0.04),
        // Left back leg
        Sphere::new(Vector3::new(-0.08, -0.32, -1.0), 0.04),
        // Right back leg
        Sphere::new(Vector3::new(0.08, -0.32, -1.0), 0.04),
    ];

    // Initialize the output image
    let mut image = RgbImage::new(image_width, image_height);

    let field_of_view_factor = (field_of_view.to_radians() / 2.0).tan();

    // Iterate through each pixel in the image
    for y in 0..image_height {
        for x in 0..image_width {
            // Convert pixel coordinates to normalized device coordinates (NDC)
            // Add 0.5 to center the ray within the pixel
            let x_ndc = (x as f64 + 0.5) / (image_width - 1) as f64;
            let y_ndc = (y as f64 + 0.5) / (image_height - 1) as f64;

            // Convert NDC to camera space coordinates
            // Scale x by aspect ratio to prevent distortion
            // let x_pixel_camera = (2.0 * x_ndc - 1.0) * aspect_ratio;
            let x_pixel_camera = (2.0 * x_ndc - 1.0) * aspect_ratio * field_of_view_factor;
            let y_pixel_camera = (1.0 - 2.0 * y_ndc) * field_of_view_factor;

            // Create a ray from the camera origin through the pixel
            let ray = Ray::new(
                Vector3::new(0.0, 0.0, 0.0),                        // Camera position
                Vector3::new(x_pixel_camera, y_pixel_camera, -1.0), // Ray direction
            );

            let mut closest_t: Option<f64> = None;
            let mut closest_object: Option<&Sphere> = None;

            for object in objects.iter() {
                match object.hit(&ray) {
                    Some(t) => {
                        if closest_t.is_none() || t < closest_t.unwrap_or(10000000.0) {
                            // t < closest_t.unwrap_or(10000000.0) {
                            closest_t = Some(t);
                            closest_object = Some(object);
                        }
                    }
                    None => {}
                }
            }

            match closest_object {
                Some(object) => {
                    let hit_point = ray.at(closest_t.unwrap());
                    let mut normal = hit_point.subtract(&object.center);
                    normal.normalize();

                    let brightness = 0.7;

                    let r = ((normal.x + 1.0) * brightness * 255.0) as u8;
                    let g = ((normal.y + 1.0) * brightness * 255.0) as u8;
                    let b = ((normal.z + 1.0) * brightness * 255.0) as u8;

                    // create a color based on the normal
                    image.put_pixel(x, y, Rgb([r, g, b]));
                }
                None => {
                    // Calculate gradient parameter based on ray direction
                    let t = 0.5 * (ray.direction.y / field_of_view_factor + 1.0);

                    // Interpolate colors for gradient effect
                    let r = (lerp(1.0, 0.5, t) * 255.0) as u8;
                    let g = 255;
                    let b = (lerp(1.0, 0.3, t) * 255.0) as u8;

                    let color = Rgb([r, g, b]);
                    image.put_pixel(x, y, color);
                }
            }
        }
    }

    image
}

fn main() {
    let image_width = 800;
    let image_height = 600;

    for fov in 1..=90 {
        let image = create_ray(image_width, image_height, fov as f64);
        let filename = format!("results/fov/frame_{:03}.png", fov);
        image.save(&filename).unwrap();
        println!("Saved {}", filename);
    }
}

// ffmpeg -framerate 30 -i results/fov/frame_%03d.png -c:v libx264 -pix_fmt yuv420p results/fov/animation.mp4
