use std::fmt::Debug;
use crate::utils::matrix::matrix::Matrix;

impl<T: Clone + Debug> Matrix<T> {
    pub fn remove_row(&mut self, row: usize) {
        if row >= self.rows {
            panic!("Row index out of bounds");
        }
        let start = row * self.cols;
        self.data.drain(start..start + self.cols);
        self.rows -= 1;
    }

    pub fn remove_col(&mut self, col: usize) {
        if col >= self.cols {
            panic!("Column index out of bounds");
        }
        let mut new_data = Vec::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                if j != col {
                    new_data.push(self.at(i, j));
                }
            }
        }
        self.data = new_data;
        self.cols -= 1;
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::matrix::matrix::Matrix;

    #[test]
    fn test_remove_row() {
        let mut matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);
        matrix.remove_row(1);
        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.size(), 6);
        assert_eq!(matrix.at(0, 0), 1);
        assert_eq!(matrix.at(0, 1), 2);
        assert_eq!(matrix.at(0, 2), 3);
        assert_eq!(matrix.at(1, 0), 7);
        assert_eq!(matrix.at(1, 1), 8);
        assert_eq!(matrix.at(1, 2), 9);
    }

    #[test]
    fn test_remove_col() {
        let mut matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);
        matrix.remove_col(1);
        assert_eq!(matrix.cols(), 2);
        assert_eq!(matrix.size(), 6);
        assert_eq!(matrix.at(0, 0), 1);
        assert_eq!(matrix.at(0, 1), 3);
        assert_eq!(matrix.at(1, 0), 4);
        assert_eq!(matrix.at(1, 1), 6);
        assert_eq!(matrix.at(2, 0), 7);
        assert_eq!(matrix.at(2, 1), 9);
    }
}
