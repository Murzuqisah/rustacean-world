use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Add for Matrix<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len()
            || self.0.get(0).map_or(0, |r| r.len()) != other.0.get(0).map_or(0, |r| r.len())
        {
            return None;
        }

        let mut result = self.0.clone();
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                result[i][j] = result[i][j] + other.0[i][j];
            }
        }
        Some(Matrix(result))
    }
}

impl<T> Sub for Matrix<T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    type Output = Option<Self>;

    fn sub(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len()
            || self.0.get(0).map_or(0, |r| r.len()) != other.0.get(0).map_or(0, |r| r.len())
        {
            return None;
        }

        let mut result = self.0.clone();
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                result[i][j] = result[i][j] - other.0[i][j];
            }
        }
        Some(Matrix(result))
    }
}
