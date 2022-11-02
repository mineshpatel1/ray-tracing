from ray import Ray
from vector import Point3, Vector

class Camera:
    def __init__(
        self,
        aspect_ratio: float = 16 / 9,
        viewport_height: float = 2.0,
        focal_length: float = 1.0,
    ):
        self.aspect_ratio = aspect_ratio
        self.viewport_height = viewport_height
        self.focal_length = focal_length
        self.viewport_width: float = self.aspect_ratio * self.viewport_height

        self.origin: Point3 = Point3(0, 0, 0)
        self.horizontal: Vector = Vector(self.viewport_width, 0, 0)
        self.vertical: Vector = Vector(0, self.viewport_height, 0)
        self.lower_left_corner: Vector = self.origin - (self.horizontal / 2) - (self.vertical / 2) - Vector(0, 0, self.focal_length)
    
    def get_ray(self, u: float, v: float) -> Ray:
        return Ray(self.origin, self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin)

