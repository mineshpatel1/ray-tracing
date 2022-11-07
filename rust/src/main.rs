mod vector;
mod point;
mod colour;
mod ray;
mod hittable;
mod camera;

use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;
use indicatif::ProgressBar;
use rand::Rng;
use rayon::prelude::*;

use vector::random_in_unit_sphere;
use point::Point;
use colour::Colour;
use ray::Ray;
use hittable::{Environment, Sphere, Hit};
use camera::Camera;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;
const IMAGES_DIR: &str = "images";
const OUTPUT_IMAGE: &str = "diffuse";
const ANTIALIAS_SAMPLES: i64 = 100;
const MAX_DEPTH: i32 = 50;

fn write_file(path: &String, content: &String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    return Ok(());
}

fn ray_colour(ray: &Ray, world: &Environment, depth: i32) -> Colour {
    if depth <= 0 { return Colour::new(0.0, 0.0, 0.0); }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        let target = rec.p + rec.normal + random_in_unit_sphere().unit();
        return ray_colour(&Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
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
    // let viewport_width = ASPECT_RATIO * VIEWPORT_HEIGHT;
    // let horizontal = Vector::new(viewport_width, 0.0, 0.0);
    // let vertical = Vector::new(0.0, VIEWPORT_HEIGHT, 0.0);
    // let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vector::new(0.0, 0.0, FOCAL_LENGTH);
    let origin = Point::new(0.0, 0.0, 0.0);
    let cam = Camera::new(origin, ASPECT_RATIO, VIEWPORT_HEIGHT, FOCAL_LENGTH);

    // World
    let mut world = Environment{ hittables: Vec::new() };
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // File
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
