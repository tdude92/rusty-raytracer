use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    dir: Vec3
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Ray {
            origin: origin,
            dir: dir,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin.clone()
    }

    pub fn dir(&self) -> Vec3 {
        self.dir.clone()
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + t * &self.dir
    }
}
