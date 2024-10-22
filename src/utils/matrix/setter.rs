use std::ops::{Add, Mul};
use num_traits::Float;
use crate::utils::matrix::matrix::Matrix;
pub trait Setter<T>: Mul<T, Output=T> + Clone {}
impl<T: Mul<T, Output=T> + Clone> Setter<T> for T {}


impl<T: Setter<T>> Matrix<T> {
    pub fn row_swapping(&mut self, row1: usize, row2: usize) {
        if row1 >= self.rows || row2 >= self.rows {
            panic!("Row index out of bounds");
        }
        let start1 = row1 * self.cols;
        let start2 = row2 * self.cols;
        for i in 0..self.cols {
            self.data.swap(start1 + i, start2 + i);
        }
    }

    pub fn row_scaling(&mut self, row: usize, scalar: T) {
        if row >= self.rows {
            panic!("Row index out of bounds");
        }
        let start = row * self.cols;
        for i in 0..self.cols {
            self.data[start + i] = self.data[start + i].clone() * scalar.clone();
        }
    }

    pub fn row_addition(&mut self, row1: usize, row2: usize, scalar: T) where
        T: Mul<T, Output = T> + Add<T, Output = T> + Float
    {
        if row1 >= self.rows || row2 >= self.rows {
            panic!("Row index out of bounds");
        }
        let start1 = row1 * self.cols;
        let start2 = row2 * self.cols;
        for i in 0..self.cols {
            self.data[start1 + i] = (self.data[start1 + i].clone() + self.data[start2 + i].clone() * scalar.clone()).round()
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds");
        }
        let index = row * self.cols + col;
        self.data[index] = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::matrix::matrix::Matrix;

    #[test]
    fn row_addition() {
        let mut matrix = Matrix::from_arrays(vec![
            vec![1., 2., 3.],
            vec![4., 5., 6.],
        ]);
        matrix.row_addition(0, 1, 1.);
        assert_eq!(matrix.get_row_vector(0).as_array(), vec![5., 7., 9.]);
    }



    #[test]
    fn row_scaling() {
        let mut matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ]);
        matrix.row_scaling(0, 2);
        assert_eq!(matrix.get_row_vector(0).as_array(), vec![2, 4, 6]);
        assert_eq!(matrix.get_row_vector(1).as_array(), vec![4, 5, 6]);
    }

    #[test]
    fn row_swapping() {
        let mut matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ]);
        matrix.row_swapping(0, 1);
        assert_eq!(matrix.get_row_vector(0).as_array(), vec![4, 5, 6]);
        assert_eq!(matrix.get_row_vector(1).as_array(), vec![1, 2, 3]);
    }


    #[test]
    fn test_set() {
        let mut matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ]);
        matrix.set(0, 0, 10);
        assert_eq!(matrix.at(0, 0), 10);
    }
}