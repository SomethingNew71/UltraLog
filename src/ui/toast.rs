//! Toast notification system for user feedback.

use eframe::egui;

use crate::app::UltraLogApp;

impl UltraLogApp {
    /// Render toast notifications in the bottom right corner
    pub fn render_toast(&mut self, ctx: &egui::Context) {
        if let Some((message, time, toast_type)) = &self.toast_message {
            if time.elapsed().as_secs() < 3 {
                let margin = 20.0;

                // Get colors for this toast type
                let bg_color = toast_type.color();
                let text_color = toast_type.text_color();

                egui::Area::new(egui::Id::new("toast"))
                    .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-margin, -margin))
                    .order(egui::Order::Foreground)
                    .show(ctx, |ui| {
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgb(
                                bg_color[0],
                                bg_color[1],
                                bg_color[2],
                            ))
                            .rounding(8.0)
                            .inner_margin(egui::Margin::symmetric(16.0, 12.0))
                            .shadow(egui::epaint::Shadow {
                                offset: egui::vec2(2.0, 2.0),
                                blur: 8.0,
                                spread: 0.0,
                                color: egui::Color32::from_black_alpha(60),
                            })
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(message)
                                        .color(egui::Color32::from_rgb(
                                            text_color[0],
                                            text_color[1],
                                            text_color[2],
                                        ))
                                        .size(14.0),
                                );
                            });
                    });
            } else {
                self.toast_message = None;
            }
        }
    }
}
