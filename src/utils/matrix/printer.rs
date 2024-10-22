use crate::utils::matrix::matrix::Matrix;
use crate::utils::matrix::traits::printable::Printable;

impl<T: Printable> Matrix<T> {
    pub fn print(&self) {
        for i in 0..self.rows {
            let start = i * self.cols;
            let end = start + self.cols;
            println!("[{:?}]", &self.data[start..end]);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::matrix::matrix::Matrix;

    #[test]
    fn test_print() {
        let matrix = Matrix::from_arrays(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);
        matrix.print();
    }
}