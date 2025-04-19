#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

use std::ops::Add;

pub use lalgebra_scalar::*;

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = Vec::new();
        for i in 0..self.0.len() {
            result.push(self.0[i] + other.0[i]);
        }
        Some(Vector(result))
    }
}

impl<T: Scalar> Vector<T> {
	pub fn new() -> Self {
        Vector(Vec::new())
	}

	pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut sum = <T as Scalar>::zero();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            sum = sum + *a * *b;
        }
        Some(sum)
	}
}