from random import random

from ray import Ray
from vector import dot, random_in_unit_sphere, refract, reflect, Colour
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


class Glass(Material):
    def __init__(self, eta: float):
        self.eta = eta

    @staticmethod
    def reflectance(cosine: float, ref_idx: float) -> float:
        """Schlick approximation for reflectivity."""
        r0 = (1 - ref_idx) / (1 + ref_idx)
        r0 = r0 ** 2
        return r0 + (1 - r0) * ((1 - cosine) ** 5)

    def scatter(
        self,
        ray_in: Ray,
        record: "HitRecord",
    ) -> Tuple[Optional[Ray], Optional[Colour]]:
        attenuation = Colour(1, 1, 1)
        refraction_ratio = 1 / self.eta if record.front_face else self.eta
        unit_direction = ray_in.direction.unit_vector
        cos_theta = min(dot(unit_direction.negative, record.normal), 1.0)
        sin_theta = (1 - (cos_theta ** 2)) ** 0.5

        cannot_refract = refraction_ratio * sin_theta > 1.0
        reflectance = self.reflectance(cos_theta, refraction_ratio)
        if cannot_refract or reflectance > random():  
            direction = reflect(unit_direction, record.normal)
        else:
            direction = refract(
                unit_direction,
                record.normal,
                refraction_ratio,
            )
        return Ray(record.p, direction), attenuation