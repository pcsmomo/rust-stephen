//! Simple ray tracer rendering a fox using spheres
mod camera;
mod material;
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
    let max_bounce: u8 = 10;

    for fov in 1..=90 {
        // Initialize camera with the specified field of view
        let camera = Camera::new(fov as f64, max_bounce);
        let mut image = RgbImage::new(image_width, image_height);

        let objects = fox::get_fox_objects();

        // Render the scene from the camera's perspective
        camera.render(&mut image, &objects);

        // Save the rendered image to disk
        let filename = format!("results/fov/frame_{:03}.png", fov);
        image.save(&filename).unwrap();
        println!("Saved {}", filename);
    }
}

// ffmpeg -framerate 15 -i results/fov/frame_%03d.png -c:v libx264 -pix_fmt yuv420p results/fov/animation.mp4
