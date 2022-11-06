use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Index, IndexMut, Sub};
use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Point {
    pub v: Vector,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        return Point { v: Vector{ xyz: [x, y, z]} };
    }

    pub fn x(&self) -> f64 {
        return self.v[0];
    }

    pub fn y(&self) -> f64 {
        return self.v[1];
    }

    pub fn z(&self) -> f64 {
        return self.v[2];
    }
}

impl Index<usize> for Point {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        return &self.v.xyz[i];
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        return &mut self.v.xyz[i];
    }
}

impl Sub for Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Self::Output {
        return self.v - rhs.v;
    }
}

impl Sub<Vector> for Point {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        return self.v - rhs;
    }
}

impl Sub<Point> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Self::Output {
        return self - rhs.v;
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        let v = self.v + rhs;
        return Point { v };
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.v);
    }
}

#[test]
fn test_points() {
    let p1 = Point::new(1.0, 2.0, 3.0);
    assert_eq!((p1[0], p1[1], p1[2]), (1.0, 2.0, 3.0));
    assert_eq!((p1.x(), p1.y(), p1.z()), (1.0, 2.0, 3.0));
    let p2 = Point::new(4.0, 8.0, 12.0);
    let v1 = p2 - p1;
    assert_eq!((v1[0], v1[1], v1[2]), (3.0, 6.0, 9.0));
}
