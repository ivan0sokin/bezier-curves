use crate::vector2::Vector2;

pub struct BezierCurve {
    points: Vec<Vector2<f32>>
}

impl BezierCurve {
    pub fn new() -> BezierCurve {
        BezierCurve {
            points: Vec::default()
        }
    }

    pub fn compute_for(&mut self, control_points: &[Vector2<f32>], steps: usize) {
        assert_ne!(steps, 0);
        assert_eq!(control_points.len(), 3);

        self.points.clear();
        self.points.reserve(steps + 1);

        let dt = 1.0 / steps as f32;

        for i in 0..=steps {
            self.points.push(self.get_quadratic_at(control_points, dt * i as f32));
        }
    }

    pub fn get_points(&self) -> &[Vector2<f32>] {
        &self.points
    }
}

impl BezierCurve {
    fn get_quadratic_at(&self, control_points: &[Vector2<f32>], t: f32) -> Vector2<f32> {
        control_points[0] * (1.0 - t) * (1.0 - t) -
        control_points[1] * (2.0 * t * t - 2.0 * t) +
        control_points[2] * t * t
    }
}
