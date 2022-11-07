use std::fmt;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        return Colour{r, g, b};
    }

    pub fn render(self, samples: i64) -> String {
        // Formats the colour and adds a correction of Gamma = 2 
        let ir = (256.0 * (self.r / (samples as f64)).powf(0.5).clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self.g / (samples as f64)).powf(0.5).clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self.b / (samples as f64)).powf(0.5).clamp(0.0, 0.999)) as u64;
        return format!("{} {} {}", ir, ig, ib);
    }
}

impl Add for Colour {
    type Output = Colour;
    fn add(self, other: Self) -> Self::Output {
        return Colour::new(
            self.r + other.r,
            self.g + other.g,
            self.b + other.b,
        );
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;
    fn mul(self, val: f64) -> Self::Output {
        return Colour::new(
            self.r * val,
            self.g * val,
            self.b * val,
        );
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "(r: {}, g: {}, b: {})", self.r, self.g, self.b);
    }
}

#[test]
fn test_colours() {
    let c1 = Colour{r: 0.8, g: 0.2, b: 0.3};
    assert_eq!(c1.render(1), String::from("228 114 140"));
}