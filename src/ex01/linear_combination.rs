use crate::ex00::add_substract_scale_vector::Calculation;
use crate::utils::vector::vector::Vector;

fn linear_combination<T: Calculation<T>>(vecs: &[Vector<T>], coefs: &[T]) -> Vector<T> {
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
            acc += v;
            acc
        })
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;
    use super::*;

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

    #[test]
    fn test_complex_numbers() {
        let e1 = Vector::<Complex<f64>>::from_array(vec![Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.)]);
        let e2 = Vector::<Complex<f64>>::from_array(vec![Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.)]);
        let e3 = Vector::<Complex<f64>>::from_array(vec![Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.)]);
        let v1 = Vector::<Complex<f64>>::from_array(vec![Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(3., 0.)]);
        let v2 = Vector::<Complex<f64>>::from_array(vec![Complex::new(0., 0.), Complex::new(10., 0.), Complex::new(-100., 0.)]);

        assert_eq!(Vector::<Complex<f64>>::from_array(vec![Complex::new(10., 0.), Complex::new(-2., 0.), Complex::new(0.5, 0.)]), linear_combination::<Complex<f64>>(&[e1, e2, e3], &[Complex::new(10., 0.), Complex::new(-2., 0.), Complex::new(0.5, 0.)]));
        assert_eq!(Vector::<Complex<f64>>::from_array(vec![Complex::new(10., 0.), Complex::new(0., 0.), Complex::new(230., 0.)]), linear_combination::<Complex<f64>>(&[v1, v2], &[Complex::new(10., 0.), Complex::new(-2., 0.)]));
    }
}
