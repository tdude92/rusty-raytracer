use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal {albedo}
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r.dir().unit_vector().reflect(hit_record.normal());
        let scattered = Ray::new(hit_record.p().clone(), reflected);
        let attenuation = self.albedo.clone();
        
        if Vec3::dot(scattered.dir(), hit_record.normal()) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
