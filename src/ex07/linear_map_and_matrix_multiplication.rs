use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{AddAssign, Mul, MulAssign, SubAssign};
use num_complex::Complex;
use num_traits::Float;
use crate::utils::matrix::matrix::Matrix;
use crate::utils::vector::vector::Vector;

impl<T> Matrix<T> where
    T: MulAssign + AddAssign + SubAssign + Copy + Clone + Debug + Default + AddAssign + Sum + Mul<T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>
{

    fn mul_vec(&mut self, vec: Vector<T>) -> Vector<T> {
        if self.cols != vec.size() {
            panic!("The number of columns in the matrix must be the same as the size of the vector");
        }
        let mut sum = Vector::new(vec![T::default(); self.rows]);
        for i in 0..self.rows {
            let v= self.get_row_vector(i);
            sum.data[i] = v.dot(vec.clone());
        }
        sum
    }

    fn mul_mat(&mut self, mat: Matrix<T>) -> Matrix<T> {
        if self.cols != mat.rows {
            panic!("The number of columns in the first matrix must be the same as the number of rows in the second matrix");
        }
        let mut result = Matrix::new(vec![T::default(); self.rows * mat.cols], self.rows, mat.cols);
        for i in 0..self.rows {
            for j in 0..mat.cols {
                let v = self.get_row_vector(i);
                let v2 = mat.get_col_vector(j);
                result.data[i * mat.cols + j] = v.dot(v2);
            }
        }
        result
    }

}

impl<T> Matrix<Complex<T>> where
    T: Clone + Float + Debug + Default
{
    // Multiplication Matrice x Vecteur
    pub fn mul_vec_complex(&mut self, vec: Vector<Complex<T>>) -> Vector<Complex<T>> {
        if self.cols != vec.size() {
            panic!("The number of columns in the matrix must be the same as the size of the vector");
        }
        let mut sum = Vector::new(vec![Complex::default(); self.rows]);
        for i in 0..self.rows {
            let v = self.get_row_vector(i);
            sum.data[i] = v.dot_complex(vec.clone());
        }
        sum
    }

    // Multiplication Matrice x Matrice
    pub fn mul_mat_complex(&mut self, mat: Matrix<Complex<T>>) -> Matrix<Complex<T>> {
        if self.cols != mat.rows {
            panic!("The number of columns in the first matrix must be the same as the number of rows in the second matrix");
        }
        let mut result = Matrix::new(vec![Complex::default(); self.rows * mat.cols], self.rows, mat.cols);
        for i in 0..self.rows {
            for j in 0..mat.cols {
                let v = self.get_row_vector(i);
                let v2 = mat.get_col_vector(j);
                result.data[i * mat.cols + j] = v.dot_complex(v2);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::vector::vector::Vector;

    #[test]
    fn it_works() {
        let mut m1 = Matrix::new(vec![1., 2., 3., 4., 5., 6.], 2, 3);
        let v = Vector::<f32>::new(vec![1., 2., 3.]);
        assert_eq!(Vector::<f32>::new(vec![14., 32.]), m1.mul_vec(v));

        let mut m1 = Matrix::new(vec![1., 2., 3., 4., 5., 6., 7., 8., 9.], 3, 3);
        let m2 = Matrix::new(vec![1., 2., 3., 4., 5., 6., 7., 8., 9.], 3, 3);
        let result =  Matrix::new(vec![30., 36., 42., 66., 81., 96., 102., 126., 150.], 3, 3);
        assert_eq!(result, m1.mul_mat(m2));
    }

    #[test]
    #[should_panic]
    fn it_panics_mul_vec() {
        let mut m1 = Matrix::new(vec![1., 2., 3., 4., 5., 6.], 2, 3);
        let v = Vector::<f32>::new(vec![1., 2.]);
        m1.mul_vec(v);

    }

    #[test]
    #[should_panic]
    fn it_panics_mul_mat() {
        let mut m1 = Matrix::new(vec![1., 2., 3., 4., 5., 6., 7., 8., 9.], 3, 3);
        let m2 = Matrix::new(vec![1., 2., 3., 4., 5., 6.], 2, 3);
        m1.mul_mat(m2);
    }

    #[test]
    fn test_complex_numbers() {
        let mut m1 = Matrix::new(vec![Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(3., 0.), Complex::new(4., 0.), Complex::new(5., 0.), Complex::new(6., 0.)], 2, 3);
        let v = Vector::<Complex<f64>>::from_array(vec![Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(3., 0.)]);
        assert_eq!(Vector::<Complex<f64>>::from_array(vec![Complex::new(14., 0.), Complex::new(32., 0.)]), m1.mul_vec_complex(v));

        let mut m1 = Matrix::new(vec![Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(3., 0.), Complex::new(4., 0.), Complex::new(5., 0.), Complex::new(6., 0.), Complex::new(7., 0.), Complex::new(8., 0.), Complex::new(9., 0.)], 3, 3);
        let m2 = Matrix::new(vec![Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(3., 0.), Complex::new(4., 0.), Complex::new(5., 0.), Complex::new(6., 0.), Complex::new(7., 0.), Complex::new(8., 0.), Complex::new(9., 0.)], 3, 3);
        let result =  Matrix::new(vec![Complex::new(30., 0.), Complex::new(36., 0.), Complex::new(42., 0.), Complex::new(66., 0.), Complex::new(81., 0.), Complex::new(96., 0.), Complex::new(102., 0.), Complex::new(126., 0.), Complex::new(150., 0.)], 3, 3);
        assert_eq!(result, m1.mul_mat_complex(m2));
    }

}