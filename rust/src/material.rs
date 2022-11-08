use rand::Rng;
use std::sync::Arc;

use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::random_in_unit_sphere;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Colour)>;
}

pub struct Diffuse {
    pub colour: Colour,
}

impl Diffuse {
    pub fn new(colour: Colour) -> Arc<Diffuse> {
        return Arc::new(Diffuse { colour });
    }
}

impl Material for Diffuse {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Ray, Colour)> {
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
        return Arc::new(Metal { colour, fuzz });
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Colour)> {
        let reflected = ray.direction.reflect(record.normal);
        let scattered = Ray::new(record.p, reflected + (random_in_unit_sphere() * self.fuzz));

        if reflected.dot(record.normal) > 0.0 {
            return Some((scattered, self.colour));
        } else {
            return None;
        }
    }
}

pub struct Glass {
    pub refractive_idx: f64,
}

impl Glass {
    pub fn new(refractive_idx: f64) -> Arc<Glass> {
        return Arc::new(Glass { refractive_idx });
    }

    fn reflectance(cosine: f64, refractive_idx: f64) -> f64 {
        let r0 = ((1.0 - refractive_idx) / (1.0 + refractive_idx)).powi(2);
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Glass {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Colour)> {
        let unit_direction = ray.direction.unit();

        let cos_theta = (-unit_direction).dot(record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).powf(0.5);
        let refractive_idx = if record.front_face {
            1.0 / self.refractive_idx
        } else {
            self.refractive_idx
        };

        let cannot_refract = refractive_idx * sin_theta > 1.0;
        let will_reflect = Self::reflectance(cos_theta, refractive_idx) > rand::thread_rng().gen();

        let direction = if cannot_refract || will_reflect {
            // Reflect
            unit_direction.reflect(record.normal)
        } else {
            // Refract
            unit_direction.refract(record.normal, refractive_idx)
        };

        return Some((Ray::new(record.p, direction), Colour::new(1.0, 1.0, 1.0)));
    }
}
