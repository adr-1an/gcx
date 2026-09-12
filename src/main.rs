mod app;
mod core;
mod ui;

use crate::core::vehicle::vehicle;
use eframe::egui;

fn main() -> eframe::Result {
    let mut app = app::App::new();

    app.vehicle = Some(vehicle::Vehicle {
        logs: Vec::new(),
        state: vehicle::VehicleState { armed: false },
    });

    eframe::run_native(
        "GCX",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_maximized(true),
            ..Default::default()
        },
        Box::new(|_| Ok(Box::new(app))),
    )
}
