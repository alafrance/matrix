use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use crate::models::matrix::Matrix;

pub struct Vector<T: Clone + Debug> {
    pub(crate) data: Vec<T>,
}

// Utils
impl<T: Clone + Debug> Vector<T> {
    pub fn new(array: Vec<T>) -> Vector<T> {
        Vector {
            data: array,
        }
    }

    pub fn from_array(array: Vec<T>) -> Vector<T> {
        Vector { data: array }
    }

    pub fn as_matrix(&self) -> Matrix<T> {
        Matrix::from_vector(self)
    }

    pub fn as_array(&self) -> Vec<T> {
        self.data.clone()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn x(&self) -> T {
        if self.size() < 1 {
            panic!("Vector is empty");
        }
        self.data[0].clone()
    }

    pub fn y(&self) -> T {
        if self.size() < 2 {
            panic!("Vector has less than 2 elements");
        }
        self.data[1].clone()
    }

    pub fn z(&self) -> T {
        if self.size() < 3 {
            panic!("Vector has less than 3 elements");
        }
        self.data[2].clone()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn print(&self) {
        println!("[{:?}]", self.data);
    }
}

// Debug and Display
impl<T: Clone + Debug> Debug for Vector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T: Clone + Debug> Display for Vector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T: Clone + Debug> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Vector {
            data: self.data.clone(),
        }
    }
}

impl<T: Clone + Debug + PartialEq> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Mul<f32> for Vector<f32> {
    type Output = Vector<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut v = self.clone();
        v.scl(rhs);
        v
    }
}

impl<T: Clone + Debug + MulAssign + AddAssign + SubAssign + Copy> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut v = self.clone();
        v.add_vector(&rhs);
        v
    }
}

impl<T: Clone + Debug + MulAssign + AddAssign + SubAssign + Copy> Sub for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut v = self.clone();
        v.sub_vector(&rhs);
        v
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        assert_eq!(vector.size(), 3);
        assert_eq!(vector.x(), 1);
        assert_eq!(vector.y(), 2);
        assert_eq!(vector.z(), 3);
    }

    #[test]
    fn test_as_matrix() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        let matrix = vector.as_matrix();
        assert_eq!(matrix.rows(), 3);
        assert_eq!(matrix.cols(), 1);
    }

    #[test]
    fn test_as_array() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        let array = vector.as_array();
        assert_eq!(array, vec![1, 2, 3]);
    }

    #[test]
    fn test_is_empty() {
        let vector: Vector<i32> = Vector::new(vec![]);
        assert_eq!(vector.is_empty(), true);
    }

    #[test]
    fn test_print() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        vector.print();
    }

    #[test]
    fn test_x() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        assert_eq!(vector.x(), 1);
    }

    #[test]
    #[should_panic]
    fn test_x_panic() {
        let vector: Vector<i32> = Vector::new(vec![]);
        vector.x();
    }

    #[test]
    fn test_y() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        assert_eq!(vector.x(), 2);
    }

    #[test]
    #[should_panic]
    fn test_y_panic() {
        let vector = Vector::from_array(vec![1]);
        vector.y();
    }

    #[test]
    fn test_z() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        assert_eq!(vector.x(), 3);
    }

    #[test]
    #[should_panic]
    fn test_z_panic() {
        let vector = Vector::from_array(vec![1, 2]);
        vector.z();
    }

}