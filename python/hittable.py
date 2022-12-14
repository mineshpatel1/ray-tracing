from dataclasses import dataclass
from typing import List, Optional

from material import Material
from ray import Ray
from vector import dot, Point3, Vector


@dataclass
class HitRecord:
    p: Point3
    normal: Vector
    t: float
    material: Material
    front_face: bool = False

    def set_face_normal(self, ray: Ray, outward_normal: Vector):
        self.front_face = dot(ray.direction, outward_normal) < 0
        self.normal = outward_normal if self.front_face else outward_normal.negative


@dataclass
class Hittable:
    def hit(self, ray: Ray, t_min: float, t_max: float, record: HitRecord) -> bool:
        raise NotImplementedError()


@dataclass
class HittableList:
    def __init__(self):
        self.hittables: List[Hittable] = []

    def append(self, hittable: Hittable):
        self.hittables.append(hittable)

    def clear(self):
        self.hittables = []

    def hit(self, ray: Ray, t_min: float, t_max: float) -> Optional[HitRecord]:
        record = None
        closest_so_far = t_max
        for hittable in self.hittables:
            temp_record = hittable.hit(ray, t_min, closest_so_far)
            if temp_record:
                record = temp_record
                closest_so_far = temp_record.t
        return record


class Sphere(Hittable):
    def __init__(
        self,
        centre: Point3,
        radius: float,
        material: Material,
    ):
        self.centre = centre
        self.radius = radius
        self.material = material
        super(Sphere, self).__init__()

    def hit(self, ray: Ray, t_min: float, t_max: float) -> Optional[HitRecord]:
        o_c = ray.origin - self.centre
        a = ray.direction.length_squared
        h = dot(o_c, ray.direction)
        c = o_c.length_squared - (self.radius ** 2)

        discriminant = (h ** 2) - (a * c)
        if discriminant < 0:
            return None

        sqrtd =  discriminant ** 0.5
        t = (-h - sqrtd) / a
        if t < t_min or t_max < t:
            t = (-h + sqrtd) / a
            if t < t_min or t_max < t:
                return None

        p = ray.at(t)
        outward_normal = (p - self.centre) / self.radius
        record = HitRecord(p, outward_normal, t, self.material)
        record.set_face_normal(ray, outward_normal)
        return record