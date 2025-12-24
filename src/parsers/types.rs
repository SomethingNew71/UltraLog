use serde::Serialize;
use std::error::Error;

use super::ecumaster::{EcuMasterChannel, EcuMasterMeta};
use super::haltech::{HaltechChannel, HaltechMeta};
use super::speeduino::{SpeeduinoChannel, SpeeduinoMeta};

/// Metadata enum supporting different ECU formats
#[derive(Clone, Debug, Serialize, Default)]
pub enum Meta {
    Haltech(HaltechMeta),
    EcuMaster(EcuMasterMeta),
    Speeduino(SpeeduinoMeta),
    #[default]
    Empty,
}

/// Channel enum supporting different ECU formats
#[derive(Clone, Debug)]
pub enum Channel {
    Haltech(HaltechChannel),
    EcuMaster(EcuMasterChannel),
    Speeduino(SpeeduinoChannel),
}

impl Serialize for Channel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Channel::Haltech(h) => h.serialize(serializer),
            Channel::EcuMaster(e) => e.serialize(serializer),
            Channel::Speeduino(s) => s.serialize(serializer),
        }
    }
}

impl Channel {
    pub fn name(&self) -> String {
        match self {
            Channel::Haltech(h) => h.name.clone(),
            Channel::EcuMaster(e) => e.name.clone(),
            Channel::Speeduino(s) => s.name.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> String {
        match self {
            Channel::Haltech(h) => h.id.clone(),
            Channel::EcuMaster(e) => e.path.clone(),
            Channel::Speeduino(s) => s.name.clone(),
        }
    }

    pub fn type_name(&self) -> String {
        match self {
            Channel::Haltech(h) => h.r#type.as_ref().to_string(),
            Channel::EcuMaster(e) => e.path.clone(),
            Channel::Speeduino(_) => "Speeduino/rusEFI".to_string(),
        }
    }

    pub fn display_min(&self) -> Option<f64> {
        match self {
            Channel::Haltech(h) => h.display_min,
            Channel::EcuMaster(_) => None,
            Channel::Speeduino(_) => None,
        }
    }

    pub fn display_max(&self) -> Option<f64> {
        match self {
            Channel::Haltech(h) => h.display_max,
            Channel::EcuMaster(_) => None,
            Channel::Speeduino(_) => None,
        }
    }

    pub fn unit(&self) -> &str {
        match self {
            Channel::Haltech(h) => h.unit(),
            Channel::EcuMaster(e) => e.unit(),
            Channel::Speeduino(s) => s.unit(),
        }
    }
}

/// Optimized value storage - all ECU log data is stored as f64
/// This uses 8 bytes per value instead of 16 bytes with the previous enum
#[derive(Clone, Copy, Debug, Default)]
pub struct Value(f64);

impl Value {
    /// Create a new Value from an f64
    /// Note: Named 'Float' to maintain API compatibility with previous enum variant
    #[inline]
    #[allow(non_snake_case)]
    pub fn Float(value: f64) -> Self {
        Self(value)
    }

    /// Convert value to f64 for charting
    #[inline]
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f64(self.0)
    }
}

/// Parsed log file structure
#[derive(Clone, Debug, Default)]
pub struct Log {
    #[allow(dead_code)]
    pub meta: Meta,
    pub channels: Vec<Channel>,
    /// Time values stored directly as f64 (seconds) for efficiency
    pub times: Vec<f64>,
    pub data: Vec<Vec<Value>>,
}

impl Log {
    /// Get data for a specific channel by index
    pub fn get_channel_data(&self, channel_index: usize) -> Vec<f64> {
        self.data
            .iter()
            .filter_map(|row| row.get(channel_index).map(|v| v.as_f64()))
            .collect()
    }

    /// Get time values as f64 slice (seconds) - no parsing needed, stored directly
    pub fn get_times_as_f64(&self) -> &[f64] {
        &self.times
    }

    /// Find channel index by name
    #[allow(dead_code)]
    pub fn find_channel_index(&self, name: &str) -> Option<usize> {
        self.channels.iter().position(|c| c.name() == name)
    }
}

/// Trait for log file parsers
pub trait Parseable {
    fn parse(&self, data: &str) -> Result<Log, Box<dyn Error>>;
}

/// Supported ECU types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum EcuType {
    #[default]
    Haltech,
    EcuMaster,
    MegaSquirt,
    Aem,
    MaxxEcu,
    MotEc,
    Link,
    Speeduino,
    Unknown,
}

impl EcuType {
    pub fn name(&self) -> &'static str {
        match self {
            EcuType::Haltech => "Haltech",
            EcuType::EcuMaster => "ECUMaster",
            EcuType::MegaSquirt => "MegaSquirt",
            EcuType::Aem => "AEM",
            EcuType::MaxxEcu => "MaxxECU",
            EcuType::MotEc => "MoTeC",
            EcuType::Link => "Link",
            EcuType::Speeduino => "Speeduino/rusEFI",
            EcuType::Unknown => "Unknown",
        }
    }
}
