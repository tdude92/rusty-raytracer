use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,  // Vertical field-of-view in degrees
        aspect_ratio: f64,
        aperture_width: f64,
        focus_distance: f64,
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
        let horizontal = focus_distance * viewport_width * u; // Horizontal unit vector across +u-axis in focus plane
        let vertical = focus_distance * viewport_height * v;  // Vertical unit vector across +v-axis in focus plane
        let lower_left_corner = &origin - &horizontal/2.0 - &vertical/2.0 - &w*focus_distance;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture_width / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disc();
        let offset = &self.u*rd.x() + &self.v*rd.y();

        // s and t are in [0, 1]
        Ray::new(
            &self.origin + &offset,
            &self.lower_left_corner + &self.horizontal*s + &self.vertical*t - &self.origin - &offset
        )
    }
}
