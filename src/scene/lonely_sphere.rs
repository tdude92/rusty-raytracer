use std::rc::Rc;

use crate::color::Color;
use crate::hittable::{Hittable, Sphere};
use crate::material::{Material, Lambertian};
use crate::vec3::Point3;
use crate::scene::Scene;

pub fn get_scene() -> Scene {
    // Image config
    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: u32 = 400;

    // Camera config
    let lookfrom: Point3 = Point3::new(0.0, 0.0, 0.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, -1.0);
    let vfov: f64 = 90.0;

    // Raytracer config
    let samples_per_pixel: u32 = 100;
    let recursion_depth: u32 = 50;

    // Materials
    let material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));

    // World creation
    let sphere: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new( 0.0,  0.0,   -1.0), 0.5, &material));

    Scene::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        recursion_depth,
        lookfrom,
        lookat,
        vfov,
        sphere,
    )
}
