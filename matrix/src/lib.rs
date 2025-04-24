use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(Vec::new())
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![<T as Scalar>::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut m = Matrix::zero(n, n);
        for i in 0..n {
            m.0[i][i] = <T as Scalar>::one();
        }
        m
    }
}