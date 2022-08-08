pub mod dielectric_lambertian_metal;
pub mod lonely_sphere;
pub mod lots_of_random_spheres;
pub mod two_spheres_wide_fov;

use std::rc::Rc;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::vec3::{Point3, Vec3};

pub struct Scene {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub recursion_depth: u32,
    pub cam: Camera,
    pub world: Rc<dyn Hittable>,
}

impl Scene {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        recursion_depth: u32,
        lookfrom: Point3,
        lookat: Point3,
        vfov: f64,
        aperture_width: f64,
        focus_distance: f64,
        world: Rc<dyn Hittable>,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: ((image_width as f64) / aspect_ratio) as u32,
            samples_per_pixel,
            recursion_depth,
            cam: Camera::new(
                lookfrom,
                lookat,
                Vec3::new(0.0, 1.0, 0.0),
                vfov,
                aspect_ratio,
                aperture_width,
                focus_distance,
            ),
            world,
        }
    }
}
