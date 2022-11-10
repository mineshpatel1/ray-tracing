use std::sync::Arc;

use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new_from_ray(
        p: Point,
        normal: Vector,
        t: f64,
        ray: &Ray,
        material: Arc<dyn Material>,
    ) -> HitRecord {
        let mut rec = HitRecord {
            p,
            normal,
            t,
            material,
            front_face: false,
        };
        rec.set_face_normal(ray, normal);
        return rec;
    }

    fn set_face_normal(&mut self, ray: &Ray, normal: Vector) {
        self.front_face = ray.direction.dot(normal) < 0.0;
        self.normal = if self.front_face { normal } else { -normal };
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Environment {
    pub hittables: Vec<Box<dyn Hit>>,
}

impl Environment {
    pub fn add(&mut self, hittable: impl Hit + 'static) {
        self.hittables.push(Box::new(hittable));
    }
}

impl Hit for Environment {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut out: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for hittable in self.hittables.iter() {
            if let Some(temp_record) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_record.t;
                out = Some(temp_record);
            }
        }
        return out;
    }
}
