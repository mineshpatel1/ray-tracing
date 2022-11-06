use std::ops::{Index, IndexMut, Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub e: [f64; 3]
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        return Vector {e: [x, y, z]};
    }

    pub fn x(&self) -> f64 {
        return self.e[0];
    }

    pub fn y(&self) -> f64 {
        return self.e[1];
    }

    pub fn z(&self) -> f64 {
        return self.e[2];
    }
}

impl Index<usize> for Vector {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        return &self.e[i];
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        return &mut self.e[i];
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        return Vector {
            e: [
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
            e: [
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
            e: [
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
            e: [
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
            e: [
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

pub type Point = Vector;
pub type Colour = Vector;

#[test]
fn basic_vector() {
    let vec1 = Vector { e: [1.0, 2.0, 3.0] };
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
}

#[test]
fn vector_types() {
    let p1 = Point::new(1.0, 2.0, 3.0);
    let c1 = Colour::new(2.0, 3.0, 4.0);

    let vec2 = p1 * c1;
}