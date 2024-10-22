use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use crate::utils::matrix::matrix::Matrix;

impl<T> Matrix<T> where
    T: Clone + Debug + Default + AddAssign + PartialEq + Neg<Output = T>,
    T: Mul<T, Output = T> + Sub<T, Output = T> + Add<T, Output = T> + From<f64> + Div<T, Output = T>
{
    fn rank(&mut self) -> usize {
        let echelon = self.row_echelon();
        let mut rank = 0;
        for row in 0..echelon.rows {
            if !echelon.is_null_row(row) {
                rank += 1;
            }
        }
        rank
    }

    fn is_null_row(&self, row: usize) -> bool {
        for col in 0..self.cols {
            if self.at(row, col) != T::default() {
                return false;
            }
        }
        true
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
        assert_eq!(matrix.rank(), 3);

        let mut matrix = Matrix::from_arrays(vec![
            vec![1., 2.],
            vec![2., 4.],
        ]);
        assert_eq!(matrix.rank(), 1);

        let mut matrix = Matrix::from_arrays(vec![
            vec![1., 2.],
            vec![3., 4.],
        ]);
        assert_eq!(matrix.rank(), 2);

        let mut matrix = Matrix::from_arrays(vec![
            vec![1., 0., 0.],
            vec![0., 1., 0.],
            vec![0., 0., 1.],
        ]);
        assert_eq!(matrix.rank(), 3);

        let mut matrix = Matrix::from_arrays(vec![
            vec![ 1., 2., 0., 0.],
            vec![ 2., 4., 0., 0.],
            vec![-1., 2., 1., 1.],
        ]);
        assert_eq!(matrix.rank(), 2);

        let mut matrix = Matrix::from_arrays(vec![
            vec![ 8., 5., -2.],
            vec![ 4., 7., 20.],
            vec![ 7., 6., 1.],
            vec![21., 18., 7.],
        ]);
        assert_eq!(matrix.rank(), 3);
    }

    #[test]
    fn test_complex_numbers() {
        let mut matrix = Matrix::from_arrays(vec![
            vec![Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(3., 0.)],
            vec![Complex::new(0., 1.), Complex::new(-1., 1.), Complex::new(1., 1.)],
            vec![Complex::new(1., 0.), Complex::new(2., 2.), Complex::new(3., 3.)]
        ]);
        assert_eq!(matrix.rank(), 3);
    }
}