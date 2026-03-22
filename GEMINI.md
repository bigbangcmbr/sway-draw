# Sway-Draw: Technical Mandates & Project Context

## Project Overview
Sway-Draw is a lightweight, native Wayland screen annotation utility. It functions as a transparent overlay using the `wlr-layer-shell` protocol, allowing users to draw directly over their desktop environment.

### Core Stack
- **Language**: Rust (Edition 2021)
- **Windowing/Protocol**: `smithay-client-toolkit` (SCTK), raw Wayland protocols.
- **Rendering**: `tiny-skia` (Software-based 2D vector graphics).
- **SVG Handling**: `resvg` / `usvg` for UI icons.

## Architecture
The application follows a modular, state-driven architecture designed for high performance on high-resolution (4K) displays.

- **`src/main.rs`**: Entry point. Handles Wayland connection, registry initialization, and the global event loop.
- **`src/state.rs`**: The core engine. Implements SCTK handlers (`CompositorHandler`, `PointerHandler`, etc.) and manages `AppState`.
- **`src/draw.rs`**: Pure rendering logic. Contains algorithms for vector shape rasterization and UI components.
- **`src/types.rs`**: Data primitives (`Point`, `Rect`), tool definitions, and UI geometry (`Toolbar`, `Button`).

## Key Engineering Standards

### 1. Partial Damage Tracking (Performance)
To maintain responsiveness without GPU acceleration, Sway-Draw **must not** redraw the entire screen every frame.
- **Mandate**: Every visual change must calculate a precise `dirty_rect`.
- **Mechanism**: The `draw()` method in `state.rs` uses the union of `pending_damage`, `last_active_stroke_rect`, and the `Toolbar` rect to minimize `wl_surface::damage_buffer` calls.
- **Persistent Canvas**: Finished strokes are baked into a `completed_canvas` pixmap to avoid re-rendering the entire vector history during standard pointer movement.

### 2. State-Based UI
The toolbar and flyout menus are rendered manually into the shared memory buffer.
- **Interaction**: The `PointerHandler` detects clicks based on `Rect::contains`.
- **Feedback**: UI updates must trigger an immediate `draw()` call if a frame is not already pending to ensure snappy interaction.

### 3. Vector-First History
To support the **Undo** feature, all completed strokes are stored as `Shape` enums in a `completed_shapes` vector. 
- **Baking**: Shapes are only "lost" to pixels in the `completed_canvas` for performance, but the vector data is preserved for reconstruction during undo operations.

### 4. Smoothing Algorithm
Freehand curves use a multi-pass Laplacian smoothing filter combined with Quadratic Bézier conversion.
- **Levels**: Supports 0 (None), 1 (Low), and 2 (High).
- **Iteration**: High-level smoothing uses 8 iterations of weighted averaging.

## Development Workflow

### Building & Running
- **Build**: `cargo build`
- **Run**: `cargo run` (Requires a Wayland compositor supporting `wlr-layer-shell`).
- **Debug**: Use `RUST_LOG=debug cargo run` to see protocol and input logs.

### Tool Control Shortcuts
- **Freehand**: `Ctrl+1`
- **Rectangle**: `Ctrl+2`
- **Line/Arrow**: `Ctrl+3` (Right-click button to toggle mode)
- **Toggle Smoothing**: `Ctrl+4` (Toggles between Off and Last Active Level).
- **Undo**: `Ctrl+Z`

### UI Interaction
- **Left Click**: Select tool or toggle smoothing.
- **Right Click** (on Smooth button): Open horizontal flyout for level selection.
- **Escape**: Exit application.
