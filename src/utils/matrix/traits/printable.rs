use std::fmt::{Debug, Display};
use crate::utils::matrix::matrix::Matrix;

pub trait Printable: Clone + Debug {}
impl <T: Clone + Debug> Printable for T {}

impl<T: Printable> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T: Printable> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}