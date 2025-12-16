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
```

## Architecture

### Core Modules

- **`src/main.rs`** - Application entry point, configures eframe/egui window
- **`src/app.rs`** - Main UI application state and rendering logic (`UltraLogApp`)
- **`src/lib.rs`** - Library exports for bin access
- **`src/parsers/`** - ECU log file parsing system

### Parser System

The parser system uses a trait-based design for supporting multiple ECU formats:

- **`parsers/types.rs`** - Core types: `Log`, `Channel`, `Value`, `Meta`, `EcuType`, and the `Parseable` trait
- **`parsers/haltech.rs`** - Haltech ECU log parser implementation

To add a new ECU format:
1. Create a new module in `src/parsers/` (e.g., `megasquirt.rs`)
2. Define format-specific channel types and metadata structs
3. Implement the `Parseable` trait
4. Add enum variants to `Channel`, `Meta`, and wire up in `mod.rs`

### UI Architecture (egui)

The app uses egui's immediate-mode GUI pattern:

- **Left panel**: File list with drag-and-drop support
- **Right panel**: Channel selection with search filtering
- **Center**: Selected channel cards + interactive chart (egui_plot)

Key performance optimizations:
- Background thread file loading via `std::sync::mpsc` channels
- LTTB (Largest Triangle Three Buckets) downsampling for chart rendering
- Cached downsampled data to avoid recomputation each frame

### Data Flow

1. Files are loaded asynchronously via `start_loading_file()` → background thread
2. Parser converts CSV to `Log` struct with channels, times, and data vectors
3. User selects channels → added to `selected_channels` with unique color assignment
4. Chart renders downsampled data from cache, limited to 2000 points per channel

## Key Dependencies

- **eframe/egui** (0.29) - Native GUI framework
- **egui_plot** (0.29) - Charting/plotting
- **rfd** (0.15) - Native file dialogs
- **strum** - Enum string conversion for channel types
- **regex** - Log file parsing

## Test Data

Example Haltech log files are in `exampleLogs/haltech/` for testing the parser.
