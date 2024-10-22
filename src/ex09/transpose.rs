use std::fmt::Debug;
use crate::utils::matrix::matrix::Matrix;

impl<T: Clone + Debug + PartialEq> Matrix<T> {
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
    use num_complex::Complex;
    use crate::utils::matrix::matrix::Matrix;

    #[test]
    fn it_works() {
        let m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let m2 = Matrix::new(vec![1, 4, 2, 5, 3, 6], 3, 2);
        assert_eq!(m2, m1.transpose());
    }

    #[test]
    fn test_with_complex_numbers() {
        let m1 = Matrix::<Complex<f64>>::new(vec![Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(3., 0.), Complex::new(4., 0.)], 2, 2);
        let m2 = Matrix::<Complex<f64>>::new(vec![Complex::new(1., 0.), Complex::new(3., 0.), Complex::new(2., 0.), Complex::new(4., 0.)], 2, 2);
        assert_eq!(m2, m1.transpose());
    }
}