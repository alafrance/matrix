use crate::utils::vector::vector::Vector;

impl<T: Clone> Vector<T> {
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn x(&self) -> T {
        if self.size() < 1 {
            panic!("Vector is empty");
        }
        self.data[0].clone()
    }

    pub fn y(&self) -> T {
        if self.size() < 2 {
            panic!("Vector has less than 2 elements");
        }
        self.data[1].clone()
    }

    pub fn z(&self) -> T {
        if self.size() < 3 {
            panic!("Vector has less than 3 elements");
        }
        self.data[2].clone()
    }

    pub fn get_index(&self, index: usize) -> T {
        if index >= self.size() {
            panic!("Index out of bounds");
        }
        self.data[index].clone()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::vector::vector::Vector;

    #[test]
    fn test_x() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        assert_eq!(vector.x(), 1);
    }

    #[test]
    #[should_panic]
    fn test_x_panic() {
        let vector: Vector<i32> = Vector::new(vec![]);
        vector.x();
    }

    #[test]
    fn test_y() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        assert_eq!(vector.y(), 2);
    }

    #[test]
    #[should_panic]
    fn test_y_panic() {
        let vector = Vector::from_array(vec![1]);
        vector.y();
    }

    #[test]
    fn test_z() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        assert_eq!(vector.z(), 3);
    }

    #[test]
    #[should_panic]
    fn test_z_panic() {
        let vector = Vector::from_array(vec![1, 2]);
        vector.z();
    }

    #[test]
    fn test_get_index() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        assert_eq!(vector.get_index(0), 1);
        assert_eq!(vector.get_index(1), 2);
        assert_eq!(vector.get_index(2), 3);
    }
    #[test]
    fn test_is_empty() {
        let vector: Vector<i32> = Vector::new(vec![]);
        assert_eq!(vector.is_empty(), true);
    }
}