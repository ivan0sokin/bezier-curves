use crate::coefficient_matrix_computer::CoefficientMatrixComputer;
use crate::matrix::Matrix;

pub struct JITCoefficientMatrixComputer;

impl JITCoefficientMatrixComputer {
    pub fn new() -> JITCoefficientMatrixComputer {
        JITCoefficientMatrixComputer
    }

    fn next_binomial(coefficient: usize, n: usize, k: usize) -> usize {
        (coefficient * (n - k)) / (k + 1)
    }
}

impl CoefficientMatrixComputer for JITCoefficientMatrixComputer {
    fn compute_for(&self, n: usize) -> Matrix<f32> {
        let mut result = Matrix::new(n + 1, n + 1);

        let mut factor = 1;
        for k in 0..=n {
            let mut binomial_coefficient = 1;
            result.row_at_mut(k).iter_mut().rev().skip(k).enumerate().for_each(|(i, value)| {
                let sign = if i & 1 == 0 { 1 } else { -1 };
                *value = (sign as f32) * (factor as f32) * (binomial_coefficient as f32);
                binomial_coefficient = Self::next_binomial(binomial_coefficient, n - k, i);
            });

            factor = Self::next_binomial(factor, n, k);
        }

        result
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
