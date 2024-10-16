use std::fmt::Debug;
use std::ops::Add;
use crate::models::vector::Vector;
use num_traits::{Float, Pow};

impl<T: Clone + Debug + Float + Pow<T, Output = T> + Add<f32>> Vector::<T> where f32: Add<T, Output = T>{

    // Taxicab norm from the origin
    fn norm_1(&self) -> f32 {
        self.data
            .iter()
            .fold(T::zero(), |acc, x| acc + x.abs()).to_f32().unwrap()
    }

    // euclidian norm from the origin
    pub(crate) fn norm(&self) -> f32 {
        self.data.iter().fold(T::zero(), |acc, x| acc + x.powi(2)).sqrt().to_f32().unwrap()
    }

    // supremum norm from the origin
    fn norm_inf(&self) -> f32 {
        self.data.iter().fold(T::zero(), |acc: T, x| acc.max(x.abs())).to_f32().unwrap()
    }

}

#[cfg(test)]
mod tests {
    use crate::models::vector::Vector;

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

}