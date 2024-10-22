use std::fmt::Debug;
use std::ops::{Mul, Sub};
use crate::utils::vector::vector::Vector;

fn cross_product<T: Clone + Debug + Mul<T, Output = T> + Sub<T, Output= T>>(u: &Vector<T>, v: &Vector<T>) -> Vector<T> {
    if u.size() != 3 || v.size() != 3 {
        panic!("Vectors must have 3 elements");
    }
    Vector {
        data: vec![
            u.y() * v.z() - u.z() * v.y(),
            u.z() * v.x() - u.x() * v.z(),
            u.x() * v.y() - u.y() * v.x()
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::vector::vector::Vector;

    #[test]
    fn it_works() {
        let v = Vector::<f32>::new(vec![0., 0., 1.]);
        let v2 = Vector::<f32>::new(vec![1., 0., 0.]);
        assert_eq!(Vector::<f32>::new(vec![0., 1., 0.]), cross_product(&v, &v2));

        let v = Vector::<f32>::new(vec![1., 0., 0.]);
        let v2 = Vector::<f32>::new(vec![0., 1., 0.]);
        assert_eq!(Vector::<f32>::new(vec![0., 0., 1.]), cross_product(&v, &v2));

        let v = Vector::<f32>::new(vec![1., 0., 0.]);
        let v2 = Vector::<f32>::new(vec![1., 0., 0.]);
        assert_eq!(Vector::<f32>::new(vec![0., 0., 0.]), cross_product(&v, &v2));

        let v = Vector::<f32>::new(vec![1., 0., 0.]);
        let v2 = Vector::<f32>::new(vec![-1., 0., 0.]);
        assert_eq!(Vector::<f32>::new(vec![0., 0., 0.]), cross_product(&v, &v2));
    }

}