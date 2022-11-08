use crate::point::Point;
use crate::vector::Vector;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        return Ray { origin, direction };
    }

    pub fn at(&self, t: f64) -> Point {
        return self.origin + (self.direction * t);
    }
}
