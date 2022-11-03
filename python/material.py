from ray import Ray
from vector import dot, random_in_unit_sphere, reflect, Colour
from typing import Optional, Tuple, TYPE_CHECKING

if TYPE_CHECKING:
    from hittable import HitRecord

class Material:
    def scatter(
        self,
        ray: Ray,
        record: "HitRecord",
    ) -> Tuple[Optional[Ray], Optional[Colour]]:
        raise NotImplementedError()


class Diffuse(Material):
    def __init__(self, albedo: Colour):
        self.albedo = albedo

    def scatter(
        self,
        ray_in: Ray,
        record: "HitRecord",
    ) -> Tuple[Optional[Ray], Optional[Colour]]:
        direction = record.normal + random_in_unit_sphere().unit_vector
        if direction.near_zero:
            direction = record.normal
        return Ray(record.p, direction), self.albedo


class Metal(Material):
    def __init__(self, albedo: Colour, fuzz: float = 0):
        self.albedo = albedo
        self.fuzz = fuzz if fuzz < 1 else 1
    
    def scatter(
        self,
        ray_in: Ray,
        record: "HitRecord",
    ) -> Tuple[Optional[Ray], Optional[Colour]]:
        reflected = reflect(ray_in.direction.unit_vector, record.normal)
        if dot(reflected, record.normal) > 0:
            return Ray(record.p, reflected + (random_in_unit_sphere() * self.fuzz)), self.albedo
        else:
            return None, None