use crate::core::vehicle::vehicle;
use crate::ui;
use eframe::egui;
use std::fmt::Display;

#[derive(PartialEq)]
pub enum ConnectionType {
    Disconnected,
    UDP,
}

impl Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionType::Disconnected => write!(f, "Disconnected"),
            ConnectionType::UDP => write!(f, "UDP"),
        }
    }
}

pub struct Connection {
    pub connection_type: ConnectionType,
    pub address: String,
    pub loading: bool,
}

pub struct App {
    pub vehicle: Option<vehicle::Vehicle>,
    pub connection: Connection,
}

impl App {
    pub fn new() -> Self {
        Self {
            vehicle: None,
            connection: Connection {
                connection_type: ConnectionType::Disconnected,
                address: String::new(),
                loading: false,
            },
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
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
