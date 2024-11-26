use crate::utils::matrix::matrix::Matrix;
use nalgebra::Matrix4;
use nalgebra::Perspective3;

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
    #[test]
    fn test_projection() {
        let fov = 45.0;
        let ratio = 1.;
        let near = 0.1;
        let far = 10000.0;
        let m = super::projection(fov, ratio, near, far);
        let perspective = Perspective3::new(ratio, fov, near, far);
        let expected_matrix: Matrix4<f32> = perspective.into_inner();

        println!("{:?}", m);
        // Compare les matrices avec tolérance
        let tolerance = 1e-6;
        for i in 0..4 {
            for j in 0..4 {
                assert!(
                    (m.at(i,j) - expected_matrix[(i, j)]).abs() < tolerance,
                    "Mismatch at ({}, {}): expected {}, got {}",
                    i, j, expected_matrix[(i, j)], m.at(i, j)
                );
            }
        }
    }
}