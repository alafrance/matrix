use std::fmt::Debug;
use std::ops::Mul;
use num_traits::Zero;
use crate::models::vector::Vector;

impl<T: Clone + Debug + Zero> Vector::<T> where for<'a> &'a T: Mul<&'a T, Output = T>{
    pub(crate) fn dot(&self, v: Vector::<T>) -> T {
        self.data
            .iter()
            .zip(v.data.iter())
            .fold(T::zero(), |acc, (x, y)| acc + x * y)
    }
}