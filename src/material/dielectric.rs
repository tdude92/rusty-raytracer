use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::random_f64;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Dielectric {
    eta: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Dielectric {eta: refractive_index}
    }

    /// Schlick reflectance approximation
    fn reflectance(cos_theta: f64, eta_ratio: f64) -> f64 {
        let r0 = (1.0 - eta_ratio) / (1.0 + eta_ratio);
        let r0 = r0*r0;
        r0 + (1.0 - r0)*(1.0 - cos_theta).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let eta_ratio = if hit_record.front_face {1.0/self.eta} else {self.eta};
        let unit_direction = r.dir().unit_vector();

        let cos_theta = Vec3::dot(&(-(&unit_direction)), hit_record.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = eta_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, self.eta) > random_f64() {
            // Snell's law has no solution; must reflect
            unit_direction.reflect(hit_record.normal())
        } else {
            // Snell's law has a solution; can refract
            unit_direction.refract(hit_record.normal(), eta_ratio)
        };
        
        let scattered = Ray::new(hit_record.p().clone(), direction);
        let attenuation = Color::new(1.0, 1.0, 1.0);

        Some((attenuation, scattered))
    }
}
