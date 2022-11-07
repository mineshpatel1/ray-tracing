use crate::vector::Vector;
use crate::point::Point;
use crate::ray::Ray;


pub struct Camera {
    pub origin: Point,
    pub aspect_ratio: f64,
    pub focal_length: f64,
    pub viewport_height: f64,
    viewport_width: f64,
    horizontal: Vector,
    vertical: Vector,
    lower_left_corner: Point,
}

impl Camera {
    pub fn new(
        origin: Point,
        aspect_ratio: f64,
        viewport_height: f64,
        focal_length: f64,
    ) -> Camera {
        let viewport_width = aspect_ratio * viewport_height; 
        let horizontal = Vector::new(viewport_width, 0.0, 0.0);
        let vertical = Vector::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vector::new(0.0, 0.0, focal_length);

        return Camera {
            origin,
            aspect_ratio,
            focal_length,
            viewport_height,
            viewport_width,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin,
        );
    }
}