use std::{ops::{Add, Sub}, fmt::Display};

pub const ZERO_VECTOR: Vector = Vector { x: 0, y: 0 };
pub const ONE_VECTOR: Vector = Vector { x: 1, y: 1 };

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    pub x: i32,
    pub y: i32
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Vector { x, y }
    }

    pub fn tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl From<(i32, i32)> for Vector {
    fn from(tuple: (i32, i32)) -> Self {
        Vector { x: tuple.0, y: tuple.1 }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}