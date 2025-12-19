//! Unit preference types and conversion utilities.
//!
//! This module provides user-configurable unit preferences for displaying
//! ECU log data in various measurement systems (metric, imperial, etc.).

/// Temperature unit preference
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TemperatureUnit {
    Kelvin,
    #[default]
    Celsius,
    Fahrenheit,
}

impl TemperatureUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            TemperatureUnit::Kelvin => "K",
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
        }
    }

    /// Convert from Kelvin to the selected unit
    pub fn convert_from_kelvin(&self, kelvin: f64) -> f64 {
        match self {
            TemperatureUnit::Kelvin => kelvin,
            TemperatureUnit::Celsius => kelvin - 273.15,
            TemperatureUnit::Fahrenheit => (kelvin - 273.15) * 9.0 / 5.0 + 32.0,
        }
    }
}

/// Pressure unit preference
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PressureUnit {
    #[default]
    KPa,
    PSI,
    Bar,
}

impl PressureUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            PressureUnit::KPa => "kPa",
            PressureUnit::PSI => "PSI",
            PressureUnit::Bar => "bar",
        }
    }

    /// Convert from kPa to the selected unit
    pub fn convert_from_kpa(&self, kpa: f64) -> f64 {
        match self {
            PressureUnit::KPa => kpa,
            PressureUnit::PSI => kpa * 0.145038,
            PressureUnit::Bar => kpa * 0.01,
        }
    }
}

/// Speed unit preference
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SpeedUnit {
    #[default]
    KmH,
    Mph,
}

impl SpeedUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            SpeedUnit::KmH => "km/h",
            SpeedUnit::Mph => "mph",
        }
    }

    /// Convert from km/h to the selected unit
    pub fn convert_from_kmh(&self, kmh: f64) -> f64 {
        match self {
            SpeedUnit::KmH => kmh,
            SpeedUnit::Mph => kmh * 0.621371,
        }
    }
}

/// Distance unit preference
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DistanceUnit {
    #[default]
    Kilometers,
    Miles,
}

impl DistanceUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            DistanceUnit::Kilometers => "km",
            DistanceUnit::Miles => "mi",
        }
    }

    /// Convert from km to the selected unit
    pub fn convert_from_km(&self, km: f64) -> f64 {
        match self {
            DistanceUnit::Kilometers => km,
            DistanceUnit::Miles => km * 0.621371,
        }
    }
}

/// Fuel economy unit preference
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum FuelEconomyUnit {
    #[default]
    LPer100Km,
    Mpg,
    KmPerL,
}

impl FuelEconomyUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            FuelEconomyUnit::LPer100Km => "L/100km",
            FuelEconomyUnit::Mpg => "mpg",
            FuelEconomyUnit::KmPerL => "km/L",
        }
    }

    /// Convert from L/100km to the selected unit
    pub fn convert_from_l_per_100km(&self, l_per_100km: f64) -> f64 {
        match self {
            FuelEconomyUnit::LPer100Km => l_per_100km,
            FuelEconomyUnit::Mpg => {
                if l_per_100km > 0.0 {
                    235.215 / l_per_100km
                } else {
                    0.0
                }
            }
            FuelEconomyUnit::KmPerL => {
                if l_per_100km > 0.0 {
                    100.0 / l_per_100km
                } else {
                    0.0
                }
            }
        }
    }
}

/// Volume unit preference
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum VolumeUnit {
    #[default]
    Liters,
    Gallons,
}

impl VolumeUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            VolumeUnit::Liters => "L",
            VolumeUnit::Gallons => "gal",
        }
    }

    /// Convert from liters to the selected unit
    pub fn convert_from_liters(&self, liters: f64) -> f64 {
        match self {
            VolumeUnit::Liters => liters,
            VolumeUnit::Gallons => liters * 0.264172,
        }
    }
}

/// Flow rate unit preference
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum FlowUnit {
    #[default]
    CcPerMin,
    LbPerHr,
}

impl FlowUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            FlowUnit::CcPerMin => "cc/min",
            FlowUnit::LbPerHr => "lb/hr",
        }
    }

    /// Convert from cc/min to the selected unit (assuming gasoline density ~0.75 g/cc)
    pub fn convert_from_cc_per_min(&self, cc_per_min: f64) -> f64 {
        match self {
            FlowUnit::CcPerMin => cc_per_min,
            // cc/min * 0.75 g/cc * 60 min/hr / 453.592 g/lb = lb/hr
            FlowUnit::LbPerHr => cc_per_min * 0.75 * 60.0 / 453.592,
        }
    }
}

/// Acceleration unit preference
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AccelerationUnit {
    #[default]
    MPerS2,
    G,
}

impl AccelerationUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            AccelerationUnit::MPerS2 => "m/s²",
            AccelerationUnit::G => "g",
        }
    }

    /// Convert from m/s² to the selected unit
    pub fn convert_from_m_per_s2(&self, m_per_s2: f64) -> f64 {
        match self {
            AccelerationUnit::MPerS2 => m_per_s2,
            AccelerationUnit::G => m_per_s2 / 9.80665,
        }
    }
}

/// User preferences for display units
#[derive(Clone, Debug, Default)]
pub struct UnitPreferences {
    pub temperature: TemperatureUnit,
    pub pressure: PressureUnit,
    pub speed: SpeedUnit,
    pub distance: DistanceUnit,
    pub fuel_economy: FuelEconomyUnit,
    pub volume: VolumeUnit,
    pub flow: FlowUnit,
    pub acceleration: AccelerationUnit,
}

impl UnitPreferences {
    /// Convert a value and get the display unit based on the source unit string
    /// Returns (converted_value, display_unit)
    pub fn convert_value<'a>(&self, value: f64, source_unit: &'a str) -> (f64, &'a str) {
        match source_unit {
            // Temperature (source is Kelvin)
            "K" => (
                self.temperature.convert_from_kelvin(value),
                self.temperature.symbol(),
            ),
            // Pressure (source is kPa)
            "kPa" => (
                self.pressure.convert_from_kpa(value),
                self.pressure.symbol(),
            ),
            // Speed (source is km/h)
            "km/h" => (self.speed.convert_from_kmh(value), self.speed.symbol()),
            // Distance (source is km)
            "km" => (self.distance.convert_from_km(value), self.distance.symbol()),
            // Fuel economy (source is L/100km)
            "L/100km" => (
                self.fuel_economy.convert_from_l_per_100km(value),
                self.fuel_economy.symbol(),
            ),
            // Volume (source is L)
            "L" => (self.volume.convert_from_liters(value), self.volume.symbol()),
            // Flow (source is cc/min)
            "cc/min" => (self.flow.convert_from_cc_per_min(value), self.flow.symbol()),
            // Acceleration (source is m/s²)
            "m/s²" => (
                self.acceleration.convert_from_m_per_s2(value),
                self.acceleration.symbol(),
            ),
            // No conversion needed for other units
            _ => (value, source_unit),
        }
    }
}
