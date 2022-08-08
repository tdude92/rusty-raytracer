use std::rc::Rc;

use image::{ImageFormat, RgbImage};

use rusty_raytracer::material::{Lambertian, Metal, Material};
use rusty_raytracer::{write_pixel, random_f64};
use rusty_raytracer::camera::Camera;
use rusty_raytracer::color::Color;
use rusty_raytracer::hittable::{Hittable, HittableList, Sphere};
use rusty_raytracer::vec3::{Point3};

fn main() {
    // Basic config
    let output_file: &str = "output.png";
    let output_file_format = ImageFormat::Png;

    // Image config
    let aspect_ratio = 16.0/9.0;
    let image_width: u32 = 400;
    let image_height: u32 = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;
    let recursion_depth: u32 = 50;

    // Materials
    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left:   Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right:  Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // World
    let mut world = HittableList::new();
    let ground_sphere:  Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, &material_ground));
    let sphere1:        Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new( 0.0,  0.0,   -1.0), 0.5, &material_center));
    let sphere2:        Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(-1.0,  0.0,   -1.0), 0.5, &material_left));
    let sphere3:        Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new( 1.0,  0.0,   -1.0), 0.5, &material_right));

    world.add(&ground_sphere);
    world.add(&sphere1);
    world.add(&sphere2);
    world.add(&sphere3);

    // world is frozen as an Rc<dyn Hittable> until the render loop is over
    let world: Rc<dyn Hittable> = Rc::new(world);

    // Camera
    let cam = Camera::new();

    // Render
    let mut img = RgbImage::new(image_width, image_height);
    for x in 0..image_width {
        for y in 0..image_height {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = ((x as f64) + random_f64()) / (image_width as f64);  // Percentage of width for current pixel
                let v = ((y as f64) + random_f64()) / (image_height as f64); // Precentage of height for current pixel
                let r = cam.get_ray(u, v);
                pixel_color += r.color(&world, recursion_depth);
            }

            // Our coordinate system is right-handed and defines +y as up
            // While the Rust image crate is right-handed and defines +y as down
            // So we need to flip the y coordinates.
            write_pixel(&mut img, &pixel_color, x, image_height - y - 1, samples_per_pixel);
        }

        // Print progress on every 10th scanline
        println!("{}/{} scanlines rendered", x + 1, image_width);
    }
    img.save_with_format(output_file, output_file_format).unwrap();
}
