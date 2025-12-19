//! Menu bar UI components (Units, Help menus).

use eframe::egui;

use crate::app::UltraLogApp;
use crate::units::{
    AccelerationUnit, DistanceUnit, FlowUnit, FuelEconomyUnit, PressureUnit, SpeedUnit,
    TemperatureUnit, VolumeUnit,
};

impl UltraLogApp {
    /// Render the application menu bar
    pub fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            // Units menu
            ui.menu_button("Units", |ui| {
                ui.set_min_width(180.0);

                // Temperature submenu
                ui.menu_button("🌡️  Temperature", |ui| {
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.temperature,
                            TemperatureUnit::Celsius,
                            "Celsius (°C)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.temperature,
                            TemperatureUnit::Fahrenheit,
                            "Fahrenheit (°F)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.temperature,
                            TemperatureUnit::Kelvin,
                            "Kelvin (K)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                });

                // Pressure submenu
                ui.menu_button("💨  Pressure", |ui| {
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.pressure,
                            PressureUnit::KPa,
                            "Kilopascal (kPa)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.pressure,
                            PressureUnit::PSI,
                            "PSI",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.pressure,
                            PressureUnit::Bar,
                            "Bar",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                });

                // Speed submenu
                ui.menu_button("🏎️  Speed", |ui| {
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.speed,
                            SpeedUnit::KmH,
                            "Kilometers/hour (km/h)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.speed,
                            SpeedUnit::Mph,
                            "Miles/hour (mph)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                });

                // Distance submenu
                ui.menu_button("📏  Distance", |ui| {
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.distance,
                            DistanceUnit::Kilometers,
                            "Kilometers (km)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.distance,
                            DistanceUnit::Miles,
                            "Miles (mi)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                });

                ui.separator();

                // Fuel Economy submenu
                ui.menu_button("⛽  Fuel Economy", |ui| {
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.fuel_economy,
                            FuelEconomyUnit::LPer100Km,
                            "Liters/100km (L/100km)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.fuel_economy,
                            FuelEconomyUnit::Mpg,
                            "Miles/gallon (mpg)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.fuel_economy,
                            FuelEconomyUnit::KmPerL,
                            "Kilometers/liter (km/L)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                });

                // Volume submenu
                ui.menu_button("🪣  Volume", |ui| {
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.volume,
                            VolumeUnit::Liters,
                            "Liters (L)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.volume,
                            VolumeUnit::Gallons,
                            "Gallons (gal)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                });

                // Flow submenu
                ui.menu_button("💧  Flow Rate", |ui| {
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.flow,
                            FlowUnit::CcPerMin,
                            "cc/min",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(&mut self.unit_preferences.flow, FlowUnit::LbPerHr, "lb/hr")
                        .clicked()
                    {
                        ui.close_menu();
                    }
                });

                ui.separator();

                // Acceleration submenu
                ui.menu_button("📈  Acceleration", |ui| {
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.acceleration,
                            AccelerationUnit::MPerS2,
                            "m/s²",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(
                            &mut self.unit_preferences.acceleration,
                            AccelerationUnit::G,
                            "g-force (g)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                });
            });

            ui.menu_button("Help", |ui| {
                ui.set_min_width(200.0);

                if ui.button("📖  Documentation").clicked() {
                    let _ = open::that("https://github.com/SomethingNew71/UltraLog#readme");
                    ui.close_menu();
                }

                if ui.button("🐛  Report Issue").clicked() {
                    let _ = open::that("https://github.com/SomethingNew71/UltraLog/issues");
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("📧  Email Support").clicked() {
                    let _ =
                        open::that("mailto:support@classicminidiy.com?subject=UltraLog%20Support");
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("💝  Support Development").clicked() {
                    let _ = open::that("https://github.com/sponsors/SomethingNew71");
                    ui.close_menu();
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION")))
                            .small()
                            .color(egui::Color32::GRAY),
                    );
                });
            });
        });
    }
}
