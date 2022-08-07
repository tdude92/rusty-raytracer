use std::rc::Rc;

use image::{ImageFormat, RgbImage};

use rusty_raytracer::hittable::{Hittable, HittableList, Sphere};
use rusty_raytracer::ray::Ray;
use rusty_raytracer::vec3::{Point3, Vec3};

fn main() {
    // Basic config
    let output_file: &str = "output.png";
    let output_file_format = ImageFormat::Png;

    // Image config
    let aspect_ratio = 16.0/9.0;
    let image_width: u32 = 400;
    let image_height: u32 = ((image_width as f64) / aspect_ratio) as u32;

    // World
    let mut world = HittableList::new();
    let sphere1:        Rc<dyn Hittable> = Rc::new(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5));
    let ground_sphere:  Rc<dyn Hittable> = Rc::new(Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0));

    world.add(&sphere1);
    world.add(&ground_sphere);

    // world is frozen as an Rc<dyn Hittable> until the render loop is over
    let world: Rc<dyn Hittable> = Rc::new(world);

    // Camera config
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio*viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = &origin - &horizontal/2.0 - &vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    let mut img = RgbImage::new(image_width, image_height);
    for x in 0..image_width {
        for y in 0..image_height {
            let u = (x as f64) / ((image_width - 1) as f64);  // Percentage of width for current pixel
            let v = (y as f64) / ((image_height - 1) as f64); // Precentage of height for current pixel
            let r = Ray::new(origin, &lower_left_corner + &horizontal*u + &vertical*v - &origin);

            // Our coordinate system is right-handed and defines +y as up
            // While the Rust image crate is right-handed and defines +y as down
            // So we need to flip the y coordinates.
            let pixel_color = r.color(&world);
            img.put_pixel(x, image_height - y - 1, pixel_color);
        }

        // Print progress on every 10th scanline
        if (x + 1) % 10 == 0 {
            println!("{}/{} scanlines rendered", x + 1, image_width);
        }
    }
    img.save_with_format(output_file, output_file_format).unwrap();
}
