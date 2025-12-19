//! Toast notification system for user feedback.

use eframe::egui;

use crate::app::UltraLogApp;

impl UltraLogApp {
    /// Render toast notifications
    pub fn render_toast(&mut self, ctx: &egui::Context) {
        if let Some((message, time)) = &self.toast_message {
            if time.elapsed().as_secs() < 3 {
                egui::Area::new(egui::Id::new("toast"))
                    .fixed_pos(egui::pos2(10.0, 10.0))
                    .show(ctx, |ui| {
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgb(191, 78, 48))
                            .rounding(5.0)
                            .inner_margin(10.0)
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new(message).color(egui::Color32::WHITE));
                            });
                    });
            } else {
                self.toast_message = None;
            }
        }
    }
}
