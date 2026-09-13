use eframe::egui;

pub fn show(
    error: &mut Option<Box<dyn std::error::Error + Send + Sync>>,
    ui: &mut egui::Ui,
) {
    let Some(err) = error.as_ref() else {
        return;
    };

    let mut close = false;

    egui::Window::new("Error")
        .collapsible(false)
        .resizable(false)
        .show(ui.ctx(), |ui| {
            ui.label(err.to_string());

            if ui.button("OK").clicked() {
                close = true;
            }
        });

    if close {
        *error = None;
    }
}