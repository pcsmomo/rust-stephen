//! Simple ray tracer rendering a fox using spheres
mod camera;
mod objects;
mod ray;
mod sphere;
mod vector3;

use image::RgbImage;

use crate::camera::Camera;
use crate::objects::fox;

fn main() {
    // Image dimensions and camera field of view
    let image_width = 800;
    let image_height = 600;
    let field_of_view = 45;
    let max_bounce: u8 = 10;

    // Initialize camera with the specified field of view
    let camera = Camera::new(field_of_view as f64, max_bounce);
    let mut image = RgbImage::new(image_width, image_height);

    let objects = fox::get_fox_objects();

    // Render the scene from the camera's perspective
    camera.render(&mut image, &objects);

    // Save the rendered image to disk
    let filename = format!("results/06_refactor_ray_tracer.png");
    image.save(&filename).unwrap();
    println!("Saved {}", filename);
}
