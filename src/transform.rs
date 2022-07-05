use crate::vectors::Vector;

pub struct Transform {
    offset: Vector,
}

impl Transform {
    pub fn new(offset: Vector) -> Self {
        Transform { offset }
    }

    pub fn apply(&self, input: Vector) -> Vector {
        input + self.offset
    }

    pub fn inverse_apply(&self, input: Vector) -> Vector {
        input - self.offset
    }
}
