use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, Sub};
use crate::utils::matrix::matrix::Matrix;

impl<T: Clone + Debug> Matrix<T> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn size(&self) -> usize {
        self.rows * self.cols
    }

    pub fn is_null_matrix(&self) -> bool where T: Default + PartialEq {
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.at(i, j) != T::default() {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_singular(&self) -> bool where
        T: Clone + Debug + Default + AddAssign + PartialEq,
        T: Mul<T, Output = T> + Sub<T, Output = T> + Add<T, Output = T> + From<i32>
    {
        if self.is_square() {
            let mut matrix = self.clone();
            matrix.determinant() == T::default()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.cols(), 3);
        assert_eq!(matrix.size(), 6);
        assert_eq!(matrix.shape(), (2, 3));
        assert_eq!(matrix.is_square(), false);
    }

    #[test]
    fn test_is_null_matrix() {
        let matrix = Matrix::from_arrays(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);
        assert_eq!(matrix.is_null_matrix(), true);

        let matrix = Matrix::from_arrays(vec![
            vec![1, 0, 0],
            vec![0, 0, 0],
        ]);
        assert_eq!(matrix.is_null_matrix(), false);
    }

    #[test]
    fn test_is_singular() {
        let matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);
        assert_eq!(matrix.is_singular(), true);

        let matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 10],
        ]);
        assert_eq!(matrix.is_singular(), false);
    }
}