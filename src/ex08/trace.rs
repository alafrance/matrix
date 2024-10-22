use std::fmt::Debug;
use std::ops::AddAssign;
use crate::utils::matrix::matrix::Matrix;

impl<T: Clone + Debug + AddAssign + Default> Matrix<T> {
    fn trace(&mut self) -> T{
        if !self.is_square() {
            panic!("The matrix must be square");
        }
        let mut sum = T::default();
        let mut index;
        for i in 0..self.rows {
            index = i * self.cols + i;
            sum += self.data[index].clone();
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;
    use super::*;

    #[test]
    fn it_works() {

        let mut m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
        assert_eq!(15, m1.trace());

        let mut m1 = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        assert_eq!(5, m1.trace());

        let mut m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16 ], 4, 4);
        assert_eq!(34, m1.trace());
    }
    #[test]
    #[should_panic]
    fn it_panics() {
        let mut m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        m1.trace();
    }

    #[test]
    #[should_panic]
    fn it_panics_too() {
        let mut m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2, 5);
        m1.trace();
    }

    #[test]
    fn test_with_complex_numbers() {
        let mut m1 = Matrix::new(vec![Complex::new(1, 0), Complex::new(2, 0), Complex::new(3, 0), Complex::new(4, 0)], 2, 2);
        assert_eq!(Complex::new(5, 0), m1.trace());
    }
}
