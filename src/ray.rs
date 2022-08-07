use std::rc::Rc;

use image::Rgb;

use crate::INFINITY;
use crate::color::{Color, into_pixel};
use crate::vec3::{Point3, Vec3};
use crate::hittable::Hittable;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    dir: Vec3
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Ray {
            origin,
            dir,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + t * &self.dir
    }

    pub fn color(&self, world: &Rc<dyn Hittable>) -> Rgb<u8> {
        if let Some(hit_record) = world.hit(self, 0.0, INFINITY) {
            return Rgb(into_pixel(0.5*(hit_record.normal + 1.0)));
        }

        let unit_direction = self.dir.unit_vector();
        let t = 0.5*(unit_direction.y() + 1.0); // Normalize y to [0, 1]
        Rgb(into_pixel((1.0 - t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)))
    }
}
