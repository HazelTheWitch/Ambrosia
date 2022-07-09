use crate::vectors::{Vector, ZERO_VECTOR};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transform {
    offset: Vector,
}

impl Transform {
    pub fn new(offset: Vector) -> Self {
        Transform { offset }
    }

    pub fn identity() -> Self {
        Transform::new(ZERO_VECTOR)
    }

    pub fn apply(&self, input: Vector) -> Vector {
        input + self.offset
    }

    pub fn inverse_apply(&self, input: Vector) -> Vector {
        input - self.offset
    }
}
