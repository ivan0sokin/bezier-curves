use eframe::egui::{self};

use crate::vector2::Vector2;
use crate::bezier_curve::BezierCurve;

pub struct Application {
    control_points: [Vector2<f32>; 3],
    steps: usize,
    curve: BezierCurve,
    render_curve_points: bool
}

impl Application {
    pub fn new() -> Application {
        Application {
            control_points: [Vector2::new(100.0, 100.0), Vector2::new(250.0, 400.0), Vector2::new(400.0, 100.0)],
            steps: 1,
            curve: BezierCurve::new(),
            render_curve_points: false
        }
    }
}

impl Application {
    const CONTROL_POINT_COLOR: egui::Color32 = egui::Color32::RED;
    const CURVE_POINT_COLOR: egui::Color32 = egui::Color32::YELLOW;

    const DEFAULT_STROKE: egui::Stroke = egui::Stroke::NONE;

    const CONTROL_POINT_RADIUS: f32 = 5.0;
    const CURVE_POINT_RADIUS: f32 = 4.0;
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("options")
            .exact_width(260.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Steps: ");
                    if ui.add(egui::DragValue::new(&mut self.steps).clamp_range(1..=100000)).lost_focus() {
                        self.curve.compute_for(&self.control_points, self.steps);
                    }

                    if ui.radio(self.render_curve_points, "Render curve points").clicked() {
                        self.render_curve_points = !self.render_curve_points;
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            for i in 0..3 {
                ui.painter().circle(
                    self.control_points[i].into(),
                    Self::CONTROL_POINT_RADIUS,
                    Self::CONTROL_POINT_COLOR,
                    Self::DEFAULT_STROKE
                );
            }

            for i in 1..3 {
                ui.painter().line_segment(
                    [
                        self.control_points[i - 1].into(),
                        self.control_points[i].into(),
                    ],
                    egui::Stroke::new(1.0, egui::Color32::TEMPORARY_COLOR),
                );
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
