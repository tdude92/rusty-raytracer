use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - &self.center;

        let a = r.dir().length_squared();
        let half_b = Vec3::dot(r.dir(), &oc);
        let c = oc.length_squared() - (&self.radius)*(&self.radius);

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
            Some(HitRecord{
                p: hit_point,
                normal: (hit_point - &self.center) / self.radius,
                t: root,
            })
        }
    }
}
