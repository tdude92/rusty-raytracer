use image::Rgb;

use crate::vec3::Vec3;
use crate::clamp;

pub type Color = Vec3;

/// Converts a Color vector with RGB randing [0, 1] to an array of u8
pub fn into_pixel(pixel_color: &Color) -> Rgb<u8> {
    Rgb([
        clamp(pixel_color.x()*255.0, 0.0, 255.0).trunc() as u8,
        clamp(pixel_color.y()*255.0, 0.0, 255.0).trunc() as u8,
        clamp(pixel_color.z()*255.0, 0.0, 255.0).trunc() as u8,
    ])
}
