from __future__ import annotations

import os
import math
from typing import Callable

from utils import log
from ray import Ray
from hittable import Sphere, HitRecord, HittableList
from vector import (
    dot,
    interpolate,
    Colour, 
    Point3,
    Vector,
)

SRC_DIR = os.path.dirname(__file__)
IMAGE_DIR = os.path.join(SRC_DIR, 'images')


def sample_ppm():
    write_ppm_file(
        256, 256,
        lambda u, v: Colour(u, v, 0.8),
        'sample',
    )


def hit_sphere(
    centre: Point3,
    radius: float,
    ray: Ray,
) -> float:
    # Function describing a ray hitting a sphere:
    # (b.b)t^2 + 2b(A-C)t + (A-C)(A-C) - r^2 = 0
    # Quadratic, can solve with quadratic formula
    o_c = ray.origin - centre
    a = ray.direction.length_squared
    h = dot(ray.direction, o_c)
    c = o_c.length_squared - (radius ** 2)

    discriminant = (h ** 2) - (a * c)
    if discriminant < 0:
        return -1
    else:
        return (-h - (discriminant ** 0.5)) / a


def write_ppm_file(
    width: int,
    height: int,
    colour_function: Callable[[float, float], Colour],
    fname: str,
):
    output = f"P3\n{width} {height}\n255\n"

    for j in range(height - 1, -1, -1):
        log.info(f"Lines remaining: {j}")
        for i in range(width):
            u = i / (width - 1)
            v = j / (height - 1)
            colour = colour_function(u, v)
            output += f"{colour.rgb}\n"

    path = os.path.join(IMAGE_DIR, f'{fname}.ppm')
    with open(path, 'w') as f:
        f.write(output)

    log.info(f'Written PPM image to {path}.')


def trace_rays():
    # Image
    aspect_ratio = 16 / 9
    image_width = 400
    image_height = int(image_width / aspect_ratio)

    # Camera
    viewport_height = 2
    viewport_width = viewport_height * aspect_ratio
    focal_length = 1

    origin = Point3(0, 0, 0)
    horizontal = Vector(viewport_width, 0, 0)
    vertical = Vector(0, viewport_height, 0)
    lower_left_corner = origin - (horizontal / 2) - (vertical / 2) - Vector(0, 0, focal_length)

    # World
    world = HittableList()
    world.append(Sphere(Point3(0, -100.5, -1), 100))
    world.append(Sphere(Point3(0, 0, -1), 0.5))  # Main Sphere

    # Render
    def _gen_colour_from_ray(u, v):
        centre = Point3(0, 0, -1)
        ray = Ray(origin, lower_left_corner + (horizontal * u) + (vertical * v))

        record = world.hit(ray, 0, math.pi)
        if record:
            return (record.normal + Colour(1, 1, 1)) * 0.5

        # Colour in the Sky
        t = 0.5 * (ray.unit_direction.y + 1)
        start_colour = Colour(1, 1, 1)
        end_colour = Colour(0.5, 0.7, 1.0)
        return interpolate(start_colour, end_colour, t)

    write_ppm_file(
        image_width,
        image_height,
        _gen_colour_from_ray,
        'sphere_class',
    )

if __name__ == '__main__':
    trace_rays()
