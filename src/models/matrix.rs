use std::fmt::Debug;
use std::ops::{Mul};
use crate::models::vector::Vector;


pub struct Matrix<T: Clone + Debug> {
    pub(crate) data: Vec<T>,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl<T: Clone + Debug> Matrix<T> {
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> Matrix<T> {
        if data.len() % rows != 0 || data.len() % cols != 0 {
            panic!("Data length must be a multiple of rows");
        }
        Matrix { data, rows, cols }
    }

    pub fn from_vector(vector: &Vector<T>) -> Matrix<T> {
        Matrix::new(vector.as_array(), vector.size(), 1)
    }

    pub fn from_vectors(vectors: Vec<Vector<T>>) -> Matrix<T> {
        let mut data = Vec::new();
        let rows = vectors.len();
        let cols = vectors[0].size();
        for vector in vectors.iter() {
            if vector.size() != cols {
                panic!("All vectors must have the same size");
            }
            data.extend(vector.as_array());
        }
        Matrix::new(data, rows, cols)
    }

    pub fn from_arrays(arrays: Vec<Vec<T>>) -> Matrix<T> {
        if arrays.is_empty() {
            panic!("Arrays must not be empty");
        }
        let rows = arrays.len();

        let len = arrays[0].len();
        for array in arrays.iter() {
            if array.len() != len {
                panic!("All arrays must have the same length");
            }
            if array.is_empty() {
                panic!("Arrays must not be empty");
            }
        }
        Matrix::new(arrays.into_iter().flatten().collect(), rows, len)
    }

    pub fn as_vectors(&self) -> Vec<Vector<T>> {
        let mut vectors = Vec::new();
        for i in 0..self.rows {
            let start = i * self.cols;
            let end = start + self.cols;
            vectors.push(Vector::from_array(self.data[start..end].to_vec().clone()));
        }
        vectors
    }

    pub fn print(&self) {
        for i in 0..self.rows {
            let start = i * self.cols;
            let end = start + self.cols;
            println!("[{:?}]", &self.data[start..end]);
        }
    }

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
}

impl Mul<f32> for Matrix<f32> {
    type Output = Matrix<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut v = self.clone();
        v.scl(rhs);
        v
    }
}

impl<T: Clone + Debug> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Matrix {
            data: self.data.clone(),
            rows: self.rows,
            cols: self.cols,
        }
    }

}

// Utils
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
    fn test_from_vector_and_vectors() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        let matrix = Matrix::from_vector(&vector);
        assert_eq!(matrix.rows(), 3);
        assert_eq!(matrix.cols(), 1);
        assert_eq!(matrix.shape(), (3, 1));
        assert_eq!(matrix.size(), 3);
        assert_eq!(matrix.is_square(), false);

        let matrix = Matrix::from_vectors(vec![
            Vector::from_array(vec![1, 2, 3]),
            Vector::from_array(vec![4, 5, 6]),
            Vector::from_array(vec![7, 8, 9]),
        ]);
        assert_eq!(matrix.rows(), 3);
        assert_eq!(matrix.cols(), 3);
        assert_eq!(matrix.size(), 9);
        assert_eq!(matrix.is_square(), true);
    }

    #[test]
    fn test_from_arrays() {
        let matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);
        assert_eq!(matrix.rows(), 3);
        assert_eq!(matrix.cols(), 3);
        assert_eq!(matrix.size(), 9);
        assert_eq!(matrix.is_square(), true);
    }

    #[test]
    fn test_print() {
        let matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);
        matrix.print();
    }

    #[test]
    fn test_as_vectors() {
        let matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);
        let vectors = matrix.as_vectors();
        for vector in vectors.iter() {
            assert_eq!(vector.size(), 3);
        }
    }
}