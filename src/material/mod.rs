pub mod lambertian;
pub mod metal;

// Re-export structs that implement Material
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material {
    /// Takes an incident ray r and a hit record
    /// Returns an Option<(attenuation, scattered ray)>
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}
