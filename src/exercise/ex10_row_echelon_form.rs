use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use crate::models::matrix::Matrix;

impl<T: Clone + Debug + Default + PartialEq + Add<T, Output= T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output= T>> Matrix<T> {
    fn row_echelon(&mut self) -> Matrix<T> {
        let mut m = self.clone();
        let mut index_pivot = 0;
        m.data.fin
        todo!()
    }

    pub fn is_row_echelon(&self) -> bool {
        for row in 0..self.rows {
            for cols in 0..self.cols {
                if cols < row && self.data[row * self.cols + cols] != T::default() {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::models::matrix::Matrix;

    #[test]
    fn it_works() {
        let mut m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let m2 = Matrix::new(vec![1, 2, 3, 0, 1, 2], 2, 3);
        assert_eq!(m2, m1.row_echelon());
        assert_eq!(true, m1.row_echelon().is_row_echelon());
    }
}