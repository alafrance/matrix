use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use crate::utils::matrix::matrix::Matrix;

impl<T> Matrix<T> where
    T: Clone + Debug + Default + AddAssign + PartialEq + Neg<Output = T>,
    T: Mul<T, Output = T> + Sub<T, Output = T> + Add<T, Output = T> + Div<T, Output = T>
{
    pub fn row_echelon(&mut self) -> Matrix<T> {
        for row in 0..self.rows {
            if let Some(pivot_col) = self.get_pivot_col(row) {
                for row_below in row+1..self.rows {
                    if self.at(row_below, pivot_col) != T::default() {
                        let scalar = -(self.at(row_below, pivot_col) / self.at(row, pivot_col));
                        self.row_addition(row_below, row, scalar);
                    }
                }
            } else {
                break;
            }
        }
        self.clone()
    }

    fn get_first_nonzero_column(&self, row: usize) -> Option<usize> {
        for col in 0..self.cols {
            if self.at(row, col) != T::default() {
                return Some(col);
            }
        }
        None
    }

    fn get_pivot_col(&mut self, row: usize) -> Option<usize> {
        if let Some(col) = self.get_first_nonzero_column(row) {
            return Some(col);
        }
        for row_below in row..self.rows {
            if let Some(col) = self.get_first_nonzero_column(row_below) {
                self.row_swapping(row, row_below);
                return Some(col);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;
    use crate::utils::matrix::matrix::Matrix;

    #[test]
    fn it_works() {
        let mut matrix = Matrix::from_arrays(vec![
            vec![1., 2., 3.],
            vec![4., 5., 6.],
            vec![1., 1., 0.],
        ]);
        assert_eq!(matrix.row_echelon(), Matrix::from_arrays(vec![
            vec![1., 2., 3.],
            vec![0., -3., -6.],
            vec![0., 0., -1.],
        ]));

        let mut matrix = Matrix::from_arrays(vec![
            vec![1., 2.],
            vec![2., 4.],
        ]);
        assert_eq!(matrix.row_echelon(), Matrix::from_arrays(vec![
            vec![1., 2.],
            vec![0., 0.],
        ]));

        let mut matrix = Matrix::from_arrays(vec![
            vec![1., 2.],
            vec![3., 4.],
        ]);
        assert_eq!(matrix.row_echelon(), Matrix::from_arrays(vec![
            vec![1., 2.],
            vec![0., -2.],
        ]));

        let mut matrix = Matrix::from_arrays(vec![
            vec![8., 5., -2., 4., 28.],
            vec![4., 2.5, 20., 4., -4.],
            vec![8., 5., 1., 4., 17.],
        ]);
        assert_eq!(matrix.row_echelon(), Matrix::from_arrays(vec![
            vec![8., 5., -2., 4., 28.],
            vec![0., 0.0, 21., 2.0, -18.],
            vec![0., 0., 0., -0.2857142857142857, -8.428571428571429],
        ]));
    }

    #[test]
    fn test_with_complex_numbers() {
        let mut m1 = Matrix::<Complex<f64>>::new(vec![Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(3., 0.), Complex::new(4., 0.)], 2, 2);
        assert_eq!(m1.row_echelon(), Matrix::new(vec![Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(0., 0.), Complex::new(-2., 0.)], 2, 2));
    }
}