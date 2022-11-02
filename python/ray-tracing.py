from __future__ import annotations

import os
import math
from random import random

from camera import Camera
from ray import Ray
from hittable import Sphere, HittableList
from utils import log
from vector import (
    interpolate,
    Colour, 
    Point3,
)

SRC_DIR = os.path.dirname(__file__)
IMAGE_DIR = os.path.join(SRC_DIR, 'images')
FNAME = 'antialiasing'

def ray_colour(ray: Ray, world: HittableList) -> Colour:
    record = world.hit(ray, 0, math.pi)
    if record:
        return (record.normal + Colour(1, 1, 1)) * 0.5

    # Colour in the Sky
    t = 0.5 * (ray.unit_direction.y + 1)
    start_colour = Colour(1, 1, 1)
    end_colour = Colour(0.5, 0.7, 1.0)
    return interpolate(start_colour, end_colour, t)


def trace_rays():
    # Image
    aspect_ratio = 16 / 9
    image_width = 400
    image_height = int(image_width / aspect_ratio)
    samples = 100

    # Camera
    cam = Camera(aspect_ratio)

    # World
    world = HittableList()
    world.append(Sphere(Point3(0, -100.5, -1), 100))
    world.append(Sphere(Point3(0, 0, -1), 0.5))  # Main Sphere

    # Render
    output = f"P3\n{image_width} {image_height}\n255\n"

    for j in range(image_height - 1, -1, -1):
        log.info(f"Lines remaining: {j}")
        for i in range(image_width):
            colour = Colour(0, 0, 0)
            for _ in range(samples):
                u = (i + random()) / (image_width - 1)
                v = (j + random()) / (image_height - 1)
                ray = cam.get_ray(u, v)
                colour += ray_colour(ray, world)
            output += f"{colour.rgb(samples)}\n"

    path = os.path.join(IMAGE_DIR, f'{FNAME}.ppm')
    with open(path, 'w') as f:
        f.write(output)

    log.info(f'Written PPM image to {path}.')

if __name__ == '__main__':
    trace_rays()
