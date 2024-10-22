use std::fmt::Debug;
use crate::utils::matrix::matrix::Matrix;
use crate::utils::vector::vector::Vector;

impl<T: Clone + Debug> Vector<T> {
    pub fn from_array(array: Vec<T>) -> Vector<T> {
        Vector { data: array }
    }

    pub fn as_matrix(&self) -> Matrix<T> {
        Matrix::from_vector(self)
    }

    pub fn as_array(&self) -> Vec<T> {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::vector::vector::Vector;

    #[test]
    fn test_as_matrix() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        let matrix = vector.as_matrix();
        assert_eq!(matrix.rows(), 3);
        assert_eq!(matrix.cols(), 1);
    }

    #[test]
    fn test_as_array() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        let array = vector.as_array();
        assert_eq!(array, vec![1, 2, 3]);
    }


}