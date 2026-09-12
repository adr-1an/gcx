use crate::app::{App, ConnectionType};
use eframe::egui;
pub fn show(app: &mut App, ui: &mut egui::Ui) {
    egui::Frame::new().show(ui, |ui| {
        ui.set_width(ui.available_width());
        if let Some(vehicle) = &app.vehicle {
            ui.horizontal(|ui| {
                /* Arming status */
                egui::Frame::new()
                    .inner_margin(egui::Margin::symmetric(20, 10))
                    .show(ui, |ui| {
                        match vehicle.state.armed {
                            true => ui.label(
                                egui::RichText::new("Armed")
                                    .strong()
                                    .color(egui::Color32::GREEN)
                                    .size(20.0)
                                    .monospace(),
                            ),
                            false => ui.label(
                                egui::RichText::new("Disarmed")
                                    .strong()
                                    .color(egui::Color32::GRAY)
                                    .size(20.0)
                                    .monospace(),
                            ),
                        };
                    });
                egui::Frame::new()
                    .inner_margin(egui::Margin::symmetric(10, 5))
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), {
                            |ui| {
                                let conn_button_label: &str;
                                match app.connection.loading {
                                    true => conn_button_label = "Connecting...",
                                    false => conn_button_label = "Connect",
                                };
                                if ui
                                    .add_enabled(
                                        !app.connection.address.is_empty(),
                                        egui::Button::new(conn_button_label),
                                    )
                                    .clicked()
                                {
                                    app.connection.loading = !app.connection.loading;
                                }
                                egui::ComboBox::from_id_salt("type")
                                    .selected_text(&app.connection.connection_type.to_string())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut app.connection.connection_type,
                                            ConnectionType::Disconnected,
                                            "Disconnected",
                                        );
                                        ui.selectable_value(
                                            &mut app.connection.connection_type,
                                            ConnectionType::UDP,
                                            "UDP",
                                        );
                                    });
                                ui.text_edit_singleline(&mut app.connection.address);
                            }
                        });
                    })
            });
        } else {
            ui.label(
                egui::RichText::new("No vehicle connected.")
                    .strong()
                    .monospace()
                    .color(egui::Color32::ORANGE)
                    .size(20.0),
            );
        }
    });
}
