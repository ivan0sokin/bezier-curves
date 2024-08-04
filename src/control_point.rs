use crate::vector2::Vector2;

pub struct ControlPoint {
    position: Vector2<f32>,
    position_before_drag: Vector2<f32>,
    is_dragged: bool,
    delta_position: Vector2<f32>
}

impl ControlPoint {
    pub fn new(position: Vector2<f32>) -> ControlPoint {
        ControlPoint {
            position,
            position_before_drag: position,
            is_dragged: false,
            delta_position: Vector2::new(0.0, 0.0)
        }
    }

    pub fn get_position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn has_changed(&self) -> bool {
        self.delta_position.x.abs() > std::f32::EPSILON ||
        self.delta_position.y.abs() > std::f32::EPSILON
    }

    pub fn show(&self, ui: &mut eframe::egui::Ui) {
        ui.painter().circle(self.position.into(), Self::RADIUS, Self::COLOR, if self.is_dragged { Self::OUTLINE_STROKE } else { Self::DEFAULT_STROKE });
    }

    pub fn update(&mut self, ui: &mut eframe::egui::Ui) {
        let last_position = self.position;

        if self.is_dragged {
            if let Some(position) = ui.ctx().pointer_latest_pos() {
                self.position = Vector2::from(position);
            }

            if !ui.ui_contains_pointer() {
                self.is_dragged = false;
                self.position = self.position_before_drag;
            }
        }

        self.delta_position = self.position - last_position;

        if ui.ui_contains_pointer() {
            if ui.input(|i| i.pointer.primary_pressed()) && self.is_pointer_in_bounds(ui) {
                self.on_pointer_press(ui);
            } else if ui.input(|i| i.pointer.primary_released()) && self.is_pointer_in_bounds(ui) {
                self.on_pointer_release(ui)
            }
        }
    }

    fn is_pointer_in_bounds(&self, ui: &mut eframe::egui::Ui) -> bool {
        if let Some(position) = ui.ctx().pointer_latest_pos() {
            let position = Vector2::from(position);

            let epsilon = 2.0;
            if (position - self.position).magnitude_squared() <= (Self::RADIUS + epsilon) * (Self::RADIUS + epsilon) {
                return true;
            }
        }

        return false;
    }

    fn on_pointer_press(&mut self, _ui: &mut eframe::egui::Ui) {
        self.is_dragged = true;
        self.position_before_drag = self.position;
    }

    fn on_pointer_release(&mut self, _ui: &mut eframe::egui::Ui) {
        self.is_dragged = false;
    }
}

impl ControlPoint {
    const RADIUS: f32 = 5.0;
    const COLOR: eframe::egui::Color32 = eframe::egui::Color32::RED;
    const DEFAULT_STROKE: eframe::egui::Stroke = eframe::egui::Stroke::NONE;
    const OUTLINE_STROKE: eframe::egui::Stroke = eframe::egui::Stroke { width: 2.0, color: eframe::egui::Color32::WHITE };
}
