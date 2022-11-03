from __future__ import annotations

from dataclasses import dataclass
from random import random, uniform
from typing import Optional, Union

from utils import clamp, log

Number = Union[int, float]

@dataclass
class Vector:
    def __init__(self, x: Number, y: Number, z: Number):
        self.coords = (float(x), float(y), float(z))

    @staticmethod
    def random() -> Vector:
        return Vector(random(), random(), random())

    @staticmethod
    def uniform(min_val: float, max_val: float) -> Vector:
        return Vector(
            uniform(min_val, max_val),
            uniform(min_val, max_val),
            uniform(min_val, max_val),
        )

    @property
    def x(self) -> float:
        return self.coords[0]

    @property
    def y(self) -> float:
        return self.coords[1]

    @property
    def z(self) -> float:
        return self.coords[2]

    @property
    def length(self) -> float:
        return self.length_squared ** 0.5

    @property
    def length_squared(self) -> float:
        return (self.x ** 2) + (self.y ** 2) + (self.z ** 2)

    @property
    def negative(self) -> Vector:
        return Vector(
            self.x * -1,
            self.y * -1,
            self.z * -1,
        )

    @property
    def unit_vector(self) -> Vector:
        return self / self.length

    @property
    def near_zero(self) -> bool:
        s = 1e-8
        return abs(self.x) < s and abs(self.y) < s and abs(self.z) < s

    def rgb(self, samples: int) -> str:
        scale = 1.0 / samples  # Divide by the number of samples for anti-aliasing
        
        # Gamma correct for Gamma-2 by taking the square root 
        r = (self.x * scale) ** 0.5
        g = (self.y * scale) ** 0.5
        b = (self.z * scale) ** 0.5

        ir = int(256 * clamp(r, 0, 0.999))
        ig = int(256 * clamp(g, 0, 0.999))
        ib = int(256 * clamp(b, 0, 0.999))
        return f"{ir} {ig} {ib}"

    def __add__(self, o: Union[Vector, Number]) -> Vector:
        if not isinstance(o, Vector):
            raise TypeError(f"Cannot add Vector with {type(o)}")

        if isinstance(o, Vector):
            return Vector(
                self.x + o.x,
                self.y + o.y,
                self.z + o.z,
            )
        elif isinstance(o, (int, float)):
            return Vector(
                self.x + o,
                self.y + o,
                self.z + o,
            )
        else:
            raise TypeError(f"Cannot add Vector with {type(o)}")

    def __sub__(self, o: Vector) -> Vector:
        if isinstance(o, Vector):
            return self + o.negative
        else:
            raise TypeError(f"Cannot subtract Vector with {type(o)}")

    def __mul__(self, o: Union[Vector, Number]) -> Vector:
        if isinstance(o, Vector):
            return Vector(
                self.x * o.x,
                self.y * o.y,
                self.z * o.z,
            )
        elif isinstance(o, (int, float)):
            return Vector(
                self.x * o,
                self.y * o,
                self.z * o,
            )
        else:
            raise TypeError(f"Cannot multiply Vector with {type(o)}")

    def __truediv__(self, o: Number) -> Vector:
        if isinstance(o, (int, float)):
            return self * (1 / o)
        else:
            raise TypeError(f"Cannot divide Vector with {type(o)}")

    def __getitem__(self, i):
        return self.coords[i]

    def __str__(self):
        return f"Vector({self.x}, {self.y}, {self.z})"


Point3 = Vector
Colour = Vector


def dot(x: Vector, y: Vector) -> float:
    prod = x * y
    return prod.x + prod.y + prod.z


def cross(a: Vector, b: Vector) -> Vector:
    return Vector(
        (a.y * b.z) - (a.z * b.y),
        (a.z * b.x) - (a.x * b.z),
        (a.x * b.y) - (a.y * b.x),
    )


def reflect(v: Vector, n: Vector) -> Vector:
    return v - (n * (2 * dot(v, n)))


def refract(v: Vector, n: Vector, etai_over_etat: float) -> Vector:
    cos_theta = min(dot(v.negative, n), 1.0)
    r_out_perp = (v + (n * cos_theta)) * etai_over_etat
    r_out_parallel = n * -((abs(1 - r_out_perp.length_squared)) ** 0.5)
    return r_out_perp + r_out_parallel


def interpolate(
    start_colour: Colour,
    end_colour: Colour,
    t: float,
) -> Colour:
    return (start_colour * (1 - t)) + (end_colour * t)


def random_in_unit_sphere() -> Vector:
    while True:
        vec = Vector.random()
        if vec.length_squared >= 1:
            continue
        else:
            return vec


def random_in_hemisphere(normal: Vector) -> Vector:
    in_unit_sphere = random_in_unit_sphere().unit_vector
    if dot(in_unit_sphere, normal) > 0.0:
        return in_unit_sphere
    else:
        return in_unit_sphere.negative

