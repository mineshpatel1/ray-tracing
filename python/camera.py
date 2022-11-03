import math
from ray import Ray
from utils import deg_to_rad
from vector import cross, Point3, Vector

class Camera:
    def __init__(
        self,
        look_from: Point3,
        look_at: Point3,
        v_up: Vector,
        v_field_of_view: float,
        aspect_ratio: float = 16 / 9,
        focal_length: float = 1.0,
    ):
        self.look_from = look_from
        self.look_at = look_at
        self.v_up = v_up
        self.v_field_of_view = float(v_field_of_view)
        self.aspect_ratio = float(aspect_ratio)
        self.focal_length = float(focal_length)

        self.theta: float = deg_to_rad(self.v_field_of_view)
        self.h: float = math.tan(self.theta / 2)
        self.viewport_height: float = 2 * self.h
        self.viewport_width: float = self.aspect_ratio * self.viewport_height
        
        self.w = (self.look_from - self.look_at).unit_vector
        self.u = cross(self.v_up, self.w).unit_vector
        self.v = cross(self.w, self.u)

        self.origin: Point3 = self.look_from
        self.horizontal: Vector = self.u * self.viewport_width
        self.vertical: Vector = self.v * self.viewport_height
        self.lower_left_corner: Vector = self.origin - (self.horizontal / 2) - (self.vertical / 2) - self.w
    
    def get_ray(self, s: float, t: float) -> Ray:
        return Ray(self.origin, self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - self.origin)

