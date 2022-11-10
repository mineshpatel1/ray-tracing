use std::sync::Arc;

use crate::hittable::{Hit, HitRecord};
use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;

pub struct Sphere {
    pub centre: Point,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(centre: Point, radius: f64, material: Arc<dyn Material>) -> Sphere {
        return Sphere {
            centre,
            radius,
            material,
        };
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius.powf(2.0);
        let discriminant = half_b.powf(2.0) - (a * c);

        if discriminant < 0.0 {
            return None;
        };
        let sqrtd = discriminant.powf(0.5);
        let mut t = (-half_b - sqrtd) / a;
        if t < t_min || t > t_max {
            t = (-half_b + sqrtd) / a;
            if t < t_min || t > t_max {
                return None;
            }
        }

        let p = ray.at(t);
        let normal = (p - self.centre) / self.radius;
        let rec = HitRecord::new_from_ray(p, normal, t, ray, self.material.clone());
        return Some(rec);
    }
}
