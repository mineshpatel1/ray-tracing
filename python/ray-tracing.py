from __future__ import annotations

import os
import math
import time
from random import random

from camera import Camera
from ray import Ray
from hittable import Sphere, HittableList
from utils import log
from vector import (
    interpolate,
    random_in_unit_sphere,
    random_in_hemisphere,
    Colour, 
    Point3,
)

SRC_DIR = os.path.dirname(__file__)
IMAGE_DIR = os.path.join(SRC_DIR, 'images')

def ray_colour(ray: Ray, world: HittableList, depth: int = 10) -> Colour:
    if depth <= 0:
        return Colour(0, 0, 0)

    record = world.hit(ray, 0.001, math.inf)
    if record:
        target = record.p + random_in_hemisphere(record.normal)
        return ray_colour(Ray(record.p, target - record.p), world, depth - 1) * 0.5

    # Colour in the Sky
    t = 0.5 * (ray.unit_direction.y + 1)
    start_colour = Colour(1, 1, 1)
    end_colour = Colour(0.5, 0.7, 1.0)
    return interpolate(start_colour, end_colour, t)


def trace_rays():
    # Image
    fname = 'diffuse_material'
    aspect_ratio = 16 / 9
    image_width = 400
    image_height = int(image_width / aspect_ratio)
    antialias_samples = 100
    max_depth = 50

    # Camera
    cam = Camera(aspect_ratio)

    # World
    world = HittableList()
    world.append(Sphere(Point3(0, -100.5, -1), 100))
    world.append(Sphere(Point3(0, 0, -1), 0.5))  # Main Sphere

    # Render
    output = f"P3\n{image_width} {image_height}\n255\n"

    for j in range(image_height - 1, -1, -1):
        print(f"Lines remaining: {str(j).zfill(3)}\r", end="")
        for i in range(image_width):
            colour = Colour(0, 0, 0)
            for _ in range(antialias_samples):
                u = (i + random()) / (image_width - 1)
                v = (j + random()) / (image_height - 1)
                ray = cam.get_ray(u, v)
                colour += ray_colour(ray, world, max_depth)
            output += f"{colour.rgb(antialias_samples)}\n"

    path = os.path.join(IMAGE_DIR, f'{fname}.ppm')
    with open(path, 'w') as f:
        f.write(output)

    log.info(f'Written PPM image to {path}.')

if __name__ == '__main__':
    trace_rays()
