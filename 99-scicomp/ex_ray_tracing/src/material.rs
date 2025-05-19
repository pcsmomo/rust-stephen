use crate::ray::Ray;
use crate::vector3::Vector3;

#[derive(Debug, Clone)]
pub enum Material {
    Diffuse { albedo: Vector3<f64> },
    Metal { albedo: Vector3<f64> },
}

impl Material {
    pub fn scatter(
        &self,
        ray: Ray,
        position: &Vector3<f64>,
        normal: &Vector3<f64>,
    ) -> Option<(Ray, &Vector3<f64>)> {
        match self {
            Material::Diffuse { albedo } => {
                let scatter_direction = Vector3::random_on_hemisphere(&normal);

                let new_ray_origin = Vector3::new(
                    position.x + normal.x * 0.00000001,
                    position.y + normal.y * 0.00000001,
                    position.z + normal.z * 0.00000001,
                );

                // let scattered_ray = Ray::new(new_ray_origin, scatter_direction);
                let scattered_ray = Ray::new(position.clone(), scatter_direction);

                Some((scattered_ray, &albedo))
            }
            Material::Metal { albedo } => {
                let reflected = ray.direction.reflect(normal);
                let scattered_ray = Ray::new(position.clone(), reflected);
                Some((scattered_ray, &albedo))
            }
        }
    }
}
