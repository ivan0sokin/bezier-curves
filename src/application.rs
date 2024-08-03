use eframe::egui::{self};

use crate::vector2::Vector2;
use crate::matrix::Matrix;
use crate::bezier_curve::BezierCurve;
use crate::control_point::ControlPoint;

pub struct Application {
    control_points: [ControlPoint; 3],
    steps: usize,
    curve: BezierCurve,
    render_curve_points: bool
}

impl Application {
    pub fn new() -> Application {
        Application {
            control_points: [
                ControlPoint::new(Vector2::new(100.0, 100.0)),
                ControlPoint::new(Vector2::new(250.0, 400.0)),
                ControlPoint::new(Vector2::new(400.0, 100.0))
            ],
            steps: 1,
            curve: BezierCurve::new(),
            render_curve_points: false
        }
    }

    pub fn update_curve(&mut self) {
        let control_points = self.control_points.iter().map(|control_point| control_point.get_position()).collect::<Vec<Vector2<f32>>>();
        self.curve.compute_for(&control_points, self.steps);
    }
}

impl Application {
    const CURVE_POINT_COLOR: egui::Color32 = egui::Color32::YELLOW;
    const CURVE_POINT_RADIUS: f32 = 2.0;

    const DEFAULT_STROKE: egui::Stroke = egui::Stroke::NONE;
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("options")
            .exact_width(260.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Steps: ");
                    if ui.add(egui::DragValue::new(&mut self.steps).clamp_range(1..=100000)).lost_focus() {
                        self.update_curve();
                    }

                    if ui.radio(self.render_curve_points, "Render curve points").clicked() {
                        self.render_curve_points = !self.render_curve_points;
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_clip_rect(ui.clip_rect().split_left_right_at_x(ui.clip_rect().right() - 260.0).0);

            self.control_points.iter_mut().for_each(|control_point| control_point.update(ui));

            self.control_points.iter().for_each(|control_point| control_point.show(ui));

            self.control_points.windows(2).map(|wnd| [wnd[0].get_position(), wnd[1].get_position()]).for_each(|wnd| ui.painter().line_segment(
                [
                    wnd[0].into(),
                    wnd[1].into()
                ],
                egui::Stroke::new(1.0, egui::Color32::TEMPORARY_COLOR),
            ));

            if self.control_points.iter().any(|control_point| control_point.has_changed()) {
                self.update_curve();
            }

            if self.render_curve_points {
                self.curve.get_points().iter().for_each(|point|
                    ui.painter().circle(
                        (*point).into(),
                        Self::CURVE_POINT_RADIUS,
                        Self::CURVE_POINT_COLOR,
                        Self::DEFAULT_STROKE
                    )
                );
            }

            self.curve.get_points().windows(2).for_each(|wnd|
                ui.painter().line_segment(
                    [wnd[0].into(), wnd[1].into()],
                    egui::Stroke::new(1.0, egui::Color32::TEMPORARY_COLOR)
                )
            );
        });
    }
}
