from __future__ import annotations

import os
from typing import Callable

from utils import log
from ray import Ray
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


def hit_sphere(
    centre: Point3,
    radius: float,
    ray: Ray,
) -> bool:
    # Function describing a ray hitting a sphere:
    # (b.b)t^2 + 2b(A-C)t + (A-C)(A-C) - r^2 = 0
    # Quadratic, can solve with quadratic formula
    oc = ray.origin - centre
    a = dot(ray.direction, ray.direction)
    b = dot(ray.direction, oc) * 2
    c = dot(oc, oc) - (radius ** 2)

    discriminant = (b ** 2) - (4 * a * c)
    return discriminant > 0


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

    # Render
    def _gen_colour_from_ray(u, v):
        ray = Ray(origin, lower_left_corner + (horizontal * u) + (vertical * v))
        t = 0.5 * (ray.unit_direction.y + 1)

        centre = Point3(0, 0, -1)
        if hit_sphere(centre, 0.5, ray):
            return Colour(1.0, 0.22, 0.22)

        start_colour = Colour(1, 1, 1)
        end_colour = Colour(0.28, 0.62, 0.7)
        return interpolate(start_colour, end_colour, t)

    write_ppm_file(
        image_width,
        image_height,
        _gen_colour_from_ray,
        'first_sphere',
    )

if __name__ == '__main__':
    trace_rays()