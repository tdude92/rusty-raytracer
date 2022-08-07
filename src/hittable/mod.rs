pub mod hittable_list;
pub mod sphere;

// Re-export structs that implement Hittable
pub use sphere::Sphere;
pub use hittable_list::HittableList;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

/// Converts an outward normal (pointing out from surface) into a normal
/// that always opposes Ray r.
/// 
/// # Returns
/// (bool, Vec3) a tuple of (whether r is pointing through the front face, normal that opposes r)
pub fn into_opposing_normal(r: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let front_face = Vec3::dot(r.dir(), &outward_normal) < 0.0;
    let normal = if front_face {outward_normal} else {-outward_normal};
    (front_face, normal)
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
