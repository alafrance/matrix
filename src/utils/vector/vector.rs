
pub struct Vector<T> {
    pub(crate) data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(array: Vec<T>) -> Vector<T> {
        Vector {
            data: array,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vector = Vector::new(vec![1, 2, 3]);
        assert_eq!(vector.data, vec![1, 2, 3]);
    }
}