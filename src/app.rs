use crate::core::vehicle::vehicle;
use crate::ui;
use eframe::egui;
use crate::core::connection::Connection;

pub struct App {
    pub error: Option<Box<dyn std::error::Error + Send + Sync>>,
    pub vehicle: Option<vehicle::Vehicle>,
    pub connection: Connection,
}
impl App {
    pub fn new() -> Self {
        Self {
            error: None,
            vehicle: None,
            connection: Connection {
                mavlink_conn: None,
                address: String::new(),
                loading: false,
            },
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui::error::show(&mut self.error, ui);
        
        egui::Panel::top("navbar")
            .resizable(false)
            .default_size(50.0)
            .show(ui, |ui| ui::navbar::show(self, ui));

        egui::Panel::bottom("console")
            .resizable(true)
            .default_size(150.0)
            .show(ui, |ui| {
                if let Some(vehicle) = &self.vehicle {
                    ui::console::show(&vehicle.logs, self.vehicle.is_some(), ui);
                } else {
                    let mut logs = Vec::new();
                    for _ in 0..100 {
                        logs.push(vehicle::Log::from(vehicle::LogType::Info, "test log"));
                    }
                    ui::console::show(&logs, self.vehicle.is_some(), ui);
                }
            });
    }
}
