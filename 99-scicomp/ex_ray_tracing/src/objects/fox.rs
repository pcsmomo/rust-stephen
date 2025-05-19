use crate::material::Material;
use crate::sphere::Sphere;
use crate::vector3::Vector3;

// Construct a fox using a collection of spheres
// Each sphere represents a body part, positioned and sized accordingly
pub fn get_fox_objects() -> Vec<Sphere> {
    let diffuse = Material::Diffuse {
        albedo: Vector3::new(0.5, 0.5, 0.5),
    };
    let metal = Material::Metal {
        albedo: Vector3::new(0.8, 0.8, 0.8),
    };
    vec![
        // Head
        Sphere::new(Vector3::new(0.0, 0.15, -1.0), 0.12, diffuse.clone()),
        // Snout
        Sphere::new(Vector3::new(0.0, 0.08, -0.85), 0.04, metal.clone()),
        // Left ear
        Sphere::new(Vector3::new(-0.08, 0.28, -1.0), 0.04, metal.clone()),
        // Right ear
        Sphere::new(Vector3::new(0.08, 0.28, -1.0), 0.04, metal.clone()),
        // Left eye
        Sphere::new(Vector3::new(-0.04, 0.2, -0.9), 0.015, metal.clone()),
        // Right eye
        Sphere::new(Vector3::new(0.04, 0.2, -0.9), 0.015, metal.clone()),
        // Body
        Sphere::new(Vector3::new(0.0, -0.08, -1.0), 0.16, metal.clone()),
        // Left upper arm
        Sphere::new(Vector3::new(-0.12, 0.0, -1.0), 0.06, metal.clone()),
        // Right upper arm
        Sphere::new(Vector3::new(0.12, 0.0, -1.0), 0.06, metal.clone()),
        // Tails (two spheres for a stylized look)
        Sphere::new(Vector3::new(0.15, -0.15, -1.0), 0.06, metal.clone()),
        Sphere::new(Vector3::new(0.20, -0.20, -1.0), 0.04, metal.clone()),
        // Left front leg
        Sphere::new(Vector3::new(-0.08, -0.25, -1.0), 0.04, metal.clone()),
        // Right front leg
        Sphere::new(Vector3::new(0.08, -0.25, -1.0), 0.04, metal.clone()),
        // Left back leg
        Sphere::new(Vector3::new(-0.08, -0.32, -1.0), 0.04, metal.clone()),
        // Right back leg
        Sphere::new(Vector3::new(0.08, -0.32, -1.0), 0.04, metal.clone()),
    ]
}
