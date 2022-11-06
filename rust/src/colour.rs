use std::fmt;
use std::fmt::Display;

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

    pub fn to_str(self) -> String {
        let ir = (255.999 * self.r) as u32;
        let ig = (255.999 * self.g) as u32;
        let ib = (255.999 * self.b) as u32;
        return format!("{} {} {}", ir, ig, ib);
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
    assert_eq!(c1.to_str(), String::from("204 51 76"));
}