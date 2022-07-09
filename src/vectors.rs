use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use serde::Deserialize;

pub const ZERO_VECTOR: Vector = Vector { x: 0, y: 0 };
pub const ONE_VECTOR: Vector = Vector { x: 1, y: 1 };

pub const LEFT_VECTOR: Vector = Vector { x: -1, y: 0 };
pub const RIGHT_VECTOR: Vector = Vector { x: 1, y: 0 };
pub const UP_VECTOR: Vector = Vector { x: 0, y: -1 };
pub const DOWN_VECTOR: Vector = Vector { x: 0, y: 1 };

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Vector { x, y }
    }

    pub fn tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn distance(a: &Vector, b: &Vector) -> f32 {
        let (x0, y0) = a.tuple();
        let (x1, y1) = b.tuple();

        let (dx, dy) = (x0 - x1, y0 - y1);

        ((dx * dx + dy * dy) as f32).sqrt()
    }

    pub fn lerp(a: &Vector, b: &Vector, t: f32) -> Vector {
        let (a_x, a_y) = (a.x as f32, a.y as f32);
        let (b_x, b_y) = (b.x as f32, b.y as f32);

        Vector::new(
            (a_x + t * (b_x - a_x)).round() as i32,
            (a_y + t * (b_y - a_y)).round() as i32,
        )
    }

    pub fn line_lerp(a: &Vector, b: &Vector) -> Vec<Vector> {
        let mut points = Vec::new();

        let n = Vector::distance(a, b).ceil() as i32 * 10;

        for i in 0..n {
            let t = (i as f32) / ((n - 1) as f32);

            let next = Vector::lerp(a, b, t);

            if let Some(last) = points.last() {
                if &next != last {
                    points.push(next);
                }
            } else {
                points.push(next);
            }
        }

        points
    }

    pub fn line(a: &Vector, b: &Vector) -> Vec<Vector> {
        let dx = b.x - a.x;
        let dy = b.y - a.y;

        let nx = dx.abs();
        let ny = dy.abs();

        let sx = dx.signum();
        let sy = dy.signum();

        let mut points = Vec::new();

        points.push(*a);

        let (mut x, mut y) = a.tuple();

        let mut ix = 0;
        let mut iy = 0;

        while ix < nx || iy < ny {
            if (1 + 2 * ix) * ny < (1 + 2 * iy) * nx {
                x += sx;
                ix += 1;
            } else {
                y += sy;
                iy += 1;
            }

            points.push(Vector::new(x, y));
        }

        points
    }

    pub fn center(a: Vector, b: Vector) -> Vector {
        Vector { x: (a.x + b.x) / 2, y: (a.y + b.y) / 2 }
    }
}

impl From<(i32, i32)> for Vector {
    fn from(tuple: (i32, i32)) -> Self {
        Vector {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}
