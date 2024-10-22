use std::ops::{Add, Mul};
use crate::utils::matrix::matrix::Matrix;
use crate::utils::vector::vector::Vector;

impl Mul<f32> for Vector<f32> {
    type Output = Vector<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut v = self.clone();
        v.scl(rhs);
        v
    }
}

impl Mul<f32> for Matrix<f32> {
    type Output = Matrix<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut v = self.clone();
        v.scl(rhs);
        v
    }
}

fn lerp<V: Mul<f32, Output = V> + Add<Output = V>>(u: V, v: V, t: f32) -> V {
    if t < 0. || t > 1. {
        panic!("The interpolation factor must be between 0 and 1");
    }
    return u * (1. - t) + (v * t);
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::matrix::matrix::Matrix;
    use crate::utils::vector::vector::Vector;

    #[test]
    fn it_works() {
        assert_eq!(2.6, lerp(2., 4., 0.3));
        assert_eq!(1.3, lerp(1., 2., 0.3));
        let v = Vector::<f32>::new(vec![2., 1.]);
        let v2 = Vector::<f32>::new(vec![4., 2.]);
        let result = Vector::<f32>::new(vec![2.6, 1.3]);
        assert_eq!(result, lerp(v, v2, 0.3));

        let m = Matrix::<f32>::from_arrays(vec![vec![2., 1.], vec![3., 4.]]);
        let m2 = Matrix::<f32>::from_arrays(vec![vec![20., 10.], vec![30., 40.]]);
        let result = Matrix::<f32>::from_arrays(vec![vec![11., 5.5], vec![16.5, 22.]]);
        assert_eq!(result, lerp(m, m2, 0.5));
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        lerp(1., 2., 1.1);
    }
}