from __future__ import annotations
from dataclasses import dataclass
from typing import Union

Number = Union[int, float]

@dataclass
class Vector:
    def __init__(self, x: Number, y: Number, z: Number):
        self.coords = (float(x), float(y), float(z))

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
    def rgb(self) -> str:
        factor = 255.99
        ir = int(self.x * factor)
        ig = int(self.y * factor)
        ib = int(self.z * factor)
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


def interpolate(
    start_colour: Colour,
    end_colour: Colour,
    t: float,
) -> Colour:
    return (start_colour * (1 - t)) + (end_colour * t)
