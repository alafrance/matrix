use std::fmt::Debug;
use std::ops::Add;
use num_complex::Complex;
use crate::utils::vector::vector::Vector;
use num_traits::{Float, NumCast, Pow};

impl<T: Clone + Debug + Float + Pow<T, Output = T> + Add<f32>> Vector::<T> where f32: Add<T, Output = T>{

    fn norm_1(&self) -> f32 {
        self.data
            .iter()
            .fold(T::zero(), |acc, x| acc + x.abs())
            .to_f32()
            .unwrap()
    }

    pub(crate) fn norm(&self) -> f32 {
        self.data
            .iter()
            .fold(T::zero(), |acc, x| acc + x.powi(2))
            .sqrt()
            .to_f32()
            .unwrap()
    }

    fn norm_inf(&self) -> f32 {
        self.data
            .iter()
            .fold(T::zero(), |acc: T, x| acc.max(x.abs()))
            .to_f32()
            .unwrap()
    }
}

impl<T> Vector<Complex<T>>
    where
        T: Clone + Debug + Float + NumCast,
{
    fn norm_1_complex(&self) -> T {
        self.data
            .iter()
            .fold(T::zero(), |acc, z| {
                acc + z.re.abs() + z.im.abs()
            })
    }

    pub fn norm_complex(&self) -> T {
        self.data
            .iter()
            .fold(T::zero(), |acc, x| acc + x.norm_sqr())
            .sqrt()
    }

    pub fn norm_inf_complex(&self) -> T {
        self.data
            .iter()
            .fold(T::zero(), |acc, x| acc.max(x.norm()))
    }
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;
    use crate::utils::vector::vector::Vector;

    #[test]
    fn it_works() {
        let v = Vector::<f32>::new(vec![1., 2., 3.]);
        assert_eq!(v.norm_1(), 6.);
        assert_eq!(v.norm().round(), 4.);
        assert_eq!(v.norm_inf(), 3.);

        let v = Vector::<f32>::new(vec![-1., -2.]);
        assert_eq!(v.norm_1(), 3.);
        assert_eq!(v.norm().round(), 2.);
        assert_eq!(v.norm_inf(), 2.);
    }

    #[test]
    fn test_complex_numbers() {
        let v = Vector::<Complex<f64>>::from_array(vec![Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(3., 0.)]);
        assert_eq!(v.norm_1_complex(), 6.);
        assert_eq!(v.norm_complex().round(), 4.);
        assert_eq!(v.norm_inf_complex(), 3.);
    }

}