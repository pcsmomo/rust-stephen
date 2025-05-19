// Camera module: Implements a simple pinhole camera for ray tracing
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::Vector3;
use image::{Rgb, RgbImage};

/// Represents a simple camera with a configurable field of view.
pub struct Camera {
    pub field_of_view: f64, // Field of view in degrees
}

impl Camera {
    /// Constructs a new Camera with the given field of view (in degrees).
    pub fn new(field_of_view: f64) -> Self {
        Camera { field_of_view }
    }

    /// Linear interpolation between two values.
    ///
    /// `lerp` calculates a value between `start` and `end` based on parameter `t`:
    /// - When t = 0.0, returns `start`
    /// - When t = 1.0, returns `end`
    /// - When 0.0 < t < 1.0, returns a proportional value between `start` and `end`
    ///
    /// This is commonly used in graphics to blend between two values (colors, positions, etc.)
    /// based on a normalized parameter.
    pub fn lerp(&self, start: f64, end: f64, t: f64) -> f64 {
        (1.0 - t) * start + t * end
    }

    /// Renders a ray-traced image of the scene with spheres and a gradient background.
    ///
    /// # Arguments
    /// * `image` - Mutable reference to the image buffer to render into
    /// * `objects` - List of spheres to render in the scene
    ///
    /// This function implements a basic ray tracer that:
    /// 1. Creates a sphere in 3D space
    /// 2. Casts rays from the camera through each pixel
    /// 3. Colors pixels based on ray-sphere intersection or background gradient
    pub fn render(&self, image: &mut RgbImage, objects: &Vec<Sphere>) {
        let image_width = image.width();
        let image_height = image.height();

        // Calculate aspect ratio to maintain proper proportions
        let aspect_ratio = image_width as f64 / image_height as f64;

        // Convert field of view to a scaling factor for the image plane
        let field_of_view_factor = (self.field_of_view.to_radians() / 2.0).tan();

        // Iterate through each pixel in the image
        for y in 0..image_height {
            for x in 0..image_width {
                // Convert pixel coordinates to normalized device coordinates (NDC)
                // Add 0.5 to center the ray within the pixel for anti-aliasing
                let x_ndc = (x as f64 + 0.5) / (image_width - 1) as f64;
                let y_ndc = (y as f64 + 0.5) / (image_height - 1) as f64;

                // Convert NDC to camera space coordinates
                // Scale x by aspect ratio and field of view to prevent distortion
                let x_pixel_camera = (2.0 * x_ndc - 1.0) * aspect_ratio * field_of_view_factor;
                let y_pixel_camera = (1.0 - 2.0 * y_ndc) * field_of_view_factor;

                // Create a ray from the camera origin (0,0,0) through the pixel
                let ray = Ray::new(
                    Vector3::new(0.0, 0.0, 0.0),                        // Camera position
                    Vector3::new(x_pixel_camera, y_pixel_camera, -1.0), // Ray direction (z = -1)
                );

                // Track the closest intersection (if any)
                let mut closest_t: Option<f64> = None;
                let mut closest_object: Option<&Sphere> = None;

                // Test the ray against all objects in the scene
                for object in objects.iter() {
                    match object.hit(&ray) {
                        Some(t) => {
                            // Update closest intersection if this one is nearer
                            if closest_t.is_none() || t < closest_t.unwrap_or(10000000.0) {
                                closest_t = Some(t);
                                closest_object = Some(object);
                            }
                        }
                        None => {}
                    }
                }

                match closest_object {
                    Some(object) => {
                        // Compute intersection point and normal
                        let hit_point = ray.at(closest_t.unwrap());
                        let mut normal = hit_point.subtract(&object.center);
                        normal.normalize();

                        // Simple shading: encode normal as color, scaled by brightness
                        let brightness = 0.7;
                        let r = ((normal.x + 1.0) * brightness * 255.0) as u8;
                        let g = ((normal.y + 1.0) * brightness * 255.0) as u8;
                        let b = ((normal.z + 1.0) * brightness * 255.0) as u8;

                        // Write the color to the image
                        image.put_pixel(x, y, Rgb([r, g, b]));
                    }
                    None => {
                        // No intersection: render background gradient
                        // Calculate gradient parameter based on ray direction
                        let t = 0.5 * (ray.direction.y / field_of_view_factor + 1.0);

                        // Interpolate between two colors for the gradient
                        let r = (self.lerp(1.0, 0.5, t) * 255.0) as u8; // From white to light blue
                        let g = 255;
                        let b = (self.lerp(1.0, 0.3, t) * 255.0) as u8; // From white to blue

                        let color = Rgb([r, g, b]);
                        image.put_pixel(x, y, color);
                    }
                }
            }
        }
        // The rendered image is written in-place to the provided buffer
    }
}
