use crate::vec3::Vec3;
use crate::util::clamp;

pub type Color = Vec3;

/// Converts a Color vector with RGB randing [0, 1] to an array of u8
pub fn rgb(pixel_color: &Color) -> [u8; 3] {
    [
        clamp(pixel_color.x()*255.0, 0.0, 255.0).trunc() as u8,
        clamp(pixel_color.y()*255.0, 0.0, 255.0).trunc() as u8,
        clamp(pixel_color.z()*255.0, 0.0, 255.0).trunc() as u8,
    ]
}
