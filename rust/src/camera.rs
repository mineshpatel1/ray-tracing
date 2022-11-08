use crate::vector::{Vector, random_in_unit_disk};
use crate::point::Point;
use crate::ray::Ray;
use crate::utils::deg_to_rad;


pub struct Camera {
    look_from: Point,
    horizontal: Vector,
    vertical: Vector,
    lower_left_corner: Point,
    aperture: f64,
    u: Vector,
    v: Vector,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        v_up: Vector,
        aspect_ratio: f64,
        v_fov: i32,
        aperture: f64,
    ) -> Camera {
        let theta = deg_to_rad(v_fov);
        let focus_distance = (look_from - look_at).length();
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = v_up.cross(w).unit();
        let v = w.cross(u);

        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = look_from - (horizontal / 2.0) - (vertical / 2.0) - (focus_distance * w);

        return Camera {
            look_from,
            horizontal,
            vertical,
            lower_left_corner,
            aperture,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = (self.aperture / 2.0) * random_in_unit_disk();
        let offset = (self.u * rd.x()) + (self.v * rd.y());

        return Ray::new(
            self.look_from + offset,
            self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - self.look_from - offset,
        );
    }
}