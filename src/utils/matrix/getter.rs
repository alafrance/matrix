use std::fmt::Debug;
use crate::utils::matrix::matrix::Matrix;
use crate::utils::vector::vector::Vector;

pub trait Getter: Clone + Debug {}

impl<T: Clone + Debug> Getter for T {}

impl<T: Getter > Matrix<T> {

    pub fn at(&self, row: usize, col: usize) -> T {
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds");
        }
        let index = row * self.cols + col;
        self.data[index].clone()
    }

    pub fn get_row_vector(&self, row: usize) -> Vector<T> {
        if row >= self.rows {
            panic!("Row index out of bounds");
        }
        let start = row * self.cols;
        let end = start + self.cols;
        Vector::new(self.data[start..end].to_vec())
    }

    pub fn get_col_vector(&self, col: usize) -> Vector<T> {
        if col >= self.cols {
            panic!("Column index out of bounds");
        }
        let start = col;
        let mut data = Vec::new();
        for i in 0..self.rows {
            let index = i * self.cols + start;
            data.push(self.data[index].clone());
        }
        Vector::new(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::matrix::matrix::Matrix;


    #[test]
    fn transform_row_and_vector() {
        let matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ]);
        let vector = matrix.get_row_vector(1);
        assert_eq!(vector.size(), 3);
        assert_eq!(vector.x(), 4);
        assert_eq!(vector.y(), 5);
        assert_eq!(vector.z(), 6);

        let vector = matrix.get_col_vector(1);
        assert_eq!(vector.size(), 2);
        assert_eq!(vector.x(), 2);
        assert_eq!(vector.y(), 5);
    }

    #[test]
    fn test_at_position() {
        let matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ]);
        assert_eq!(matrix.at(0, 0), 1);
        assert_eq!(matrix.at(0, 1), 2);
        assert_eq!(matrix.at(0, 2), 3);
        assert_eq!(matrix.at(1, 0), 4);
        assert_eq!(matrix.at(1, 1), 5);
        assert_eq!(matrix.at(1, 2), 6);
    }

    #[test]
    #[should_panic]
    fn test_at_position_out_of_bounds() {
        let matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ]);
        matrix.at(2, 0);
    }
}