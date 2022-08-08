pub mod hittable_list;
pub mod sphere;

// Re-export structs that implement Hittable
pub use sphere::Sphere;
pub use hittable_list::HittableList;

use std::rc::Rc;
use crate::material::Material;
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
    p: Point3,
    normal: Vec3,
    material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn p(&self) -> &Point3 {&self.p}
    pub fn normal(&self) -> &Vec3 {&self.normal}
    pub fn material(&self) -> &Rc<dyn Material> {&self.material}
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
