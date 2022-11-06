use crate::vector::Vector;
use crate::point::Point;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    fn new_from_ray(p: Point, normal: Vector, t: f64, ray: &Ray) -> HitRecord {
        let mut rec = HitRecord {p, normal, t, front_face: false};
        rec.set_face_normal(ray, normal);
        return rec;
    }

    fn set_face_normal(&mut self, ray: &Ray, normal: Vector) {
        self.front_face = ray.direction.dot(normal) < 0.0;
        self.normal = if self.front_face {normal} else {-normal};
    }
}

pub trait Hit {
    fn hit (&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Environment {
    pub hittables:Vec<Box<dyn Hit>>,
}

impl Environment {
    pub fn add(&mut self, hittable: Box<dyn Hit>) {
        self.hittables.push(hittable);
    }
}

impl Hit for Environment {
    fn hit (&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut out: Option<HitRecord> = None;
        for hittable in self.hittables.iter().rev() {
            if let Some(temp_record) = hittable.hit(ray, t_min, t_max) {
                out = Some(temp_record);
            }
        }
        return out;
    }
}

pub struct Sphere {
    centre: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(centre: Point, radius: f64) -> Sphere {
        return Sphere {centre, radius};
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius.powf(2.0);
        let discriminant = half_b.powf(2.0) - (a * c);

        if discriminant < 0.0 { return None };
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
        let rec = HitRecord::new_from_ray(p, normal, t, ray);
        return Some(rec);
    }
}