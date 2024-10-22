use crate::utils::matrix::matrix::Matrix;

fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix::<f32> {
    let fov_radians = fov.to_radians();

    let f = 1.0 / (fov_radians / 2.0).tan();

    let mut projection_matrix = vec![vec![0.0; 4]; 4];

    projection_matrix[0][0] = f / ratio;
    projection_matrix[1][1] = f;
    projection_matrix[2][2] = (far + near) / (near - far);
    projection_matrix[2][3] = (2.0 * far * near) / (near - far);
    projection_matrix[3][2] = -1.0;
    projection_matrix[3][3] = 0.0;

    Matrix::from_arrays(projection_matrix)
}

#[cfg(test)]
mod tests {
    use num_complex::Complex;

    #[test]
    fn test_projection() {
        Complex::new(1.0, 2.0);
        let fov = 90.0;
        let ratio = 16.0 / 9.0;
        let near = 0.1;
        let far = 100.0;
        let m = super::projection(fov, ratio, near, far);
        println!("{:?}", m);
    }
}