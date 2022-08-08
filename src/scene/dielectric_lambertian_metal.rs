use std::rc::Rc;

use crate::color::Color;
use crate::hittable::{Hittable, HittableList, Sphere};
use crate::material::{Material, Lambertian, Dielectric, Metal};
use crate::vec3::Point3;
use crate::scene::Scene;

pub fn get_scene() -> Scene {
    // Image config
    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: u32 = 400;

    // Camera config
    let lookfrom: Point3 = Point3::new(3.0, 3.0, 2.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, -1.0);
    let vfov: f64 = 30.0;
    let aperture_width: f64 = 2.0;
    let focus_distance: f64 = (lookfrom - lookat).length();

    // Raytracer config
    let samples_per_pixel: u32 = 500;
    let recursion_depth: u32 = 50;

    // Materials
    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left:   Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    let material_right:  Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    // World creation
    let mut world = HittableList::new();
    let ground_sphere:  Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, &material_ground));
    let sphere1:        Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new( 0.0,    0.0, -1.0),   0.5, &material_center));
    let sphere2_outer:  Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, &material_left));
    let sphere2_inner:  Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0), -0.45, &material_left));
    let sphere3:        Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, &material_right));

    world.add(&ground_sphere);
    world.add(&sphere1);
    world.add(&sphere2_outer);
    world.add(&sphere2_inner);
    world.add(&sphere3);

    // world is frozen as an Rc<dyn Hittable> until the render loop is over
    let world: Rc<dyn Hittable> = Rc::new(world);

    Scene::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        recursion_depth,
        lookfrom,
        lookat,
        vfov,
        aperture_width,
        focus_distance,
        world,
    )
}
