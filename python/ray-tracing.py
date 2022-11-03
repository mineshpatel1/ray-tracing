from __future__ import annotations

import os
import math
import multiprocessing
from random import random

from camera import Camera
from ray import Ray
from hittable import Sphere, HittableList
from material import Diffuse, Glass, Metal
from utils import log
from vector import (
    interpolate,
    Colour, 
    Point3,
    Vector,
)

SRC_DIR = os.path.dirname(__file__)
IMAGE_DIR = os.path.join(SRC_DIR, 'images')
BLACK = Colour(0, 0, 0)
NUM_CORES = multiprocessing.cpu_count()

def ray_colour(ray: Ray, world: HittableList, depth: int = 10) -> Colour:
    if depth <= 0:
        return BLACK

    record = world.hit(ray, 0.001, math.inf)
    if record:        
        scattered, attenuation = record.material.scatter(ray, record)
        if ray:
            return attenuation * ray_colour(scattered, world, depth - 1)
        else:
            return BLACK

    # Colour in the Sky
    t = 0.5 * (ray.unit_direction.y + 1)
    start_colour = Colour(1, 1, 1)
    end_colour = Colour(0.5, 0.7, 1.0)
    return interpolate(start_colour, end_colour, t)


def process_line(
    j: int,
    image_width: int,
    image_height: int,
    antialias_samples: int,
    max_depth: int,
    cam: Camera,
    world: HittableList,
) -> str:
    output = ""
    for i in range(image_width):
        colour = Colour(0, 0, 0)
        for _ in range(antialias_samples):
            u = (i + random()) / (image_width - 1)
            v = (j + random()) / (image_height - 1)
            ray = cam.get_ray(u, v)
            colour += ray_colour(ray, world, max_depth)
        output += f"{colour.rgb(antialias_samples)}\n"
    return j, output

def trace_rays():
    # Image
    fname = 'parallel'
    aspect_ratio = 16 / 9
    image_width = 400
    image_height = int(image_width / aspect_ratio)
    antialias_samples = 10
    max_depth = 50
    look_at = Point3(3, 3, 2)
    look_from = Point3(0, 0, -1)
    focus_distance = (look_from - look_at).length

    # Camera
    cam = Camera(
        look_at,
        look_from,
        Vector(0, 1, 0),
        20,
        aspect_ratio,
        0.01,
        focus_distance,
    )

    # World
    world = HittableList()
    material_ground = Diffuse(Colour(0.8, 0.8, 0))
    material_centre = Diffuse(Colour(0.1, 0.2, 0.5))
    material_left = Glass(1.5)
    material_right = Metal(Colour(0.8, 0.6, 0.2))

    world.append(Sphere(Point3(0.0, -100.5, -1.0), 100, material_ground))  # Ground
    world.append(Sphere(Point3(-1, 0, -1), 0.5, material_left)) # Left Sphere
    world.append(Sphere(Point3(0, 0, -1), 0.5, material_centre)) # Centre Sphere
    world.append(Sphere(Point3(1, 0, -1), 0.5, material_right)) # Right Sphere

    # Render
    output = f"P3\n{image_width} {image_height}\n255\n"
    items = []

    log.info("Starting render...")
    for j in range(image_height - 1, -1, -1):
        items.append((
            j,
            image_width,
            image_height,
            antialias_samples,
            max_depth,
            cam,
            world,
        ))

    # tasks_done = 0
    def progress(result):
        j, _ = result
        n = len(items)
        if j % math.floor(n / 100) == 0:
            print(f".", end='', flush=True)

    with multiprocessing.Pool(NUM_CORES) as pool:
        results = [
            pool.apply_async(
                func=process_line,
                args=args,
                callback=progress,
            ) for args in items
        ]

        pool.close()
        pool.join()

        for result in results:
            j, line = result.get()
            output += line

    path = os.path.join(IMAGE_DIR, f'{fname}.ppm')
    with open(path, 'w') as f:
        f.write(output)

    log.newline()
    log.info(f'Written PPM image to {path}.')

if __name__ == '__main__':
    trace_rays()
