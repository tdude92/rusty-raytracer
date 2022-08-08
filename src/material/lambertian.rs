use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian {albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = hit_record.normal() + Vec3::random_unit_vector();

        // Handle case where scatter_direction ~= Vec::new(0.0, 0.0, 0.0)
        let scatter_direction = if scatter_direction.is_near_zero() {hit_record.normal().clone()} else {scatter_direction};

        let scattered = Ray::new(hit_record.p().clone(), scatter_direction);
        let attenuation = self.albedo.clone();
        Some((attenuation, scattered))
    }
}
