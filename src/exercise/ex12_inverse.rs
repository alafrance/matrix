use std::fmt::{Debug};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};
use crate::models::matrix::Matrix;

impl<T> Matrix<T> where
T: Clone + Debug + Default + AddAssign + PartialEq + MulAssign + SubAssign + Copy + From<i32>,
T: Mul<T, Output = T> + Sub<T, Output = T> + Add<T, Output = T> + Div<T, Output = T>
{
    pub fn inverse(&mut self) -> Result<Matrix<T>, &'static str> {
        if self.is_singular() {
            return Err("Matrix is singular");
        }
        else if !self.is_square() {
            return Err("Matrix must be square");
        }
        let mut adjoint = self.adjoint();
        let determinant_inverse = T::from(1) / self.determinant();
        adjoint.scl(determinant_inverse);
        Ok(adjoint)
    }

    pub fn adjoint(&mut self) -> Matrix<T> {
        let mut adjoint: Matrix<T> = Matrix::new(vec![T::default();self.rows*self.cols], self.rows, self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let mut sub_matrix = self.clone();
                sub_matrix.remove_row(row);
                sub_matrix.remove_col(col);
                let sign = if (row + col) % 2 == 0 { 1 } else { -1 };
                adjoint.set(row, col, sub_matrix.determinant() * T::from(sign));
            }
        }
        adjoint.transpose()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::matrix::Matrix;

    #[test]
    fn it_works() {
        let mut matrix = Matrix::from_arrays(vec![
            vec![1., 0., 0.],
            vec![0., 1., 0.],
            vec![0., 0., 1.],
        ]);
        let inverse = matrix.inverse().unwrap();
        inverse.print();
        println!();

        let mut matrix = Matrix::from_arrays(vec![
            vec![2., 0., 0.],
            vec![0., 2., 0.],
            vec![0., 0., 2.],
        ]);
        let inverse = matrix.inverse().unwrap();
        inverse.print();
        println!();

        let mut matrix = Matrix::from_arrays(vec![
            vec![8., 5., -2.],
            vec![4., 7., 20.],
            vec![7., 6., 1.],
        ]);
        let inverse = matrix.inverse().unwrap();
        inverse.print();
        println!();

    }

    #[test]
    #[should_panic]
    fn test_singular_matrix() {
        let mut matrix = Matrix::from_arrays(vec![
            vec![1., 2., 3.],
            vec![4., 5., 6.],
            vec![7., 8., 9.],
        ]);
        matrix.inverse().unwrap();
    }
}