use std::rc::Rc;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    v: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {v: vec![]}
    }

    pub fn add(&mut self, object: &Rc<dyn Hittable>) {
        self.v.push(Rc::clone(object));
    }

    pub fn clear(&mut self) {
        self.v.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut nearest_hit: Option<HitRecord> = None;
        for object in self.v.iter() {
            match nearest_hit {
                Some(ref nearest_hit_record) => {
                    let hit = object.hit(r, t_min, nearest_hit_record.t);
                    if let Some(ref _hit_record) = hit {nearest_hit = hit;}
                },
                None => {
                    nearest_hit = object.hit(r, t_min, t_max);
                },
            }
        }

        nearest_hit
    }
}
