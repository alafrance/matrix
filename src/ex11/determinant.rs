use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, Sub};
use crate::utils::matrix::matrix::Matrix;

impl<T> Matrix<T> where
T: Clone + Debug + Default + AddAssign,
T: Mul<T, Output = T> + Sub<T, Output = T> + Add<T, Output = T> + From<f64>
{
    pub fn determinant(&mut self) -> T {
        if !self.is_square() {
            panic!("Matrix must be square");
        }
        if self.rows == 2 {
            return self.determinant_2_2();
        }
        if self.rows == 3 {
            return self.determinant_3_3();
        }
        if self.rows == 4 {
            return self.determinant_4_4();
        }
        panic!("Matrix size not supported");
    }
    fn determinant_2_2(&self) -> T {
        self.at(0, 0) * self.at(1, 1)
            - self.at(0, 1) * self.at(1, 0)
    }

    fn determinant_3_3(&self) -> T {
        self.at(0,0) * Matrix::from_arrays(vec![
            vec![self.at(1, 1), self.at(1, 2)],
            vec![self.at(2, 1), self.at(2, 2)],
        ]).determinant_2_2()
        - self.at(0, 1) * Matrix::from_arrays(vec![
            vec![self.at(1, 0), self.at(1, 2)],
            vec![self.at(2, 0), self.at(2, 2)],
        ]).determinant_2_2()
        + self.at(0, 2) * Matrix::from_arrays(vec![
            vec![self.at(1, 0), self.at(1, 1)],
            vec![self.at(2, 0), self.at(2, 1)],
        ]).determinant_2_2()
    }

    fn determinant_4_4(&self) -> T {
        let mut det: T = T::default();
        for row in 0..self.rows {
            let mut sub_matrix = self.clone();
            sub_matrix.remove_row(row);
            sub_matrix.remove_col(0);
            let sub_det = sub_matrix.determinant();
            let sign = if row % 2 == 0 { T::from(1.0) } else { T::from(-1.0) };
            det += self.at(row, 0) * sub_det * sign;
        }
        det
    }
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;
    use crate::utils::matrix::matrix::Matrix;

    #[test]
    fn it_works() {
        let mut u = Matrix::from_arrays(vec![
            vec![ 1., -1.],
            vec![-1., 1.],
        ]);
        assert_eq!(u.determinant(), 0.);
        let mut u = Matrix::from_arrays(vec![
            vec![2., 0., 0.],
            vec![0., 2., 0.],
            vec![0., 0., 2.],
        ]);
        assert_eq!(u.determinant(), 8.);

        let mut u = Matrix::from_arrays(vec![
        vec![8., 5., -2.],
        vec![4., 7., 20.],
        vec![7., 6., 1.],
        ]);
        assert_eq!(u.determinant(), -174.);

        let mut u = Matrix::from_arrays(vec![
            vec![ 8., 5., -2., 4.],
            vec![ 4., 2.5, 20., 4.],
            vec![ 8., 5., 1., 4.],
            vec![28., -4., 17., 1.],
        ]);
        assert_eq!(u.determinant(), 1032.);
    }

    #[test]
    fn test_with_complex_numbers() {
        let mut u = Matrix::<Complex<f64>>::new(vec![Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(3., 0.), Complex::new(4., 0.)], 2, 2);
        assert_eq!(u.determinant(), Complex::new(-2., 0.));
    }
}