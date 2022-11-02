from __future__ import annotations
from asyncore import write

import os
from utils import log
from vector import Colour, Vector, dot

SRC_DIR = os.path.dirname(__file__)
IMAGE_DIR = os.path.join(SRC_DIR, 'images')
IMAGE_WIDTH = 256
IMAGE_HEIGHT = 256

def write_ppm_file():
    output = f"P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n"

    for j in range(IMAGE_HEIGHT - 1, -1, -1):
        log.debug(f"Lines remaining: {j}")
        for i in range(IMAGE_WIDTH):
            r = i / (IMAGE_WIDTH)
            g = j / (IMAGE_HEIGHT)
            b = 0.8
            output += f"{Colour(r, g, b).rgb}\n"

    path = os.path.join(IMAGE_DIR, 'sample.ppm')
    with open(path, 'w') as f:
        f.write(output)

    log.info(f'Written PPM image to {path}.')


if __name__ == '__main__':
    write_ppm_file()