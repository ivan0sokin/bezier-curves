use crate::matrix::Matrix;
use std::any::Any;

pub trait CoefficientMatrixComputer {
    fn compute_for(&self, n: usize) -> Matrix<f32>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
