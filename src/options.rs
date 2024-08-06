use crate::coefficient_matrix_computer::CoefficientMatrixComputerType;

#[derive(Debug)]
pub struct Options {
    pub steps: usize,
    pub coefficient_matrix_computer_type: CoefficientMatrixComputerType,
    pub render_curve_points: bool,
    pub render_control_points: bool,
    pub render_lines: bool
}

impl Options {
    pub fn new() -> Options {
        Options {
            steps: 1,
            coefficient_matrix_computer_type: CoefficientMatrixComputerType::Cached,
            render_curve_points: false,
            render_control_points: true,
            render_lines: true
        }
    }
}
