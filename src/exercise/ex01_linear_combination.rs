use std::fmt::Debug;
use std::ops::{AddAssign, MulAssign, SubAssign};
use crate::models::vector::Vector;

fn linear_combination<T:  Clone + Debug + MulAssign + AddAssign + SubAssign + Copy>(vecs: &[Vector<T>], coefs: &[T]) -> Vector<T> {
    if vecs.len() != coefs.len() {
        panic!("The number of vectors and coefficients must be the same");
    }
    if vecs.is_empty() {
        panic!("The vectors array is empty");
    }
    vecs.iter()
        .enumerate()
        .map(|(index, vector)| {
            let mut v = vector.clone();
            v.scl(coefs[index]);
            v
        })
        .fold(Vector::new(vec![]), |mut acc, v| {
            acc.add(&v);
            acc
        })
}

#[cfg(test)]
mod tests {
    use crate::exercise::ex01_linear_combination::linear_combination;
    use crate::models::vector::Vector;

    #[test]
    fn it_works() {
        let e1 = Vector::<f64>::from_array(vec![1., 0., 0.]);
        let e2 = Vector::<f64>::from_array(vec![0., 1., 0.]);
        let e3 = Vector::<f64>::from_array(vec![0., 0., 1.]);
        let v1 = Vector::<f64>::from_array(vec![1., 2., 3.]);
        let v2 = Vector::<f64>::from_array(vec![0., 10., -100.]);

        assert_eq!(Vector::<f64>::from_array(vec![10., -2., 0.5]), linear_combination::<f64>(&[e1, e2, e3], &[10., -2., 0.5]));
        assert_eq!(Vector::<f64>::from_array(vec![10., 0., 230.]), linear_combination::<f64>(&[v1, v2], &[10., -2.]));
    }
}
