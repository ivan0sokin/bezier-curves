use eframe::egui::{self};

use crate::cache_coefficient_matrix_computer::CacheCoefficientMatrixComputer;
use crate::jit_coefficient_matrix_computer::JITCoefficientMatrixComputer;
use crate::vector2::Vector2;
use crate::matrix::Matrix;
use crate::control_point::ControlPoint;
use crate::bezier_curve::BezierCurve;
use crate::coefficient_matrix_computer::CoefficientMatrixComputer;
use crate::options::Options;

pub struct Application {
    control_points: Vec<ControlPoint>,
    curve: BezierCurve,
    coefficient_matrix_computers: Vec<Box<dyn CoefficientMatrixComputer>>,
    options: Options
}

impl Application {
    pub fn new() -> Application {
        let mut app = Application {
            control_points: vec![
                ControlPoint::new(Vector2::new(100.0, 100.0)),
                ControlPoint::new(Vector2::new(250.0, 400.0)),
                ControlPoint::new(Vector2::new(400.0, 100.0))
            ],
            curve: BezierCurve::new(),
            coefficient_matrix_computers: vec![Box::new(CacheCoefficientMatrixComputer::new()), Box::new(JITCoefficientMatrixComputer::new())],
            options: Options::new()
        };

        app.update_coefficient_matrix();
        app.update_curve();

        app
    }

    fn update_coefficient_matrix(&mut self) {
        self.coefficient_matrix_computers[0].as_any_mut().downcast_mut::<CacheCoefficientMatrixComputer>().unwrap().precompute_cache_for(self.control_points.len() - 1);
        self.curve.set_coefficient_matrix(self.coefficient_matrix_computers[0].compute_for(self.control_points.len() - 1));
    }

    fn update_curve(&mut self) {
        let control_points_matrix = Matrix { data: self.control_points.iter().map(|p| p.get_position().x).chain(self.control_points.iter().map(|p| p.get_position().y)).collect(), rows: 2, columns: self.control_points.len() };
        self.curve.compute_for(&control_points_matrix, self.options.steps);
    }
}

impl Application {
    const CURVE_POINT_COLOR: egui::Color32 = egui::Color32::YELLOW;
    const CURVE_POINT_RADIUS: f32 = 2.0;

    const CURVE_COLOR: egui::Color32 = egui::Color32::BLUE;
    const CURVE_STROKE: egui::Stroke = egui::Stroke { width: 2.0, color: Self::CURVE_COLOR };

    const DEFAULT_STROKE: egui::Stroke = egui::Stroke::NONE;
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("options").exact_width(260.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Steps: ");
                if ui.add(egui::DragValue::new(&mut self.options.steps).clamp_range(1..=100000)).lost_focus() {
                    self.update_curve();
                }
            });

            ui.checkbox(&mut self.options.render_curve_points, "Render curve points");

            ui.checkbox(&mut self.options.render_control_points, "Render control points");

            ui.checkbox(&mut self.options.render_lines, "Render lines");

            if ui.button("Add control point").clicked() {
                let center = ui.clip_rect().split_left_right_at_x(ui.clip_rect().right() - 260.0).0.center();
                self.control_points.push(ControlPoint::new(center.into()));

                self.update_coefficient_matrix();
                self.update_curve();
            }

            if ui.button("Remove last control point").clicked() {
                if !self.control_points.is_empty() {
                    self.control_points.pop();
                }

                self.update_coefficient_matrix();
                self.update_curve();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_clip_rect(ui.clip_rect().split_left_right_at_x(ui.clip_rect().right() - 260.0).0);

            if self.options.render_control_points {
                self.control_points.iter_mut().for_each(|control_point| control_point.update(ui));

                if self.control_points.iter().any(|control_point| control_point.has_changed()) {
                    self.update_curve();
                }
            }

            if self.options.render_lines {
                self.control_points.windows(2).map(|wnd| [wnd[0].get_position(), wnd[1].get_position()]).for_each(|wnd| ui.painter().line_segment(
                    [
                        wnd[0].into(),
                        wnd[1].into()
                    ],
                    egui::Stroke::new(1.0, egui::Color32::TEMPORARY_COLOR),
                ));
            }

            self.curve.get_points().windows(2).for_each(|wnd|
                ui.painter().line_segment(
                    [wnd[0].into(), wnd[1].into()],
                    Self::CURVE_STROKE
                )
            );

            if self.options.render_control_points {
                self.control_points.iter().for_each(|control_point| control_point.show(ui));
            }

            if self.options.render_curve_points {
                self.curve.get_points().iter().for_each(|point|
                    ui.painter().circle(
                        (*point).into(),
                        Self::CURVE_POINT_RADIUS,
                        Self::CURVE_POINT_COLOR,
                        Self::DEFAULT_STROKE
                    )
                );
            }
        });
    }
}
