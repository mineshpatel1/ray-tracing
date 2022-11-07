use std::sync::Arc;

use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::{random_in_unit_sphere, reflect};

pub trait Material: Send + Sync {
    fn scatter (&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Colour)>;
}

pub struct Diffuse {
    pub colour: Colour,
}

impl Diffuse {
    pub fn new(colour: Colour) -> Arc<Diffuse> {
        return Arc::new(Diffuse{ colour });
    }
}

impl Material for Diffuse {
    fn scatter (&self, _ray: &Ray, record: &HitRecord) -> Option<(Ray, Colour)> {
        let mut scatter_direction = record.normal + random_in_unit_sphere().unit();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        let scattered = Ray::new(record.p, scatter_direction);
        return Some((scattered, self.colour));
    }
}

pub struct Metal {
    pub colour: Colour,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(colour: Colour, fuzz: f64) -> Arc<Metal> {
        return Arc::new(Metal{ colour, fuzz });
    }
}

impl Material for Metal {
    fn scatter (&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Colour)> {
        let reflected = reflect(ray.direction, record.normal);
        let scattered = Ray::new(record.p, reflected + (random_in_unit_sphere() * self.fuzz));

        if reflected.dot(record.normal) > 0.0 {
            return Some((scattered, self.colour));
        } else {
            return None;
        }
    }
}