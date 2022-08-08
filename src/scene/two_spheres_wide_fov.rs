use std::rc::Rc;

use crate::PI;
use crate::color::Color;
use crate::hittable::{Hittable, HittableList, Sphere};
use crate::material::{Material, Lambertian};
use crate::vec3::Point3;
use crate::scene::Scene;

pub fn get_scene() -> Scene {
    // Image config
    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: u32 = 400;

    // Camera config
    let vfov: f64 = 90.0;

    // Raytracer config
    let samples_per_pixel: u32 = 100;
    let recursion_depth: u32 = 50;

    // Materials
    let material_left:   Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right:  Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    // World creation
    let r = (PI / 4.0).cos();
    let mut world = HittableList::new();
    let sphere1: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(-r, 0.0, -1.0), r, &material_left));
    let sphere2: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new( r, 0.0, -1.0), r, &material_right));

    world.add(&sphere1);
    world.add(&sphere2);

    // world is frozen as an Rc<dyn Hittable> until the render loop is over
    let world: Rc<dyn Hittable> = Rc::new(world);

    Scene::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        recursion_depth,
        vfov,
        world,
    )
}
