use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;

use crate::camera::Camera;
use crate::colour::Colour;
use crate::hittable::{Environment, Hit};
use crate::material::{Diffuse, Glass, Material, Metal};
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::Vector;


const ASPECT_RATIO: f64 = 16.0 / 9.0;
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

pub fn render_scene(
    image_width: u32,
    antialias_samples: u32,
    max_depth: i32,
) -> String {
    // Image
    let image_height = (image_width as f64 / ASPECT_RATIO) as u32;

    // Camera
    let cam = Camera::new(LOOK_FROM, LOOK_AT, V_UP, ASPECT_RATIO, V_FOV, APERTURE);

    // World
    let world = create_scene(8);

    // File
    println!("\n⏳ Rendering...\n");
    let mut out = format!("P3\n{} {}\n255\n", image_width, image_height);
    let bar = ProgressBar::new(image_height as u64);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:100.cyan/blue} {percent}/100%")
        .unwrap()
        .progress_chars("█░"));

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
