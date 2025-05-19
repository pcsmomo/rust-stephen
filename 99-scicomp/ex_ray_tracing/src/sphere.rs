use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::Vector3;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, ray: &Ray) -> Option<f64> {
        // Calculate vector from ray origin to sphere center
        let oc = self.center.subtract(&ray.origin);

        // Calculate coefficients for quadratic equation
        // For ray-sphere intersection, we solve: at² + 2ht + c = 0
        // where t is the distance along the ray
        let a = ray.direction.length_squared(); // Coefficient of t²
        let h = ray.direction.dot(&oc); // Half of coefficient of t
        let c = oc.length_squared() - self.radius * self.radius; // Constant term

        // Calculate discriminant to determine if ray intersects sphere
        // If discriminant > 0, ray intersects sphere at two points
        // If discriminant = 0, ray is tangent to sphere
        // If discriminant < 0, ray misses sphere
        let discriminant = h * h - a * c;

        if discriminant > 0.0 {
            // Return the smaller of the two intersection points
            // This is the first point where the ray enters the sphere
            Some((h - discriminant.sqrt()) / a)
        } else {
            None
        }
    }

    // pub fn hit_noah_ai_gen(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
    //     // Calculate vector from sphere center to ray origin
    //     let oc = ray.origin.subtract(&self.center);

    //     // Calculate coefficients for quadratic equation
    //     // For ray-sphere intersection, we solve: t² + 2bt + c = 0
    //     // where t is the distance along the ray
    //     let a = ray.direction.dot(&ray.direction); // Always 1 for normalized rays
    //     let b = 2.0 * ray.direction.dot(&oc); // 2 * dot product of ray direction and oc
    //     let c = oc.dot(&oc) - self.radius * self.radius; // Distance squared minus radius squared

    //     // Calculate discriminant to determine if ray intersects sphere
    //     let discriminant = b * b - 4.0 * a * c;

    //     // If discriminant is negative, ray misses sphere
    //     if discriminant < 0.0 {
    //         return None;
    //     }

    //     // Calculate intersection point using quadratic formula
    //     let sqrt_discriminant = discriminant.sqrt();
    //     let mut root = (-b - sqrt_discriminant) / (2.0 * a);

    //     // Check if intersection point is within valid range
    //     if root < t_min || root > t_max {
    //         return None;
    //     }

    //     // Create hit record with intersection details
    //     let mut hit_record = HitRecord::new(ray, root);
    //     hit_record.p = ray.at(root); // Calculate intersection point
    //     hit_record.normal = (hit_record.p - self.center).divide(&self.radius); // Normalize normal vector
    //     hit_record.front_face = ray.direction.dot(&hit_record.normal) < 0.0; // Determine if ray hit from outside

    //     Some(root)
    // }
}
