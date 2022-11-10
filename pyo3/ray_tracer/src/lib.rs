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

use render::render_scene;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn render_rays(
    image_width: u32,
    antialias_samples: u32,
    max_depth: i32,
) -> PyResult<String> {
    Ok(render_scene(
        image_width,
        antialias_samples,
        max_depth,
    ))
}

/// A Python module implemented in Rust.
#[pymodule]
fn ray_tracer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(render_rays, m)?)?;
    Ok(())
}