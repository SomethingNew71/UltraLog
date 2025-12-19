# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

UltraLog is a high-performance ECU (Engine Control Unit) log viewer written in pure Rust. It parses log files exported from automotive ECUs (Haltech, MegaSquirt, AEM, etc.) and displays channel data as interactive time-series graphs.

## Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run the application
cargo run --release

# Run the test parser CLI utility
cargo run --bin test_parser

# Run tests
cargo test

# Check formatting
cargo fmt --all -- --check

# Run clippy lints
cargo clippy -- -D warnings
```

## Architecture

### Source Structure

```text
src/
├── main.rs           # Application entry point
├── lib.rs            # Library exports and module declarations
├── app.rs            # Main application state and eframe::App impl
├── state.rs          # Core data types and constants
├── units.rs          # Unit preference types and conversions
├── parsers/
│   ├── mod.rs        # Parser module exports
│   ├── types.rs      # Core parser types (Log, Channel, Value, etc.)
│   └── haltech.rs    # Haltech ECU log parser
└── ui/
    ├── mod.rs        # UI module exports
    ├── sidebar.rs    # File list and view options panel
    ├── channels.rs   # Channel selection and cards
    ├── chart.rs      # Chart rendering, legends, LTTB algorithm
    ├── timeline.rs   # Timeline scrubber and playback controls
    ├── menu.rs       # Menu bar (Units, Help menus)
    ├── toast.rs      # Toast notification system
    └── icons.rs      # Custom icon drawing utilities
```

### Core Modules

- **`app.rs`** - Main `UltraLogApp` struct with application state. Contains:
  - File loading (background threads via `std::sync::mpsc`)
  - Channel management (add/remove/color assignment)
  - Cursor and time range tracking
  - eframe::App implementation

- **`state.rs`** - Core data types:
  - `LoadedFile` - Represents a parsed log file
  - `SelectedChannel` - A channel selected for visualization
  - `CacheKey`, `LoadResult`, `LoadingState` - Internal state types
  - Color palette constants (`CHART_COLORS`, `COLORBLIND_COLORS`)

- **`units.rs`** - Unit preference system:
  - Enums for each unit type (Temperature, Pressure, Speed, etc.)
  - `UnitPreferences` struct for storing user selections
  - Conversion methods between metric/imperial units

### UI Modules (src/ui/)

UI rendering is split into focused modules that implement methods on `UltraLogApp`:

- **`sidebar.rs`** - Left panel: file list, drop zone, view options (cursor tracking, colorblind mode)
- **`channels.rs`** - Right panel: channel list with search, selected channel cards
- **`chart.rs`** - Main chart with egui_plot, min/max legend overlay, LTTB downsampling, normalization
- **`timeline.rs`** - Bottom panel: playback controls (play/pause/stop), speed selector, timeline scrubber
- **`menu.rs`** - Top menu bar with Units submenu (8 unit categories) and Help menu
- **`toast.rs`** - Toast notification overlay for user feedback
- **`icons.rs`** - Custom icon drawing (upload icon for drop zone)

### Parser System

The parser system uses a trait-based design for supporting multiple ECU formats:

- **`parsers/types.rs`** - Core types: `Log`, `Channel`, `Value`, `Meta`, `EcuType`, and the `Parseable` trait
- **`parsers/haltech.rs`** - Haltech ECU log parser implementation

To add a new ECU format:

1. Create a new module in `src/parsers/` (e.g., `megasquirt.rs`)
2. Define format-specific channel types and metadata structs
3. Implement the `Parseable` trait
4. Add enum variants to `Channel`, `Meta`, and wire up in `mod.rs`

### Data Flow

1. Files are loaded asynchronously via `start_loading_file()` → background thread
2. Parser converts CSV to `Log` struct with channels, times, and data vectors
3. User selects channels → added to `selected_channels` with unique color assignment
4. Chart renders downsampled data from cache, limited to 2000 points per channel
5. Unit conversions applied at display time based on `unit_preferences`

## Key Features

- **Unit Preferences** - Users can select display units for temperature, pressure, speed, distance, fuel economy, volume, flow rate, and acceleration
- **Colorblind Mode** - Wong's optimized color palette for accessibility
- **Playback** - Play through log data at 0.25x to 8x speed
- **Cursor Tracking** - Lock view to follow cursor during playback/scrubbing
- **Min/Max Legend** - Shows peak values for each channel
- **Initial Zoom** - Charts start zoomed to first 60 seconds for better initial view

## Key Dependencies

- **eframe/egui** (0.29) - Native GUI framework
- **egui_plot** (0.29) - Charting/plotting
- **rfd** (0.15) - Native file dialogs
- **open** (5) - Cross-platform URL/email opening
- **strum** - Enum string conversion for channel types
- **regex** - Log file parsing

## Test Data

Example Haltech log files are in `exampleLogs/haltech/` for testing the parser.
