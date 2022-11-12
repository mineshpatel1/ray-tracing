use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

use crate::camera::Camera;
use crate::colour::Colour;
use crate::hittable::{Environment, Hit};
use crate::material::{Diffuse};
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::Vector;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
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

pub fn render_spheres(
    spheres: Vec<Sphere>,
    image_width: u32,
    antialias_samples: u32,
    max_depth: i32,
    v_fov: i32,
    look_from: Point,
) -> String {
    // Image
    let image_height = (image_width as f64 / ASPECT_RATIO) as u32;

    // Camera
    let cam = Camera::new(look_from, LOOK_AT, V_UP, ASPECT_RATIO, v_fov, APERTURE);

    // World
    let mut world = Environment{ hittables: Vec::new() };

    // Ground
    world.add(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Diffuse::new(Colour::new(0.5, 0.5, 0.5)),
    ));

    for sphere in spheres.into_iter() {
        world.add(sphere);
    }

    // File
    println!("\n⏳ Rendering...\n");
    let mut out = format!("P3\n{} {}\n255\n", image_width, image_height);
    let bar = ProgressBar::new(image_height as u64);
    bar.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:100.cyan/blue} {percent}/100%")
            .unwrap()
            .progress_chars("█░"),
    );

    for j in (0..image_height).rev() {
        let pixels: Vec<Colour> = (0..image_width)
            .into_par_iter()
            .map(|i| {
                let mut rng = rand::thread_rng();
                let mut pixel = Colour::new(0.0, 0.0, 0.0);
                for _ in 0..antialias_samples {
                    let u_r: f64 = rng.gen();
                    let v_r: f64 = rng.gen();
                    let u = ((i as f64) + u_r) / ((image_width - 1) as f64);
                    let v = ((j as f64) + v_r) / ((image_height - 1) as f64);
                    let ray = cam.get_ray(u, v);
                    pixel += ray_colour(&ray, &world, max_depth);
                }
                return pixel;
            })
            .collect();

        for pixel in pixels.into_iter() {
            out.push_str(&format!("{}\n", pixel.render(antialias_samples))[..]);
        }
        bar.inc(1);
    }
    bar.finish();
    println!("\n\n✅ Rendering complete.\n");
    return out;
}
