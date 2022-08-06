use image::{ImageFormat, RgbImage, Rgb};
use rusty_raytracer::color::{Color, rgb};

fn main() {
    let mut img = RgbImage::new(32, 32);
    for x in 0u8..32 {
        for y in 0u8..32 {
            let p = rgb(&Color::new((x as f64)/31.0, (y as f64)/31.0, 0.0));
            img.put_pixel(x as u32, y as u32, Rgb(p));
        }
        println!("Raytraced {}/{} scanlines.", x + 1, 32);
    }
    img.save_with_format("test.png", ImageFormat::Png).unwrap();
}
