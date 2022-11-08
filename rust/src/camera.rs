use crate::vector::Vector;
use crate::point::Point;
use crate::ray::Ray;
use crate::utils::deg_to_rad;


pub struct Camera {
    pub look_from: Point,
    pub look_at: Point,
    pub v_up: Vector,
    
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub focal_length: f64,
    pub v_fov: i32,
    horizontal: Vector,
    vertical: Vector,
    lower_left_corner: Point,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        v_up: Vector,

        aspect_ratio: f64,
        focal_length: f64,
        v_fov: i32,
    ) -> Camera {
        let theta = deg_to_rad(v_fov);
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = v_up.cross(w).unit();
        let v = w.cross(u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = look_from - (horizontal / 2.0) - (vertical / 2.0) - w;

        return Camera {
            look_from,
            look_at,
            v_up,
            aspect_ratio,
            focal_length,
            viewport_height,
            horizontal,
            vertical,
            lower_left_corner,
            v_fov,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.look_from,
            self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.look_from,
        );
    }
}