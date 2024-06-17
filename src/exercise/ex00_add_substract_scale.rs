use std::fmt::Debug;
use std::ops::{AddAssign, MulAssign, SubAssign};
use crate::models::matrix::Matrix;
use crate::models::vector::Vector;

impl<T: Clone + Debug + MulAssign + AddAssign + SubAssign + Copy> Vector<T> {
    pub fn add(&mut self, v: &Vector<T>) {
        if self.is_empty() {
            return self.data = v.data.clone();
        }
        if v.size() != self.size() {
            panic!("The two vectors need to have the same size");
        }
        self.data.iter_mut().enumerate().for_each(|(index, element)| {
            *element += v.data[index]
        });
    }

    pub fn sub(&mut self, v: &Vector<T>) {
        if v.size() != self.size() {
            panic!("The two vectors need to have the same size");
        }
        self.data.iter_mut().enumerate().for_each(|(index, element)| {
            *element -= v.data[index]
        });
    }

    pub fn scl(&mut self, a: T) {
        self.data.iter_mut().for_each(|element| {
            *element *= a
        });
    }
}


impl<T: Clone + Debug + MulAssign + AddAssign + SubAssign + Copy> Matrix<T> {
    pub fn add(&mut self, v: &Matrix<T>) {
        if v.rows != self.rows || v.cols != self.cols {
            panic!("The two matrices need to have the same size");
        }
        self.data.iter_mut().enumerate().for_each(|(index, element)| {
            *element += v.data[index]
        });
    }

    pub fn sub(&mut self, v: &Matrix<T>) {
        if v.rows != self.rows || v.cols != self.cols {
            panic!("The two matrices need to have the same size");
        }
        self.data.iter_mut().enumerate().for_each(|(index, element)| {
            *element -= v.data[index]
        });
    }

    pub fn scl(&mut self, a: T) {
        self.data.iter_mut().for_each(|element| {
            *element *= a
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_add() {
        let mut v1 = Vector::new(vec![1, 2, 3]);
        let v2 = Vector::new(vec![4, 5, 6]);
        v1.add(&v2);
        assert_eq!(v1.data, vec![5, 7, 9]);
    }

    #[test]
    fn test_vector_sub() {
        let mut v1 = Vector::new(vec![1, 2, 3]);
        let v2 = Vector::new(vec![4, 5, 6]);
        v1.sub(&v2);
        assert_eq!(v1.data, vec![-3, -3, -3]);
    }

    #[test]
    fn test_vector_scl() {
        let mut v1 = Vector::new(vec![1, 2, 3]);
        v1.scl(2);
        assert_eq!(v1.data, vec![2, 4, 6]);
    }

    #[test]
    fn test_matrix_add() {
        let mut m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let m2 = Matrix::new(vec![7, 8, 9, 10, 11, 12], 2, 3);
        m1.add(&m2);
        assert_eq!(m1.data, vec![8, 10, 12, 14, 16, 18]);
    }

    #[test]
    fn test_matrix_sub() {
        let mut m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let m2 = Matrix::new(vec![7, 8, 9, 10, 11, 12], 2, 3);
        m1.sub(&m2);
        assert_eq!(m1.data, vec![-6, -6, -6, -6, -6, -6]);
    }

    #[test]
    fn test_matrix_scl() {
        let mut m1 = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        m1.scl(2);
        assert_eq!(m1.data, vec![2, 4, 6, 8, 10, 12]);
    }
}