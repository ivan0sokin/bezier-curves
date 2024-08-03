mod application;
mod vector2;
mod bezier_curve;
mod control_point;
mod matrix;

use application::Application;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Bezier Curves",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(Application::new())),
    )
}
