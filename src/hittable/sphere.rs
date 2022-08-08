use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable, into_opposing_normal};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: &Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material: Rc::clone(material),
        }
    }

    pub fn center(&self) -> &Point3 {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;

        let a = r.dir().length_squared();
        let half_b = Vec3::dot(r.dir(), &oc);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            // No hit
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || root > t_max {
                // - root is out of bounds
                root = (-half_b + sqrtd) / a;
                if root < t_min || root > t_max {
                    // + root is also out of bounds
                    return None;
                }
            }

            let hit_point = r.at(root);
            let outward_normal = (hit_point - self.center) / self.radius;
            let (front_face, normal) = into_opposing_normal(r, outward_normal);

            Some(HitRecord{
                p: hit_point,
                normal,
                material: Rc::clone(&self.material),
                t: root,
                front_face,
            })
        }
    }
}
