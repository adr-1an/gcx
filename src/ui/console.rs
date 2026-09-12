use eframe::egui;

use crate::core::vehicle::vehicle;

pub fn show(logs: &Vec<vehicle::Log>, vehicle_connected: bool, ui: &mut egui::Ui) {
    egui::Frame::new().show(ui, |ui| {
        // Title bar
        ui.set_width(ui.available_width());

        ui.label(
            egui::RichText::new("Console")
                .strong()
                .size(14.0)
                .monospace(),
        );

        ui.separator();

        // Scroll area
        egui::Frame::new().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_width(ui.available_width());

                if vehicle_connected {
                    if logs.len() > 0 {
                        for log in &mut logs.iter() {
                            match log.log_type {
                                vehicle::LogType::Info => ui.label(
                                    egui::RichText::new(format!("[INFO] {}", log.message))
                                        .monospace()
                                        .color(egui::Color32::LIGHT_BLUE)
                                        .size(13.0),
                                ),
                                vehicle::LogType::Warning => ui.label(
                                    egui::RichText::new(format!("[WARN] {}", log.message))
                                        .monospace()
                                        .color(egui::Color32::YELLOW)
                                        .size(13.0),
                                ),
                                vehicle::LogType::Error => ui.label(
                                    egui::RichText::new(format!("[ERR] {}", log.message))
                                        .monospace()
                                        .color(egui::Color32::ORANGE)
                                        .size(13.0),
                                ),
                                vehicle::LogType::Critical => ui.label(
                                    egui::RichText::new(format!("[CRITICAL] {}", log.message))
                                        .monospace()
                                        .color(egui::Color32::RED)
                                        .size(13.0),
                                ),
                            };
                        }
                    } else {
                        ui.label(
                            egui::RichText::new("No logs yet.")
                                .monospace()
                                .italics()
                                .color(egui::Color32::from_white_alpha(120)),
                        );
                    }
                } else {
                    ui.label(
                        egui::RichText::new("No vehicle connected.")
                            .monospace()
                            .color(egui::Color32::ORANGE)
                            .size(13.0),
                    );
                }
            });
        })
    });
}
