use std::fmt::Debug;
use crate::models::matrix::Matrix;

impl<T: Clone + Debug> Matrix<T> {
    // pub fn transpose(&self) -> Matrix<T> {
    //     let mut data = Vec::new();
    //     for i in 0..self.cols {
    //         for j in 0..self.rows {
    //             data.push(self.data[j * self.cols + i].clone());
    //         }
    //     }
    //     Matrix::new(data, self.cols, self.rows)
    // }
}