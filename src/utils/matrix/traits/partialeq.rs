use std::fmt::Debug;
use crate::utils::matrix::matrix::Matrix;

pub trait Eqqable: PartialEq + Clone + Debug {}
impl <T: PartialEq + Clone + Debug> Eqqable for T {}
impl<T: Eqqable> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
