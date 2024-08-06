use crate::matrix::Matrix;
use std::any::Any;

pub trait CoefficientMatrixComputer {
    fn compute_for(&self, n: usize) -> Matrix<f32>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, PartialEq)]
pub enum CoefficientMatrixComputerType {
    Cached,
    JIT
}

impl std::fmt::Display for CoefficientMatrixComputerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(match self {
            CoefficientMatrixComputerType::Cached => "Cached",
            CoefficientMatrixComputerType::JIT => "JIT"
        })
    }
}
