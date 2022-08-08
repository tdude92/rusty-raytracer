use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,  // Vertical field-of-view in degrees
        aspect_ratio: f64,
    ) -> Self {
        let aspect_ratio = aspect_ratio;

        let vfov = vfov.to_radians();
        let h = (vfov/2.0).tan();

        let viewport_height = 2.0*h;
        let viewport_width = aspect_ratio*viewport_height;

        let w = (&lookfrom - &lookat).unit_vector();
        let u = Vec3::cross(&vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u).unit_vector();

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = &origin - &horizontal/2.0 - &vertical/2.0 - &w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        // s and t are in [0, 1]
        Ray::new(self.origin, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin)
    }
}
