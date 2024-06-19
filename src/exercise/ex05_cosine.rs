use std::fmt::Debug;
use std::ops::{Add, Div, Mul};
use num_traits::{Float, Pow};
use crate::models::vector::Vector;

fn angle_cos<T>(u: &Vector::<T>, v: &Vector::<T>) -> f32 where
        for<'a> &'a T: Mul<&'a T, Output = T>,
        f32: Add<T, Output = T>,
        T: Clone + Debug + Add<f32> + Pow<T, Output = T> + Div<f32, Output = f32>,
        T: Float + for<'a> Add<<&'a T as Mul<&'a T, >>::Output, Output = T>
{
    let dot_product = u.dot(v.clone());
    let norm_u = u.norm();
    let norm_v = v.norm();
    dot_product / (norm_u * norm_v)
}

#[cfg(test)]
mod tests {
    use crate::exercise::ex05_cosine::angle_cos;
    use crate::models::vector::Vector;

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
}