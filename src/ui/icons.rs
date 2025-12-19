//! Custom icon drawing utilities.

use eframe::egui;

/// Draw an upload icon (circle with upward arrow)
pub fn draw_upload_icon(ui: &mut egui::Ui, center: egui::Pos2, size: f32, color: egui::Color32) {
    let painter = ui.painter();
    let radius = size / 2.0;

    // Draw circle outline
    painter.circle_stroke(center, radius, egui::Stroke::new(2.0, color));

    // Draw arrow pointing up
    let arrow_size = size * 0.35;
    let arrow_top = egui::pos2(center.x, center.y - arrow_size * 0.6);
    let arrow_bottom = egui::pos2(center.x, center.y + arrow_size * 0.4);

    // Arrow shaft
    painter.line_segment([arrow_bottom, arrow_top], egui::Stroke::new(2.0, color));

    // Arrow head
    let head_size = arrow_size * 0.4;
    painter.line_segment(
        [
            arrow_top,
            egui::pos2(arrow_top.x - head_size, arrow_top.y + head_size),
        ],
        egui::Stroke::new(2.0, color),
    );
    painter.line_segment(
        [
            arrow_top,
            egui::pos2(arrow_top.x + head_size, arrow_top.y + head_size),
        ],
        egui::Stroke::new(2.0, color),
    );
}
