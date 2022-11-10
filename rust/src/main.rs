mod camera;
mod colour;
mod hittable;
mod material;
mod point;
mod ray;
mod sphere;
mod utils;
mod vector;

use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;

use camera::Camera;
use colour::Colour;
use hittable::{Environment, Hit};
use material::{Diffuse, Glass, Material, Metal};
use point::Point;
use ray::Ray;
use sphere::Sphere;
use utils::write_file;

use crate::vector::Vector;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;

const IMAGES_DIR: &str = "images";
const OUTPUT_IMAGE: &str = "final";
const ANTIALIAS_SAMPLES: i64 = 100;
const MAX_DEPTH: i32 = 50;
const V_FOV: i32 = 20;
const LOOK_FROM: Point = Point {
    v: Vector {
        xyz: [13.0, 2.0, 3.0],
    },
};
const LOOK_AT: Point = Point {
    v: Vector {
        xyz: [0.0, 0.0, 0.0],
    },
};
const V_UP: Vector = Vector {
    xyz: [0.0, 1.0, 0.0],
};
const APERTURE: f64 = 0.01;

fn ray_colour(ray: &Ray, world: &Environment, depth: i32) -> Colour {
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

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

fn create_scene(n: i32) -> Environment {
    let mut world = Environment {
        hittables: Vec::new(),
    };

    // Ground
    let ground_mat = Diffuse::new(Colour::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    ));

    let mut rng = rand::thread_rng();
    for a in -n..n {
        for b in -n..n {
            let choose_mat: f64 = rng.gen();
            let centre = Point::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9),
            );

            let material: Arc<dyn Material>;
            if choose_mat < 0.8 {
                material = Diffuse::new(Colour::random());
            } else if choose_mat < 0.95 {
                material = Metal::new(Colour::random_range(0.0..0.5), rng.gen_range(0.0..0.5));
            } else {
                material = Glass::new(1.5);
            }
            world.add(Sphere::new(centre, 0.2, material));
        }
    }

    let left_mat = Diffuse::new(Colour::new(0.1, 0.2, 0.5));
    let centre_mat = Glass::new(1.5);
    let right_mat = Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0);

    world.add(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, left_mat));
    world.add(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, centre_mat));
    world.add(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, right_mat));
    return world;
}

fn main() {
    // Image
    let image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Camera
    let cam = Camera::new(LOOK_FROM, LOOK_AT, V_UP, ASPECT_RATIO, V_FOV, APERTURE);

    // World
    let world = create_scene(8);

    // File
    println!("\n⏳ Rendering...\n");
    let fpath = format!("{}/{}.ppm", IMAGES_DIR, OUTPUT_IMAGE);
    let mut out = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, image_height);
    let bar = ProgressBar::new(image_height as u64);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:100.cyan/blue} {percent}/100%")
        .unwrap()
        .progress_chars("█░"));

    for j in (0..image_height).rev() {
        let pixels: Vec<Colour> = (0..IMAGE_WIDTH)
            .into_par_iter()
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
    println!("\n\n✅ Rendering complete.\n");
}
