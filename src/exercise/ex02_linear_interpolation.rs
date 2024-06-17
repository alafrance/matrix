use std::ops::{Add, Mul};

fn lerp<V: Mul<f32, Output = V> + Add<Output = V> + Copy>(u: V, v: V, t: f32) -> V where f32: Mul<V, Output = V> + Add<Output = V> + Copy {
    if t < 0. || t > 1. {
        panic!("The interpolation factor must be between 0 and 1");
    }
    return (1. - t) * u + (t * v);
}

#[cfg(test)]
mod tests {
    use std::ops::Add;
    use crate::exercise::ex02_linear_interpolation::lerp;
    use crate::models::vector::Vector;

    #[test]
    fn it_works() {
        println!("{}", lerp(2., 4., 0.3));
        let vector = Vector::<f32>::new(vec![2., 1.]);
        let vector_2 = Vector::<f32>::new(vec![4., 2.]);
        // let result = vector + vector_2;
        // println!("{}", lerp(Vector::<f32>::new(vec![2., 1.]), Vector::<f32>::new(vec![4., 2.]), 0.3));
    }
}