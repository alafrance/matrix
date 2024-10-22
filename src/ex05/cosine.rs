use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul};
use num_complex::{Complex, ComplexFloat};
use num_traits::{Float, Pow, ToPrimitive};
use crate::utils::vector::vector::Vector;

fn angle_cos<T>(u: &Vector::<T>, v: &Vector::<T>) -> f32 where
        for<'a> &'a T: Mul<&'a T, Output = T>,
        f32: Add<T, Output = T>,
        T: Mul<T, Output = T> + Clone + Debug + AddAssign + Sum,
        T: Clone + Debug + Add<f32> + Pow<T, Output = T> + Div<f32, Output = f32>,
        T: Float + for<'a> Add<<&'a T as Mul<&'a T, >>::Output, Output = T>
{
    u.dot(v.clone()) / (u.norm() * v.norm())
}

fn angle_cos_complex<T>(u: &Vector::<Complex<T>>, v: &Vector::<Complex<T>>) -> Complex<T> where
    T: Float + ComplexFloat + Debug + ToPrimitive,
{
    u.dot_complex(v.clone()) / (u.norm_complex() * v.norm_complex())
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;
    use super::*;
    use crate::utils::vector::vector::Vector;

    #[test]
    fn it_works() {
        let v = Vector::<f32>::new(vec![1., 0.]);
        let v2 = Vector::<f32>::new(vec![0., 1.]);
        assert_eq!(0., angle_cos(&v, &v2));

        let v = Vector::<f32>::new(vec![1., 0.]);
        let v2 = Vector::<f32>::new(vec![1., 0.]);
        assert_eq!(1., angle_cos(&v, &v2));

        let v = Vector::<f32>::new(vec![1., 0.]);
        let v2 = Vector::<f32>::new(vec![-1., 0.]);
        assert_eq!(-1., angle_cos(&v, &v2));
    }

    #[test]
    fn test_complex_numbers() {
        let v = Vector::<Complex<f64>>::new(vec![Complex::new(1., 0.) ]);
        let v2 = Vector::<Complex<f64>>::new(vec![Complex::new(0., 1.)]);
        assert_eq!(Complex::new(0., -1.), angle_cos_complex(&v, &v2));
    }
}