//! Simple ray tracer rendering a fox using spheres
mod camera;
mod ray;
mod sphere;
mod vector3;

use image::RgbImage;
use sphere::Sphere;
use vector3::Vector3;

use crate::camera::Camera;

fn main() {
    // Image dimensions and camera field of view
    let image_width = 800;
    let image_height = 600;
    let field_of_view = 45;

    // Initialize camera with the specified field of view
    let camera = Camera::new(field_of_view as f64);
    let mut image = RgbImage::new(image_width, image_height);

    // Construct a fox using a collection of spheres
    // Each sphere represents a body part, positioned and sized accordingly
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
        // Tails (two spheres for a stylized look)
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

    // Render the scene from the camera's perspective
    camera.render(&mut image, &objects);

    // Save the rendered image to disk
    let filename = format!("results/06_refactor_ray_tracer.png");
    image.save(&filename).unwrap();
    println!("Saved {}", filename);
}
