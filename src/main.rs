use image::{ImageFormat, RgbImage};

use rusty_raytracer::{write_pixel, random_f64};
use rusty_raytracer::color::Color;

fn main() {
    // Basic config
    let output_file: &str = "output.png";
    let output_file_format = ImageFormat::Png;

    // Load a scene
    let scene = rusty_raytracer::scene::dielectric_lambertian_metal::get_scene();

    // Pull relevant data out of the Scene object
    let image_width = scene.image_width;
    let image_height = scene.image_height;
    let samples_per_pixel = scene.samples_per_pixel;
    let recursion_depth = scene.recursion_depth;
    let cam = scene.cam;
    let world = scene.world;

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
