use crate::vector2::Vector2;
use crate::matrix::Matrix;

pub struct BezierCurve {
    points: Vec<Vector2<f32>>,
    coefficient_matrix: Matrix<f32>,
}

impl BezierCurve {
    pub fn new() -> BezierCurve {
        BezierCurve {
            points: Vec::default(),
            coefficient_matrix: Matrix::default()
        }
    }

    pub fn compute_for(&mut self, control_point_matrix: &Matrix<f32>, steps: usize) {
        if steps == 0 || control_point_matrix.rows != 2 {
            return;
        }

        self.points.clear();
        self.points.reserve(steps + 1);

        let dt = 1.0 / steps as f32;
        for i in 0..=steps {
            self.points.push(self.get_point_at(control_point_matrix, dt * i as f32));
        }
    }

    pub fn set_coefficient_matrix(&mut self, coefficient_matrix: Matrix<f32>) {
        self.coefficient_matrix = coefficient_matrix;
    }

    pub fn get_points(&self) -> &[Vector2<f32>] {
        &self.points
    }
}

impl BezierCurve {
    fn get_point_at(&self, control_point_matrix: &Matrix<f32>, t: f32) -> Vector2<f32> {
        let mut power = 1.0;
        let mut data = std::iter::repeat_with(|| {
            let tmp = power;
            power *= t;
            tmp
        }).take(control_point_matrix.columns).collect::<Vec<f32>>();

        data.reverse();

        let factors_matrix = Matrix { data, rows: control_point_matrix.columns, columns: 1 };

        let result = control_point_matrix.multiply_unchecked(&self.coefficient_matrix.multiply_unchecked(&factors_matrix));

        Vector2::new(result.data[0], result.data[1])
    }
}
