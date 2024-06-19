use std::fmt::Debug;
use crate::models::matrix::Matrix;

impl<T: Clone + Debug> Matrix<T> {
    pub fn transpose(&self) -> Matrix<T> {
        let mut data = Vec::new();
        for i in 0..self.cols {
            for j in 0..self.rows {
                data.push(self.data[j * self.cols + i].clone());
            }
        }
        Matrix::new(data, self.cols, self.rows)
    }
}

#[cfg(test)]
mod tests{
    use crate::models::matrix::Matrix;

    #[test]
    fn it_works() {
        let m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let m2 = Matrix::new(vec![1, 4, 2, 5, 3, 6], 3, 2);
        assert_eq!(m2, m1.transpose());
    }
}