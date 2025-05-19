use crate::material::Material;
use crate::sphere::Sphere;
use crate::vector3::Vector3;

pub fn get_symmetric_objects() -> Vec<Sphere> {
    let diffuse = Material::Diffuse {
        albedo: Vector3::new(0.5, 0.5, 0.5),
    };
    let metal = Material::Metal {
        albedo: Vector3::new(0.8, 0.8, 0.8),
    };
    vec![
        // Central sphere
        Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.2, metal.clone()),
        // First ring of spheres (8 spheres in a circle)
        Sphere::new(Vector3::new(0.4, 0.0, -1.0), 0.1, metal.clone()),
        Sphere::new(Vector3::new(0.283, 0.283, -1.0), 0.1, metal.clone()),
        Sphere::new(Vector3::new(0.0, 0.4, -1.0), 0.1, metal.clone()),
        Sphere::new(Vector3::new(-0.283, 0.283, -1.0), 0.1, metal.clone()),
        Sphere::new(Vector3::new(-0.4, 0.0, -1.0), 0.1, metal.clone()),
        Sphere::new(Vector3::new(-0.283, -0.283, -1.0), 0.1, metal.clone()),
        Sphere::new(Vector3::new(0.0, -0.4, -1.0), 0.1, metal.clone()),
        Sphere::new(Vector3::new(0.283, -0.283, -1.0), 0.1, metal.clone()),
        // Second ring (16 smaller spheres)
        Sphere::new(Vector3::new(0.6, 0.0, -1.0), 0.05, metal.clone()),
        Sphere::new(Vector3::new(0.424, 0.424, -1.0), 0.05, metal.clone()),
        Sphere::new(Vector3::new(0.0, 0.6, -1.0), 0.05, metal.clone()),
        Sphere::new(Vector3::new(-0.424, 0.424, -1.0), 0.05, metal.clone()),
        Sphere::new(Vector3::new(-0.6, 0.0, -1.0), 0.05, metal.clone()),
        Sphere::new(Vector3::new(-0.424, -0.424, -1.0), 0.05, metal.clone()),
        Sphere::new(Vector3::new(0.0, -0.6, -1.0), 0.05, metal.clone()),
        Sphere::new(Vector3::new(0.424, -0.424, -1.0), 0.05, metal.clone()),
        // Additional points between first ring points
        Sphere::new(Vector3::new(0.35, 0.15, -1.0), 0.05, diffuse.clone()),
        Sphere::new(Vector3::new(0.15, 0.35, -1.0), 0.05, diffuse.clone()),
        Sphere::new(Vector3::new(-0.15, 0.35, -1.0), 0.05, diffuse.clone()),
        Sphere::new(Vector3::new(-0.35, 0.15, -1.0), 0.05, diffuse.clone()),
        Sphere::new(Vector3::new(-0.35, -0.15, -1.0), 0.05, diffuse.clone()),
        Sphere::new(Vector3::new(-0.15, -0.35, -1.0), 0.05, diffuse.clone()),
        Sphere::new(Vector3::new(0.15, -0.35, -1.0), 0.05, diffuse.clone()),
        Sphere::new(Vector3::new(0.35, -0.15, -1.0), 0.05, diffuse.clone()),
    ]
}
