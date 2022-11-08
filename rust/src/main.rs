mod camera;
mod colour;
mod hittable;
mod material;
mod point;
mod ray;
mod sphere;
mod utils;
mod vector;

use std::time::Instant;
use indicatif::ProgressBar;
use rand::Rng;
use rayon::prelude::*;

use point::Point;
use colour::Colour;
use ray::Ray;
use hittable::{Environment, Hit};
use camera::Camera;
use material::{Diffuse, Glass, Metal};
use sphere::Sphere;
use utils::write_file;

use crate::vector::Vector;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
// const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;
const IMAGES_DIR: &str = "images";
const OUTPUT_IMAGE: &str = "camera";
const ANTIALIAS_SAMPLES: i64 = 25;
const MAX_DEPTH: i32 = 50;
const VFOV: i32 = 20;
const LOOK_FROM: Point = Point {v: Vector {xyz: [-2.0, 2.0, 1.0]}};
const LOOK_AT: Point = Point{v: Vector {xyz: [0.0, 0.0, -1.0]}};
const VUP: Vector = Vector{xyz: [0.0, 1.0, 0.0]};



fn ray_colour(ray: &Ray, world: &Environment, depth: i32) -> Colour {
    if depth <= 0 { return Colour::new(0.0, 0.0, 0.0); }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((scattered, colour)) = rec.material.scatter(ray, &rec) {
            return colour * ray_colour(&scattered, world, depth - 1);
        } else {
            return Colour::new(0.0, 0.0, 0.0);
        }
    } else {
        let t = 0.5 * (ray.direction.unit().y() + 1.0);
        let start = Colour::new(1.0, 1.0, 1.0);
        let end = Colour::new(0.5, 0.7, 1.0);
        return start.interpolate(end, t);
    };
}

fn main() {
    // Image
    let image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Camera
    let cam = Camera::new(
        LOOK_FROM, 
        LOOK_AT, 
        VUP, 
        ASPECT_RATIO,
        FOCAL_LENGTH,
        VFOV,
    );

    // World
    let mut world = Environment{ hittables: Vec::new() };
    let ground_mat = Diffuse::new(Colour::new(0.8, 0.8, 0.0));
    let centre_mat = Diffuse::new(Colour::new(0.7, 0.3, 0.3));
    // let left_mat = Metal::new(Colour::new(0.8, 0.8, 0.8), 0.2);
    let left_mat = Glass::new(1.5);
    let right_mat = Metal::new(Colour::new(0.8, 0.6, 0.2), 0.3);

    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ground_mat));
    world.add(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, left_mat));
    world.add(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, right_mat));
    world.add(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, centre_mat));

    // File
    println!("\nRendering...");
    let fpath = format!("{}/{}.ppm", IMAGES_DIR, OUTPUT_IMAGE);
    let mut out = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, image_height);
    let bar = ProgressBar::new(image_height as u64);
    let start = Instant::now();

    for j in (0..image_height).rev() {
        let pixels: Vec<Colour> = (0..IMAGE_WIDTH).into_par_iter()
            .map(|i| {
                let mut rng = rand::thread_rng();
                let mut pixel = Colour::new(0.0, 0.0, 0.0);
                for _ in 0..ANTIALIAS_SAMPLES {
                    let u_r: f64 = rng.gen();
                    let v_r: f64 = rng.gen();
                    let u = ((i as f64) + u_r) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + v_r) / ((image_height - 1) as f64);
                    let ray = cam.get_ray(u, v);
                    pixel += ray_colour(&ray, &world, MAX_DEPTH);
                }
                return pixel;
            })
            .collect();

        for pixel in pixels.into_iter() {
            out.push_str(&format!("{}\n", pixel.render(ANTIALIAS_SAMPLES))[..]);
        }
        bar.inc(1);
    }
    write_file(&fpath, &out).expect("Failed when writing file.");
    bar.finish();
    let end = Instant::now();
    println!("Render took: {}s", ((end - start).as_millis() as f64 / 1000.0));
}
