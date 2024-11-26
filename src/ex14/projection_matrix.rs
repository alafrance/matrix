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
    use nalgebra::{Perspective3};

    #[test]
    fn test_projection() {
        let fov = 45.0;
        let fovy = 3.14 / 4.0;
        let ratio = 16.0 / 9.0;
        let near = 0.1;
        let far = 1000.0;

        let m = super::projection(fov, ratio, near, far);
        let perspective = Perspective3::new(ratio, fovy, near, far);
        let perspective = perspective.into_inner();
        let tolerance = 1e-2;

        for i in 0..4 {
            for j in 0..4 {
                assert!((m.at(i, j) - perspective[(i, j)]).abs() < tolerance);
            }
        }
    }
}