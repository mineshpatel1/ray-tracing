use std::fs::File;
use std::io::prelude::*;
use std::f64::consts::PI;

pub fn write_file(path: &String, content: &String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    return Ok(());
}

pub fn deg_to_rad(deg: i32) -> f64 {
    return (deg as f64 * PI) / 180.0;
}
