use crate::utils::vector::vector::Vector;

impl<T: Clone> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Vector {
            data: self.data.clone(),
        }
    }
}
