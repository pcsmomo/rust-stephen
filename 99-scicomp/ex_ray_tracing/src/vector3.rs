use num::Float;
use rand_distr::uniform::SampleUniform;
use rand_distr::{Distribution, UnitSphere};

#[derive(Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }

    pub fn length_squared(&self) -> T {
        // We compute the sum of squares (x² + y² + z²) which is useful on its own
        // and avoids the expensive square root operation when only comparing lengths
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> T {
        // We calculate the length using the Euclidean distance formula: √(x² + y² + z²)
        // We separate this into two steps (square then sqrt) for two reasons:
        // 1. It allows reuse of length_squared() which is useful on its own
        // 2. Square root is computationally expensive, so we avoid it when possible
        self.length_squared().sqrt()
    }

    pub fn normalize(&mut self) {
        let length = self.length();
        self.x = self.x / length;
        self.y = self.y / length;
        self.z = self.z / length;
    }

    pub fn dot(&self, rhs: &Vector3<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn subtract(&self, rhs: &Vector3<T>) -> Vector3<T> {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }

    pub fn add(&self, rhs: &Vector3<T>) -> Vector3<T> {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }

    pub fn multiply(&self, value: T) -> Vector3<T> {
        Vector3::new(self.x * value, self.y * value, self.z * value)
    }
}

impl<T: Float + SampleUniform> Vector3<T> {
    pub fn random_unit_sphere() -> Self {
        let v: [T; 3] = UnitSphere.sample(&mut rand::rng());
        Vector3::new(v[0], v[1], v[2])
    }

    pub fn random_on_hemisphere(normal: &Vector3<T>) -> Self {
        let vec_on_sphere = Self::random_unit_sphere();

        if vec_on_sphere.dot(normal).is_sign_positive() {
            vec_on_sphere
        } else {
            Vector3::new(-vec_on_sphere.x, -vec_on_sphere.y, -vec_on_sphere.z)
        }
    }
}
