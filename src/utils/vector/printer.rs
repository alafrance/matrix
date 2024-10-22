use std::fmt::Debug;
use crate::utils::vector::vector::Vector;

impl<T: Debug> Vector<T> {
    pub fn print(&self) {
        println!("[{:?}]", self.data);
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::vector::vector::Vector;

    #[test]
    fn test_print() {
        let vector = Vector::from_array(vec![1, 2, 3]);
        vector.print();
    }
}