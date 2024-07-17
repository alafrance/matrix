use std::fmt::{Debug};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};
use crate::models::matrix::Matrix;

impl<T> Matrix<T> where
T: Clone + Debug + Default + AddAssign + PartialEq + MulAssign + SubAssign + Copy + From<i32>,
T: Mul<T, Output = T> + Sub<T, Output = T> + Add<T, Output = T> + Div<T, Output = T>
{
    pub fn inverse(&mut self) -> Matrix<T> {
        let (l, u) = self.lu_decompose();
        let identity = Matrix::<T>::identity(self.rows);
        let y = Self::forward_decomposition(l, identity);
        Self::backward_substitution(&u, &y)

    }

    fn forward_decomposition(l: Matrix<T>, b: Matrix<T>) -> Matrix<T> {
        let mut y = Matrix::new(vec![T::default(); b.rows * b.cols], b.rows, b.cols);
        for col in 0..b.cols {
            for row in 0..l.rows {
                let mut sum = T::default();
                for k in 0..row {
                    sum = sum + l.at(row, k) * y.at(k, col);
                }
                y.set(row, col, b.at(row, col) - sum);
            }
        }
        y
    }

    fn backward_substitution(u: &Matrix<T>, y: &Matrix<T>) -> Matrix<T> {
        let mut x = Matrix::new(vec![T::default(); y.rows * y.cols], y.rows, y.cols);
        for col in 0..y.cols {
            for row in (0..u.rows).rev() {
                let mut sum = T::default();
                for k in row + 1..u.cols {
                    sum = sum + u.at(row, k) * x.at(k, col);
                }
                x.set(row, col, (y.at(row, col) - sum) / u.at(row, row));
            }
        }
        x
    }


    fn lu_decompose(&mut self) -> (Matrix<T>, Matrix<T>) {
        let mut l = Matrix::new(vec![T::default();self.rows*self.cols], self.rows, self.cols);
        let mut u = self.clone();
        for row in 0..self.rows {
            l.set(row, row, T::from(1));
            for row_below in row+1..self.rows {
                let scalar = u.at(row_below, row) / u.at(row, row);
                l.set(row_below, row, scalar);
                for col in 0..self.cols {
                    u.set(row_below, col, u.at(row_below, col) - u.at(row, col) * scalar);
                }
            }
        }
        (l, u)
    }

    // pub fn inverse(&mut self) -> Matrix<T> {
//          if !self.is_singular() {
//              panic!("Matrix must be singular");
//          }
    //     if !self.is_square() {
    //         return panic!("Matrix must be square");
    //     }
    //     let mut adjoint = self.adjoint();
    //     let determinant_inverse = T::from(1) / self.determinant();
    //     adjoint.scl(determinant_inverse);
    //     adjoint
    // }
    //
    // pub fn adjoint(&mut self) -> Matrix<T> {
    //     let mut adjoint: Matrix<T> = Matrix::new(vec![T::default();self.rows*self.cols], self.rows, self.cols);
    //     for row in 0..self.rows {
    //         for col in 0..self.cols {
    //             let mut sub_matrix = self.clone();
    //             sub_matrix.remove_row(row);
    //             sub_matrix.remove_col(col);
    //             let sign = if (row + col) % 2 == 0 { 1 } else { -1 };
    //             adjoint.set(row, col, sub_matrix.determinant() * T::from(sign));
    //         }
    //     }
    //     adjoint.transpose()
    // }
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
        let inverse = matrix.inverse();
        inverse.print();
        println!();

        let mut matrix = Matrix::from_arrays(vec![
            vec![2., 0., 0.],
            vec![0., 2., 0.],
            vec![0., 0., 2.],
        ]);
        let inverse = matrix.inverse();
        inverse.print();
        println!();

        let mut matrix = Matrix::from_arrays(vec![
            vec![8., 5., -2.],
            vec![4., 7., 20.],
            vec![7., 6., 1.],
        ]);
        let inverse = matrix.inverse();
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
        matrix.inverse();
    }
}