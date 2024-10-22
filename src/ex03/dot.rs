use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{AddAssign, Mul};
use num_complex::Complex;
use num_traits::Float;
use crate::utils::vector::vector::Vector;

impl<T: Mul<T, Output = T> + Clone + Debug + AddAssign + Sum> Vector<T>
    where
        for<'a> &'a T: Mul<&'a T, Output = T>
{
    pub fn dot(&self, v: Vector<T>) -> T {
        if self.size() != v.size() {
            panic!("The two vectors need to have the same size");
        }
        self.data
            .iter()
            .zip(v.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl<T> Vector<Complex<T>>
    where
        T: Float
{
    pub fn dot_complex(&self, v: Vector<Complex<T>>) -> Complex<T> {
        if self.data.len() != v.data.len() {
            panic!("The two vectors need to have the same size");
        }

        self.data
            .iter()
            .zip(v.data.iter())
            .map(|(a, b)| *a * b.conj())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;
    use crate::utils::vector::vector::Vector;

    #[test]
    fn it_works() {
        let v = Vector::<f32>::new(vec![2., 1.]);
        let v2 = Vector::<f32>::new(vec![4., 2.]);
        assert_eq!(10., v.dot(v2));

        let v = Vector::<f32>::new(vec![1., 2., 3.]);
        let v2 = Vector::<f32>::new(vec![4., 5., 6.]);
        assert_eq!(32., v.dot(v2));

    }
    #[test]
    #[should_panic]
    fn it_panics() {
        let v = Vector::<f32>::new(vec![1., 2., 3.]);
        let v2 = Vector::<f32>::new(vec![4., 5., 6., 7.]);
        v.dot(v2);
    }

    #[test]
    fn test_complex_numbers() {
        let v = Vector::<Complex<f64>>::from_array(vec![Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(3., 0.)]);
        let v2 = Vector::<Complex<f64>>::from_array(vec![Complex::new(4., 0.), Complex::new(5., 0.), Complex::new(6., 0.)]);
        assert_eq!(Complex::new(32., 0.), v.dot_complex(v2));
    }
}