use std::fmt::Debug;
use crate::utils::vector::vector::Vector;

impl<T: Clone + Debug + PartialEq> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}