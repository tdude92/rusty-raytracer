use image::{ImageFormat, RgbImage, Rgb};

fn main() {
    let mut img = RgbImage::new(32, 32);
    for x in 0u8..32 {
        for y in 0u8..32 {
            img.put_pixel(x as u32, y as u32, Rgb([0, x*8, y*8]));
        }
    }
    img.save_with_format("test.png", ImageFormat::Png).unwrap();
    println!("Hello, world!");
}
