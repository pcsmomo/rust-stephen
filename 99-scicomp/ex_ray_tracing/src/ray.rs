use crate::vector3::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        let mut norm_dir = direction;
        norm_dir.normalize();

        Ray {
            origin,
            direction: norm_dir,
        }
    }

    pub fn at(&self, t: f64) -> Vector3<f64> {
        Vector3::new(
            self.origin.x + t * self.direction.x,
            self.origin.y + t * self.direction.y,
            self.origin.z + t * self.direction.z,
        )
    }
}
