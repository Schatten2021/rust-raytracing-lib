use crate::math::Vector3;
use crate::object::CustomShape;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub position: Vector3,
    pub radius: f64,
}
impl Sphere {
    pub fn new(position: Vector3, radius: f64) -> Sphere {
        Self { position, radius }
    }
}
impl CustomShape for Sphere {
    fn distance(&self, ray_position: Vector3, ray_direction: Vector3) -> Option<f64> {
        let offset = ray_position - self.position;
        let ray_direction = ray_direction.norm();
        let a = ray_direction.dot(ray_direction);
        let b = 2f64 * offset.dot(ray_direction);
        let c = offset.dot(offset) - self.radius * self.radius;
        let discriminant = b * b - 4f64 * a * c;
        if discriminant <= 1e-100 {
            return None;
        }
        Some((-b - discriminant.sqrt()) / (2f64 * a))
    }
    fn normal(&self, world_position: Vector3) -> Vector3 {
        (world_position - self.position).norm()
    }
}