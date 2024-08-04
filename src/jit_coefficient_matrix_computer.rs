use crate::coefficient_matrix_computer::CoefficientMatrixComputer;
use crate::matrix::Matrix;

pub struct JITCoefficientMatrixComputer {

}

impl JITCoefficientMatrixComputer {
    pub fn new() -> JITCoefficientMatrixComputer {
        JITCoefficientMatrixComputer {

        }
    }
}

impl CoefficientMatrixComputer for JITCoefficientMatrixComputer {
    fn compute_for(&self, n: usize) -> Matrix<f32> {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
