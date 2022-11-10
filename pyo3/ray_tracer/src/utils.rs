use std::f64::consts::PI;

pub fn deg_to_rad(deg: i32) -> f64 {
    return (deg as f64 * PI) / 180.0;
}
