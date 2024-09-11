use std::{io::Error, ops::{Add, Neg}, str::FromStr};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Point3 { pub x: i32, pub y: i32, pub z: i32 }

impl Point3 {
    pub fn project(&self, axis: Axis) -> i32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }
}

impl Add for Point3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Neg for Point3 {
    type Output = Point3;

    fn neg(self) -> Self::Output {
        Point3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl FromStr for Point3 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let axes: Vec<i32> = s.split(',').map(|s| s.trim().parse().unwrap()).collect();
        assert!(axes.len() == 3);
        Ok(Point3 { x: axes[0], y: axes[1], z: axes[2] })
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Axis {
    X, Y, Z
}

impl Axis {
    pub fn all() -> [Axis; 3] {
        [Axis::X, Axis::Y, Axis::Z]
    }

    pub fn as_vec(&self) -> Point3 {
        match self {
            Axis::X => Point3 { x: 1, y: 0, z: 0 },
            Axis::Y => Point3 { x: 0, y: 1, z: 0 },
            Axis::Z => Point3 { x: 0, y: 0, z: 1 },
        }
    }
}
