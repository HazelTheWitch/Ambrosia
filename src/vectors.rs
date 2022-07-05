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

    pub fn distance(&self, other: &Vector) -> f32 {
        let (x0, y0) = self.tuple();
        let (x1, y1) = other.tuple();

        let (dx, dy) = (x0 - x1, y0 - y1);

        ((dx * dx + dy * dy) as f32).sqrt()
    }

    pub fn lerp(a: &Vector, b: &Vector, t: f32) -> Vector {
        let (a_x, a_y) = (a.x as f32, a.y as f32);
        let (b_x, b_y) = (b.x as f32, b.y as f32);
        
        Vector::new((a_x + t * (b_x - a_x)).round() as i32, (a_y + t * (b_y - a_y)).round() as i32)
    }

    pub fn line(a: &Vector, b: &Vector) -> Vec<Vector> {
        let mut points = Vec::new();

        let N = a.distance(b).ceil() as i32 * 10;

        for i in 0..N {
            let t = (i as f32) / ((N - 1) as f32);

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