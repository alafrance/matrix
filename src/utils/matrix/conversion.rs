use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use crate::utils::matrix::matrix::Matrix;
use crate::utils::vector::vector::Vector;

impl<T: Clone + Debug> Matrix<T> {
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
    pub fn minor(&self, row: usize, col: usize) -> Matrix<T> where
        T: Clone + Debug + Default + AddAssign + PartialEq + MulAssign + SubAssign + Copy + From<i32>,
        T: Mul<T, Output=T> + Sub<T, Output=T> + Add<T, Output=T>
    {
        let mut sub_matrix = self.clone();
        sub_matrix.remove_row(row);
        sub_matrix.remove_col(col);
        sub_matrix
    }

    pub fn identity(size: usize) -> Matrix<T> where
        T: Clone + Debug + Default + AddAssign + PartialEq + MulAssign + SubAssign + Copy + From<f64>,
        T: Mul<T, Output=T> + Sub<T, Output=T> + Add<T, Output=T>
    {
        let mut data = Vec::new();
        for i in 0..size {
            for j in 0..size {
                if i == j {
                    data.push(T::from(1.));
                } else {
                    data.push(T::default());
                }
            }
        }
        Matrix::new(data, size, size)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::matrix::matrix::Matrix;
    use crate::utils::vector::vector::Vector;

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

    #[test]
    fn test_minor() {
        let matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);
        let minor = matrix.minor(1, 1);
        assert_eq!(minor.rows(), 2);
        assert_eq!(minor.cols(), 2);
        minor.print();
        assert_eq!(minor.at(0, 0), 1);
        assert_eq!(minor.at(0, 1), 3);
        assert_eq!(minor.at(1, 0), 7);
        assert_eq!(minor.at(1, 1), 9);
    }

    #[test]
    fn test_identity() {
        let matrix = Matrix::<f64>::identity(3);
        assert_eq!(matrix.rows(), 3);
        assert_eq!(matrix.cols(), 3);
        assert_eq!(matrix.size(), 9);
        assert_eq!(matrix.at(0, 0), 1.);
        assert_eq!(matrix.at(0, 1), 0.);
        assert_eq!(matrix.at(0, 2), 0.);
        assert_eq!(matrix.at(1, 0), 0.);
        assert_eq!(matrix.at(1, 1), 1.);
        assert_eq!(matrix.at(1, 2), 0.);
        assert_eq!(matrix.at(2, 0), 0.);
        assert_eq!(matrix.at(2, 1), 0.);
        assert_eq!(matrix.at(2, 2), 1.);
    }
}