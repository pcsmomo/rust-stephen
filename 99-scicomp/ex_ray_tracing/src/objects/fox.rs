use crate::sphere::Sphere;
use crate::vector3::Vector3;

// Construct a fox using a collection of spheres
// Each sphere represents a body part, positioned and sized accordingly
pub fn get_fox_objects() -> Vec<Sphere> {
    vec![
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
    ]
}
