mod vector;
mod point;
mod colour;
mod ray;
mod hittable;

use std::fs::File;
use std::io::prelude::*;
use indicatif::ProgressBar;
use vector::Vector;
use point::Point;
use colour::Colour;
use ray::Ray;
use hittable::{Environment, Sphere, Hit};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 256;
const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;
const IMAGES_DIR: &str = "images";
const OUTPUT_IMAGE: &str = "world";

fn write_file(path: &String, content: &String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    return Ok(());
}

fn hit_sphere(centre: Point, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - centre;
    let a = ray.direction.dot(ray.direction);
    let half_b = oc.dot(ray.direction);
    let c = oc.dot(oc) - radius.powf(2.0);
    let discriminant = half_b.powf(2.0) - (a * c);

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.powf(0.5)) / a;
    }
}

fn ray_colour(ray: &Ray, world: &Environment) -> Colour {
    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        return (Colour::new(1.0, 1.0, 1.0) + rec.normal.to_colour()) * 0.5;
    } else {
        let t = 0.5 * (ray.direction.unit().y() + 1.0);
        return {
            Colour::new(1.0, 1.0, 1.0) * (1.0 - t) +
            Colour::new(0.5, 0.7, 1.0) * t
        };
    };
}

fn main() {
    // Image
    let image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Camera
    let viewport_width = ASPECT_RATIO * VIEWPORT_HEIGHT;
    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vector::new(viewport_width, 0.0, 0.0);
    let vertical = Vector::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vector::new(0.0, 0.0, FOCAL_LENGTH);

    let fpath = format!("{}/{}.ppm", IMAGES_DIR, OUTPUT_IMAGE);
    let mut out = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, image_height);
    let bar = ProgressBar::new(image_height as u64);

    // World
    let mut world = Environment{ hittables: Vec::new() };
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    for j in (0..image_height).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);

            let ray = Ray::new(
                origin,
                lower_left_corner + (horizontal * u) + (vertical * v) - origin,
            );
            let pixel = ray_colour(&ray, &world);
            out.push_str(&format!("{}\n", pixel.to_str())[..]);
        }
        bar.inc(1);
    }
    write_file(&fpath, &out).expect("Failed when writing file.");
    bar.finish();
}
