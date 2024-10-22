use std::ops::{Add, AddAssign, MulAssign, SubAssign, Sub};
use crate::utils::vector::vector::Vector;

pub trait Calculation<T>: Clone + MulAssign + Add<T, Output=T> + AddAssign + SubAssign + Copy + Sub<Output=T> {}
impl<T: Clone + MulAssign + Add<T, Output=T> + AddAssign + SubAssign + Copy + Sub<Output=T>> Calculation<T> for T {}


impl<T: Calculation<T>> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, v: Vector<T>)-> Vector<T> {
        if v.size() != self.size() {
            panic!("The two vectors need to have the same size");
        }
        let new_data =  self.data.iter().enumerate().map(|(index, element)| {
            element.clone() + v.data[index]
        }).collect();
        Vector::new(new_data)
    }
}

impl<T: Calculation<T>> AddAssign for Vector<T>{
    fn add_assign(&mut self, rhs: Self) {
        if self.size() == 0 {
            return self.data = rhs.data.clone();
        }
        if rhs.size() != self.size() {
            panic!("The two vectors need to have the same size");
        }
        self.data.iter_mut().enumerate().for_each(|(index, element)| {
            *element += rhs.data[index]
        });
    }
}

impl<T: Calculation<T>> Sub for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, v: Vector<T>)-> Vector<T> {
        if v.size() != self.size() {
            panic!("The two vectors need to have the same size");
        }
        let new_data =  self.data.iter().enumerate().map(|(index, element)| {
            element.clone() - v.data[index]
        }).collect();
        Vector::new(new_data)
    }
}

impl<T: Calculation<T>> SubAssign for Vector<T> {
    fn sub_assign(&mut self, rhs: Self) {
        if rhs.size() != self.size() {
            panic!("The two vectors need to have the same size");
        }
        self.data.iter_mut().enumerate().for_each(|(index, element)| {
            *element -= rhs.data[index]
        });
    }
}

impl<T: Calculation<T>> Vector<T> {
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
        let v1 = Vector::new(vec![1, 2, 3]);
        let v2 = Vector::new(vec![4, 5, 6]);
        let v3 = v1 + v2;
        assert_eq!(v3.data, vec![5, 7, 9]);

        let mut v1 = Vector::new(vec![1, 2, 3]);
        let v2 = Vector::new(vec![4, 5, 6]);
        v1 += v2;
        assert_eq!(v1.data, vec![5, 7, 9]);
    }

    #[test]
    fn test_vector_add_assign_empty() {
        let mut v1 = Vector::new(vec![]);
        let v2 = Vector::new(vec![4, 5, 6]);
        v1 += v2;
        assert_eq!(v1.data, vec![4, 5, 6]);
    }

    #[test]
    #[should_panic]
    fn test_vector_add_should_panic() {
        let v1 = Vector::new(vec![1, 2, 3]);
        let v2 = Vector::new(vec![4, 5]);
        let _ = v1 + v2;
    }

    #[test]
    #[should_panic]
    fn test_vector_add_assign_should_panic() {
        let mut v1 = Vector::new(vec![1, 2, 3]);
        let v2 = Vector::new(vec![4, 5]);
        v1 += v2;
    }


    #[test]
    #[should_panic]
    fn test_vector_add_empty() {
        let v1 = Vector::new(vec![]);
        let v2 = Vector::new(vec![4, 5, 6]);
        let _ = v1 + v2;
    }

    #[test]
    fn test_vector_sub() {
        let v1 = Vector::new(vec![1, 2, 3]);
        let v2 = Vector::new(vec![4, 5, 6]);

        let v3 = v1 - v2;
        assert_eq!(v3.data, vec![-3, -3, -3]);

        let mut v1 = Vector::new(vec![1, 2, 3]);
        let v2 = Vector::new(vec![4, 5, 6]);
        v1 -= v2;
        assert_eq!(v1.data, vec![-3, -3, -3]);
    }

    #[test]
    #[should_panic]
    fn test_vector_sub_should_panic() {
        let v1 = Vector::new(vec![1, 2, 3]);
        let v2 = Vector::new(vec![4, 5]);
        let _ = v1 - v2;
    }

    #[test]
    #[should_panic]
    fn test_vector_sub_assign_should_panic() {
        let mut v1 = Vector::new(vec![1, 2, 3]);
        let v2 = Vector::new(vec![4, 5]);
        v1 -= v2;
    }

    #[test]
    #[should_panic]
    fn test_vector_sub_empty() {
        let v1 = Vector::new(vec![]);
        let v2 = Vector::new(vec![4, 5, 6]);
        let _ = v1 - v2;
    }

    #[test]
    fn test_vector_scl() {
        let mut v1 = Vector::new(vec![1, 2, 3]);
        v1.scl(2);
        assert_eq!(v1.data, vec![2, 4, 6]);
    }
}