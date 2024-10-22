pub struct Matrix<T> {
    pub(crate) data: Vec<T>,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> Matrix<T> {
        if data.len() % rows != 0 || data.len() % cols != 0 {
            panic!("Data length must be a multiple of rows or cols");
        }
        if rows * cols != data.len() {
            panic!("Data length must be equal to rows * cols");
        }
        Matrix { data, rows, cols }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(matrix.data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    #[should_panic]
    fn test_new_error() {
        Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 4);
    }

    #[test]
    #[should_panic]
    fn test_new_empty_error() {
        Matrix::<i32>::new(vec![], 2, 3);
    }
}