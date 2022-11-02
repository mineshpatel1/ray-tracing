from dataclasses import dataclass
from vector import Point3, Vector

@dataclass
class Ray:
    """
    Describes a ray: P(t) = A + bt
    where A is the origin and b is the direction.
    """
    origin: Point3
    direction: Vector

    @property
    def unit_direction(self) -> Vector:
        return self.direction.unit_vector

    def point_at(self, t: float) -> Point3:
        self.origin + (self.direction * t)
