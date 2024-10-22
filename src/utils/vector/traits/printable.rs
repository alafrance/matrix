use std::fmt::{Debug, Display, Formatter};
use crate::utils::vector::vector::Vector;

// Debug and Display
impl<T: Clone + Debug> Debug for Vector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T: Clone + Debug> Display for Vector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}