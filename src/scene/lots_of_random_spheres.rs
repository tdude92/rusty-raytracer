use std::rc::Rc;

use crate::color::Color;
use crate::hittable::{Hittable, HittableList, Sphere};
use crate::material::{Material, Lambertian, Dielectric, Metal};
use crate::{random_f64, random_f64_in};
use crate::vec3::{Point3, Vec3};
use crate::scene::Scene;

pub fn get_scene() -> Scene {
    // Image config
    let aspect_ratio: f64 = 3.0/2.0;
    let image_width: u32 = 1200;

    // Camera config
    let lookfrom: Point3 = Point3::new(13.0, 2.0, 3.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, 0.0);
    let vfov: f64 = 20.0;
    let aperture_width: f64 = 0.1;
    let focus_distance: f64 = 10.0;

    // Raytracer config
    let samples_per_pixel: u32 = 500;
    let recursion_depth: u32 = 50;

    // Materials
    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    // World creation
    let mut world = HittableList::new();

    // Add ground
    let ground_sphere:  Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, &material_ground));
    world.add(&ground_sphere);

    // Create random spheres
    let mut spheres: Vec<Sphere> = vec![];
    for _ in 0..200 {
        loop {
            let choose_mat = random_f64();
            let material: Rc<dyn Material> = if choose_mat < 0.7 {
                // Lambertian
                let albedo = Color::random();
                Rc::new(Lambertian::new(albedo))
            } else if choose_mat < 0.9 {
                // Metal
                let albedo = Color::random();
                let fuzz = random_f64()*0.2;
                Rc::new(Metal::new(albedo, fuzz))
            } else {
                // Dielectric
                let refractive_index = random_f64_in(1.5, 3.5);
                Rc::new(Dielectric::new(refractive_index))
            };

            let radius = if random_f64() < 0.1 {
                // Large sphere
                random_f64_in(0.5, 1.2)
            } else {
                // Small sphere
                0.2
            };
            let ground_normal = Point3::random_unit_vector();
            let ground_point = 100.0*ground_normal + Point3::new(0.0, -100.5, -1.0);  // Point on ground sphere
            let center = &ground_point + &ground_normal*radius;

            // Continue if not within frame
            if Vec3::dot(&Vec3::new(0.0, 1.0, 0.0), &ground_normal).acos().to_degrees() > 5.0 {continue;}

            // Continue if intersecting with another sphere
            let mut overlaps = false;
            for sphere in spheres.iter() {
                let min_distance = radius + sphere.radius();
                if (&center - sphere.center()).length() < min_distance {
                    overlaps = true;
                }
            }
            if overlaps {continue;}

            spheres.push(Sphere::new(center, radius, &material));
            break;
        }
    }

    // Add spheres to the world
    for sphere in spheres.drain(..) {
        let rc_sphere: Rc<dyn Hittable> = Rc::new(sphere);
        world.add(&rc_sphere);
    }

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
