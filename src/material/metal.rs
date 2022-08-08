use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 {fuzz} else {1.0},
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r.dir().unit_vector().reflect(hit_record.normal());
        let scattered = Ray::new(hit_record.p().clone(), reflected + self.fuzz*Vec3::random_in_unit_sphere());
        let attenuation = self.albedo.clone();
        
        if Vec3::dot(scattered.dir(), hit_record.normal()) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
