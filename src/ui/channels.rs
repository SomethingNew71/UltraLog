//! Channel selection and display UI components.

use eframe::egui;

use crate::app::UltraLogApp;
use crate::state::MAX_CHANNELS;

impl UltraLogApp {
    /// Render channel selection panel - fills available space
    pub fn render_channel_selection(&mut self, ui: &mut egui::Ui) {
        ui.heading("Channels");
        ui.separator();

        if let Some(file_index) = self.selected_file {
            let file = &self.files[file_index];

            // Search box
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.channel_search)
                        .desired_width(f32::INFINITY),
                );
            });

            ui.add_space(5.0);

            // Channel count
            ui.label(format!(
                "Selected: {} / {} | Total: {}",
                self.selected_channels.len(),
                MAX_CHANNELS,
                file.log.channels.len()
            ));

            ui.separator();

            // Channel list - use all remaining vertical space
            let search_lower = self.channel_search.to_lowercase();
            let mut channel_to_add: Option<(usize, usize)> = None;
            let mut channel_to_remove: Option<usize> = None;

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());

                    for (channel_index, channel) in file.log.channels.iter().enumerate() {
                        let name = channel.name();

                        // Filter by search
                        if !search_lower.is_empty() && !name.to_lowercase().contains(&search_lower)
                        {
                            continue;
                        }

                        // Check if already selected and get its index in selected_channels
                        let selected_idx = self.selected_channels.iter().position(|c| {
                            c.file_index == file_index && c.channel_index == channel_index
                        });
                        let is_selected = selected_idx.is_some();

                        // Build the label with checkmark prefix if selected
                        let label_text = if is_selected {
                            format!("[*] {}", name)
                        } else {
                            format!("[ ] {}", name)
                        };

                        let response = ui.selectable_label(is_selected, label_text);

                        if response.clicked() {
                            if let Some(idx) = selected_idx {
                                // Already selected - remove it
                                channel_to_remove = Some(idx);
                            } else {
                                // Not selected - add it
                                channel_to_add = Some((file_index, channel_index));
                            }
                        }
                    }
                });

            // Handle deferred channel removal (must happen before addition to keep indices valid)
            if let Some(idx) = channel_to_remove {
                self.remove_channel(idx);
            }

            // Handle deferred channel addition
            if let Some((file_idx, channel_idx)) = channel_to_add {
                self.add_channel(file_idx, channel_idx);
            }
        } else {
            ui.centered_and_justified(|ui| {
                ui.label(
                    egui::RichText::new("Select a file to view channels")
                        .italics()
                        .color(egui::Color32::GRAY),
                );
            });
        }
    }

    /// Render selected channel cards
    pub fn render_selected_channels(&mut self, ui: &mut egui::Ui) {
        ui.heading("Selected Channels");
        ui.separator();

        let mut channel_to_remove: Option<usize> = None;

        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal(|ui| {
                for (i, selected) in self.selected_channels.iter().enumerate() {
                    let color = self.get_channel_color(selected.color_index);
                    let color32 = egui::Color32::from_rgb(color[0], color[1], color[2]);

                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(40, 40, 40))
                        .stroke(egui::Stroke::new(2.0, color32))
                        .rounding(5.0)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(selected.channel.name())
                                            .strong()
                                            .color(color32),
                                    );
                                    if ui.small_button("x").clicked() {
                                        channel_to_remove = Some(i);
                                    }
                                });

                                ui.label(
                                    egui::RichText::new(format!(
                                        "Type: {}",
                                        selected.channel.type_name()
                                    ))
                                    .small()
                                    .color(egui::Color32::GRAY),
                                );

                                if let (Some(min), Some(max)) = (
                                    selected.channel.display_min(),
                                    selected.channel.display_max(),
                                ) {
                                    let source_unit = selected.channel.unit();
                                    let (conv_min, display_unit) =
                                        self.unit_preferences.convert_value(min, source_unit);
                                    let (conv_max, _) =
                                        self.unit_preferences.convert_value(max, source_unit);
                                    let unit_str = if display_unit.is_empty() {
                                        String::new()
                                    } else {
                                        format!(" {}", display_unit)
                                    };
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Range: {:.0}{} - {:.0}{}",
                                            conv_min, unit_str, conv_max, unit_str
                                        ))
                                        .small()
                                        .color(egui::Color32::GRAY),
                                    );
                                }
                            });
                        });

                    ui.add_space(5.0);
                }
            });
        });

        if let Some(index) = channel_to_remove {
            self.remove_channel(index);
        }

        if self.selected_channels.is_empty() {
            ui.label(
                egui::RichText::new("Click channels to add them to the chart")
                    .italics()
                    .color(egui::Color32::GRAY),
            );
        }
    }
}
