//! Core application state types and constants.
//!
//! This module contains the fundamental data structures used throughout
//! the application, including loaded files, selected channels, and color palettes.

use std::path::PathBuf;

use crate::parsers::{Channel, EcuType, Log};

// ============================================================================
// Constants
// ============================================================================

/// Maximum number of channels that can be selected simultaneously
pub const MAX_CHANNELS: usize = 10;

/// Maximum points to render in chart (for performance via LTTB downsampling)
pub const MAX_CHART_POINTS: usize = 2000;

/// Color palette for chart lines (matches original theme)
pub const CHART_COLORS: &[[u8; 3]] = &[
    [113, 120, 78],  // Olive green (primary)
    [191, 78, 48],   // Rust orange (accent)
    [71, 108, 155],  // Blue (info)
    [159, 166, 119], // Sage green (success)
    [253, 193, 73],  // Amber (warning)
    [135, 30, 28],   // Dark red (error)
    [246, 247, 235], // Cream
    [100, 149, 237], // Cornflower blue
    [255, 127, 80],  // Coral
    [144, 238, 144], // Light green
];

/// Colorblind-friendly palette (based on Wong's optimized palette)
/// Designed to be distinguishable for deuteranopia, protanopia, and tritanopia
pub const COLORBLIND_COLORS: &[[u8; 3]] = &[
    [0, 114, 178],   // Blue
    [230, 159, 0],   // Orange
    [0, 158, 115],   // Bluish green
    [204, 121, 167], // Reddish purple
    [86, 180, 233],  // Sky blue
    [213, 94, 0],    // Vermillion
    [240, 228, 66],  // Yellow
    [0, 0, 0],       // Black (for contrast on light backgrounds, shows as white on dark)
    [136, 204, 238], // Light blue
    [153, 153, 153], // Gray
];

// ============================================================================
// Core Types
// ============================================================================

/// Represents a loaded log file with its parsed data
#[derive(Clone)]
pub struct LoadedFile {
    /// Path to the original file
    pub path: PathBuf,
    /// Display name for the file
    pub name: String,
    /// Type of ECU that generated this log
    pub ecu_type: EcuType,
    /// Parsed log data
    pub log: Log,
}

/// A channel selected for visualization on the chart
#[derive(Clone)]
pub struct SelectedChannel {
    /// Index of the file this channel belongs to
    pub file_index: usize,
    /// Index of the channel within the file
    pub channel_index: usize,
    /// The channel data itself
    pub channel: Channel,
    /// Index into the color palette for this channel's line
    pub color_index: usize,
}

/// Result from background file loading operation
pub enum LoadResult {
    Success(Box<LoadedFile>),
    Error(String),
}

/// Current state of file loading
pub enum LoadingState {
    /// No loading in progress
    Idle,
    /// Loading a file (contains filename being loaded)
    Loading(String),
}

/// Cache key for downsampled data, uniquely identifying a channel's data
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct CacheKey {
    pub file_index: usize,
    pub channel_index: usize,
}
