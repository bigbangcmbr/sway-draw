# Sway-Draw Architecture Overview

## Core Philosophy
- **"From Scratch" Approach**: Bypass heavy UI toolkits (GTK/Qt) for a lightweight, native Wayland experience.
- **Overlay Rendering**: Act as a transparent overlay on top of existing windows rather than a traditional tile/floating window.
- **Software Rendering**: Use a CPU-based 2D vector graphics library to draw into a shared memory buffer.
## Wayland Integration (The Windowing Layer)
- **Protocol**: Raw Wayland client implementation.
- **Surface Boundary**: Use the `wlr-layer-shell` protocol. It allows the surface to be pinned as an `overlay` layer (above all standard windows and panels) and bypass normal Sway tiling rules.
- **Buffer Management**: `wl_shm` (Shared Memory) will be used to allocate memory that both the app and the Sway compositor can read/write to.
- **Cursor Management**: Uses the `wp-cursor-shape-v1` protocol to set a native arrow cursor, ensuring the utility feels like a standard system tool rather than a transparent "dead zone."
- **Multimonitor Handling (Planned)**: The current version supports the primary monitor. Future updates will listen to `wl_output` events to span correctly across multiple monitors by spawning a separate layer-shell surface for each output.

## Input Handling
- Listen purely to standard Wayland `wl_pointer` and `wl_keyboard` events.
- **Pointer Events**: Capture standard coordinate data (X, Y) and button states (left click to draw). 
- **UI Interaction**: Manually hit-test pointer events against `Rect` primitives to handle toolbar buttons, toggle-actions, and flyout menus.
- **Keyboard Events**: Capture specific keybinds natively:
    - `Esc`: Quit application.
    - `Ctrl+1`: Laser Pen Tool (Fading).
    - `Ctrl+2`: Freehand Tool.
    - `Ctrl+3`: Rectangle Tool.
    - `Ctrl+4`: Line & Arrow Tool.
    - `Ctrl+5`: Toggle Smoothing.
    - `Ctrl+Z`: Undo last stroke.
    - `Ctrl+D`: Clear screen.

## Rendering Engine
- **Libraries**: `tiny-skia` (Software-based 2D vector graphics) and `resvg` / `usvg` (SVG icon rendering).
- **Process**:
  1. Map a chunk of memory that both the app and Sway can access (`wl_shm`).
  2. Treat that memory as a transparent RGBA pixel buffer.
  3. When a user interacts, calculate the geometry (lines, rectangles, smoothed Bézier curves) and instruct the rendering library to rasterize those shapes into the buffer.
  4. Submit (commit) the modified buffer to Sway for integration on the screen.
- **Toolbar & Flyouts**: The UI is rendered manually into the same buffer. Supports interactive flyout menus for advanced settings (Smoothing levels, Line Thickness) and visual separators to group tool types.
- **Damage Tracking (Performance)**: Instead of redrawing the full 4K screen on every frame, the application calculates a precise `dirty_rect` combining the bounding boxes of new strokes, active ongoing strokes, fading laser trails, and any UI changes (like opening a flyout). It persists committed strokes into a `completed_canvas` buffer in standard memory, and copies over only the bounds of the `dirty_rect` into the Wayland canvas to submit minimal `damage_buffer()` requests. Continuous animation (e.g. fading lasers) continuously drives this minimal redraw cycle.

## State Management
- **Vector-based Data Model**: Store drawings as mathematical data (e.g., coordinates, thickness, color, smoothing level), not raw pixel bitmaps.
- **Undo/Clear**: Supports popping the last shape from the stack for Undo, or purging the entire stack for a full Clear action.
- **Smoothing Algorithm**: Employs a multi-pass Laplacian smoothing filter followed by Quadratic Bézier conversion for "ink-like" freehand drawing.
- **Laser Pen (Fading)**: Fading trails are managed outside the core persistence model. Laser lines decay point-by-point based on age, continuously updating bounding boxes and triggering animation loop redraws until fully faded. They are ignored by `completed_canvas` to preserve transience.

- `src/main.rs`: Execution entry point containing the Wayland connection, registry startup logic, and event loop.
- `src/state.rs`: Holds the massive `AppState` structure, manages damage rectangles alongside `completed_canvas`, handles compositor rendering (`.draw()`), and delegates all native Wayland event interactions via smithay protocol handlers.
- `src/draw.rs`: Dedicated module containing pure algorithmic drawing subroutines interfacing with `tiny-skia` (e.g., parsing path builders for `Stroke` rendering).
- `src/types.rs`: Mathematical and state primitives: coordinates (`Point`), color structures (`Shape`), and geometry definitions for the `Toolbar` and UI.

## Application Lifecycle
- **One-Shot Execution**: Launched via a Sway `$mod` keybind. Runs until the user resolves the annotation (e.g., presses `Escape` or copies the screen), at which point it clears the Wayland surfaces and destructs completely.
