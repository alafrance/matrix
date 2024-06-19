use crate::models::matrix::Matrix;
use crate::models::vector::Vector;

impl<T> Matrix<T> {

    fn mul_vec<T>(&mut self, vec: Vector<T>) -> Vector<T> {

    }

    fn mul_mat<T>(&mut self, mat: Matrix<T>) -> Matrix<T> {

    }

}

#[cfg(test)]
mod tests {
    use crate::exercise::ex07_matrix_multiplication_and_linear_map::Matrix;
    use crate::models::vector::Vector;

    #[test]
    fn it_works() {
        let m = Matrix::<f32>::new(vec![
            vec![1., 2., 3.],
            vec![4., 5., 6.],
            vec![7., 8., 9.]
        ]);
        let v = Vector::<f32>::new(vec![1., 2., 3.]);
        assert_eq!(Vector::<f32>::new(vec![14., 32., 50.]), m.mul_vec(v));

        let m = Matrix::<f32>::new(vec![
            vec![1., 2., 3.],
            vec![4., 5., 6.],
            vec![7., 8., 9.]
        ]);
        let m2 = Matrix::<f32>::new(vec![
            vec![1., 2., 3.],
            vec![4., 5., 6.],
            vec![7., 8., 9.]
        ]);
        assert_eq!(Matrix::<f32>::new(vec![
            vec![30., 36., 42.],
            vec![66., 81., 96.],
            vec![102., 126., 150.]
        ]), m.mul_mat(m2));
    }

}