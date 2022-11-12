use std::sync::Arc;
use pyo3::prelude::*;

mod camera;
mod colour;
mod hittable;
mod material;
mod point;
mod ray;
mod render;
mod sphere;
mod utils;
mod vector;

use render::render_spheres;
use colour::Colour;
use material::{Diffuse, Metal, Glass, Material};
use sphere::{Sphere as RustSphere};
use point::Point;


#[pyclass]
#[derive(Clone)]
struct MaterialProps {
    material_type: String,
    colour: Option<(f64, f64, f64)>,
    fuzz: Option<f64>,
    refractive_idx: Option<f64>,
}

#[pymethods]
impl MaterialProps {
    #[new]
    fn new(
        material_type: String,
        colour: Option<(f64, f64, f64)>,
        fuzz: Option<f64>,
        refractive_idx: Option<f64>,
    ) -> Self {
        let allowed = vec!["diffuse", "metal", "glass"];
        if !allowed.contains(&material_type.to_lowercase().as_str()) {
            panic!("Invalid material type.");
        }
        return MaterialProps { material_type, colour, fuzz, refractive_idx };
    }
}

impl MaterialProps {
    fn gen(&self) -> Arc<dyn Material> {
        return match self.material_type.to_lowercase().as_str() {
            "diffuse" => {
                let colour = self.colour.unwrap();
                Diffuse::new(Colour::new(colour.0, colour.1, colour.2))
            },
            "metal" => {
                let colour = self.colour.unwrap();
                Metal::new(Colour::new(colour.0, colour.1, colour.2), self.fuzz.unwrap())
            },
            "glass" => {
                Glass::new(self.refractive_idx.unwrap())
            },
            _ => panic!("Invalid material type."),
        };
    }
}

#[pyclass]
struct Sphere {
    #[pyo3(get, set)]
    centre: (f64, f64, f64),
    #[pyo3(get, set)]
    radius: f64,
    #[pyo3(get, set)]
    material: MaterialProps,
}

#[pymethods]
impl Sphere {
    #[new]
    fn new(centre: (f64, f64, f64), radius: f64, material: MaterialProps) -> Self {
        return Sphere { centre, radius, material};
    }
}

#[pyfunction]
fn sum_as_string(a: i32, b: i32) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn render_scene(
    spheres: Vec<PyRef<Sphere>>,
    image_width: u32,
    antialias_samples: u32,
    max_depth: i32,
    v_fov: Option<i32>,
    look_from: Option<(f64, f64, f64)>,
) -> PyResult<String> {
    let mut rust_spheres: Vec<RustSphere> = Vec::new();

    for sphere in spheres.into_iter() {
        let rust_sphere = RustSphere::new(
            Point::new(sphere.centre.0, sphere.centre.1, sphere.centre.2),
            sphere.radius,
            sphere.material.gen(),
        );
        rust_spheres.push(rust_sphere);
    }

    let rust_look_from = look_from.unwrap_or((0.0, 2.0, 3.0));
    let rust_look_from = Point::new(rust_look_from.0, rust_look_from.1, rust_look_from.2);
    Ok(render_spheres(
        rust_spheres,
        image_width,
        antialias_samples,
        max_depth,
        v_fov.unwrap_or(90),
        rust_look_from,
    ))
}

#[pymodule]
fn ray_tracer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Sphere>()?;
    m.add_class::<MaterialProps>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(render_scene, m)?)?;
    Ok(())
}
