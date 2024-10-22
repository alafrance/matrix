use std::ops::{Add, AddAssign, MulAssign, SubAssign, Sub};
use crate::utils::matrix::matrix::Matrix;

pub trait Calculation<T>: Clone + MulAssign + Add<T, Output=T> + AddAssign + SubAssign + Copy + Sub<Output=T> {}
impl<T: Clone + MulAssign + Add<T, Output=T> + AddAssign + SubAssign + Copy + Sub<Output=T>> Calculation<T> for T {}

impl<T: Calculation<T>> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, v: Matrix<T>)-> Matrix<T> {
        if v.rows != self.rows || v.cols != self.cols {
            panic!("The two matrices need to have the same size");
        }
        let new_data = self.data.iter().enumerate().map(|(index, element)| {
            element.clone() + v.data[index]
        }).collect();
        Matrix::new(new_data, self.rows, self.cols)
    }
}

impl<T: Calculation<T>> AddAssign for Matrix<T> {
    fn add_assign(&mut self, v: Matrix<T>) {
        if v.rows != self.rows || v.cols != self.cols {
            panic!("The two matrices need to have the same size");
        }
        self.data.iter_mut().enumerate().for_each(|(index, element)| {
            *element += v.data[index]
        });
    }
}

impl<T: Calculation<T>> Sub for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, v: Matrix<T>)-> Matrix<T> {
        if v.rows != self.rows || v.cols != self.cols {
            panic!("The two matrices need to have the same size");
        }
        let new_data = self.data.iter().enumerate().map(|(index, element)| {
            element.clone() - v.data[index]
        }).collect();
        Matrix::new(new_data, self.rows, self.cols)
    }
}

impl<T: Calculation<T>> SubAssign for Matrix<T> {
    fn sub_assign(&mut self, v: Matrix<T>) {
        if v.rows != self.rows || v.cols != self.cols {
            panic!("The two matrices need to have the same size");
        }
        self.data.iter_mut().enumerate().for_each(|(index, element)| {
            *element -= v.data[index]
        });
    }
}


impl<T: Calculation<T>> Matrix<T> {
    pub fn scl(&mut self, a: T) {
        self.data.iter_mut().for_each(|element| {
            *element *= a;
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::matrix::matrix::Matrix;

    #[test]
    fn test_matrix_add() {
        let m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let m2 = Matrix::new(vec![7, 8, 9, 10, 11, 12], 2, 3);
        let m3 = m1 + m2;
        assert_eq!(m3.data, vec![8, 10, 12, 14, 16, 18]);

        let mut m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let m2 = Matrix::new(vec![7, 8, 9, 10, 11, 12], 2, 3);
        m1 += m2;
        assert_eq!(m1.data, vec![8, 10, 12, 14, 16, 18]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_add_empty() {
        let m1 = Matrix::new(vec![], 0, 0);
        let m2 = Matrix::new(vec![7, 8, 9, 10, 11, 12], 2, 3);
        let _ = m1 + m2;
    }

    #[test]
    #[should_panic]
    fn test_matrix_add_panic() {
        let m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let m2 = Matrix::new(vec![7, 8, 9, 10], 2, 2);
        let _ = m1 + m2;
    }

    #[test]
    fn test_matrix_sub() {
        let m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let m2 = Matrix::new(vec![7, 8, 9, 10, 11, 12], 2, 3);
        let m3 = m1 - m2;
        assert_eq!(m3.data, vec![-6, -6, -6, -6, -6, -6]);

        let mut m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let m2 = Matrix::new(vec![7, 8, 9, 10, 11, 12], 2, 3);
        m1 -= m2;
        assert_eq!(m1.data, vec![-6, -6, -6, -6, -6, -6]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_sub_empty() {
        let m1 = Matrix::new(vec![], 0, 0);
        let m2 = Matrix::new(vec![7, 8, 9, 10, 11, 12], 2, 3);
        let _ =  m1 - m2;
    }

    #[test]
    #[should_panic]
    fn test_matrix_sub_panic() {
        let m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let m2 = Matrix::new(vec![7, 8, 9, 10, 11, 12], 2, 2);
        let _ = m1 - m2;
    }

    #[test]
    #[should_panic]
    fn test_matrix_sub_assign_panic() {
        let mut m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let m2 = Matrix::new(vec![7, 8, 9, 10, 11, 12], 2, 2);
        m1 -= m2;
    }

    #[test]
    fn test_matrix_scl() {
        let mut m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        m1.scl(2);
        assert_eq!(m1.data, vec![2, 4, 6, 8, 10, 12]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_scl_empty() {
        let mut m1 = Matrix::new(vec![], 0, 0);
        m1.scl(2);
    }

    #[test]
    fn test_add_complex_numbers() {
        use num_complex::Complex;
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        let v1 = Matrix::new(vec![c1], 1, 1);
        let v2 = Matrix::new(vec![c2], 1, 1);

        let v3 = v1 + v2;
        assert_eq!(v3.data, vec![Complex::new(4.0, 6.0)]);

        let mut v1 = Matrix::new(vec![c1], 1, 1);
        let v2 = Matrix::new(vec![c2], 1, 1);
        v1 += v2;
        assert_eq!(v1.data, vec![Complex::new(4.0, 6.0)]);
    }

    #[test]
    fn test_complex_numbers_sub() {
        use num_complex::Complex;
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        let v1 = Matrix::new(vec![c1], 1, 1);
        let v2 = Matrix::new(vec![c2], 1, 1);

        let v3 = v1 - v2;
        assert_eq!(v3.data, vec![Complex::new(-2.0, -2.0)]);

        let mut v1 = Matrix::new(vec![c1], 1, 1);
        let v2 = Matrix::new(vec![c2], 1, 1);
        v1 -= v2;
        assert_eq!(v1.data, vec![Complex::new(-2.0, -2.0)]);
    }

    #[test]
    fn test_complex_numbers_scl() {
        use num_complex::Complex;
        let c1 = Complex::new(1.0, 2.0);
        let mut v1 = Matrix::new(vec![c1], 1, 1);

        v1.scl(Complex::new(2.0, 0.0));
        assert_eq!(v1.data, vec![Complex::new(2.0, 4.0)]);
    }
}