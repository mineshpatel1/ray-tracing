use std::fmt;
use std::fmt::Display;
use std::ops::{Index, IndexMut, Add, Div, Mul, Neg, Sub, Range};
use rand::Rng;
use crate::colour::Colour;


#[derive(Clone, Copy)]
pub struct Vector {
    pub xyz: [f64; 3]
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        return Vector {xyz: [x, y, z]};
    }

    pub fn random(range: Range<f64>) -> Vector {
        let mut rng = rand::thread_rng();
        return Vector {
            xyz: [
                rng.gen_range(range.clone()),
                rng.gen_range(range.clone()),
                rng.gen_range(range.clone()),
            ],
        }
    }

    pub fn to_colour(self) -> Colour {
        return Colour {r: self.x(), g: self.y(), b: self.z()};
    }

    pub fn x(self) -> f64 {
        return self.xyz[0];
    }

    pub fn y(self) -> f64 {
        return self.xyz[1];
    }

    pub fn z(self) -> f64 {
        return self.xyz[2];
    }

    pub fn dot(self, u: Vector) -> f64 {
        return (self[0] * u[0]) + (self[1] * u[1]) + (self[2] * u[2]);
    }

    pub fn cross(self, u: Vector) -> Vector {
        return Vector::new(
            (self[1] * u[2]) - (self[2] * u[1]),
            (self[2] * u[0]) - (self[0] * u[2]),
            (self[0] * u[1]) - (self[1] * u[0]),
        );
    }

    pub fn length(self) -> f64 {
        return self.dot(self).powf(0.5);
    }

    pub fn unit(self) -> Vector {
        return self / self.length();
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        return self[0].abs() < s && self[1].abs() < s && self[2].abs() < s;
    }

    pub fn reflect(self, v: Vector) -> Vector {
        return self - (self.dot(v) * v * 2.0);
    }

    // (-23.490282122342506, -5.272570530585627, -0.47257053058562715)
    pub fn refract(self, v: Vector, eta_ratio: f64) -> Vector {
        let cos_theta = (-self).dot(v).min(1.0);
        let perp = (self + v * cos_theta) * eta_ratio;
        let parallel = -(1.0 - perp.dot(perp)).abs().powf(0.5) * v;
        return perp + parallel;
    }
}

impl Index<usize> for Vector {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        return &self.xyz[i];
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        return &mut self.xyz[i];
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        return Vector {
            xyz: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2],
            ]
        }
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        return Vector {
            xyz: [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2],
            ]
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;
    fn mul(self, rhs: Self) -> Self::Output {
        return Vector {
            xyz: [
                self[0] * rhs[0],
                self[1] * rhs[1],
                self[2] * rhs[2],
            ]
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        return Vector {
            xyz: [
                self[0] * rhs,
                self[1] * rhs,
                self[2] * rhs,
            ]
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        return rhs * self;
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        return Vector {
            xyz: [
                self[0] / rhs,
                self[1] / rhs,
                self[2] / rhs,
            ]
        }
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        return self * -1.0;
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({}, {}, {})", self[0], self[1], self[2]);
    }
}

pub fn random_in_unit_sphere() -> Vector {
    loop {
        let v = Vector::random(-1.0..1.0);
        if v.length() < 1.0 {
            return v;
        }
    }
}

pub fn random_in_unit_disk() -> Vector {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.dot(p) < 1.0 {
            return p;
        }
    }
}


#[test]
fn test_vectors() {
    let vec1 = Vector { xyz: [1.0, 2.0, 3.0] };
    let vec2 = Vector::new(4.0, 8.0, 12.0);
    assert_eq!((vec1[0], vec1[1], vec1[2]), (1.0, 2.0, 3.0));
    assert_eq!((vec1.x(), vec1.y(), vec1.z()), (1.0, 2.0, 3.0));

    let vec3 = vec1 + vec2;
    assert_eq!((vec3[0], vec3[1], vec3[2]), (5.0, 10.0, 15.0));
    let vec3 = vec1 - vec2;
    assert_eq!((vec3[0], vec3[1], vec3[2]), (-3.0, -6.0, -9.0));
    let vec3 = vec1 * vec2;
    assert_eq!((vec3[0], vec3[1], vec3[2]), (4.0, 16.0, 36.0));
    assert_eq!((-vec1[0], -vec1[1], -vec1[2]), (-1.0, -2.0, -3.0));
    let vec3 = vec1 / 2.0;
    assert_eq!((vec3[0], vec3[1], vec3[2]), (0.5, 1.0, 1.5));

    let x = Vector::new(2.0, 1.0, 5.0);
    let y = Vector::new(4.0, 1.0, 1.0);
    assert_eq!(x.dot(y), 14.0);

    let vec = x.cross(y);
    assert_eq!((vec[0], vec[1], vec[2]), (-4.0, 18.0, -2.0));

    let vec = x.reflect(y);
    assert_eq!((vec[0], vec[1], vec[2]), (-110.0, -27.0, -23.0));

    let vec = x.refract(y, 1.2);
    assert_eq!((vec[0], vec[1], vec[2]), (-334.8530318289354, -83.11325795723384, -78.31325795723384))
}
