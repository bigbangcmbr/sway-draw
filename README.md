# Sway-Draw 🖌️

A lightweight, native Wayland screen annotation and drawing utility designed for the [Sway](https://swaywm.org/) compositor.

## ✨ Overview

**Sway-Draw** acts as a transparent overlay on top of your existing windows, allowing you to quickly draw and annotate directly on your screen. It uses pure Wayland protocols (`wlr-layer-shell`) and software rendering (`tiny-skia`) for a fast, minimal, and responsive experience. 🚀

## 🌟 Features

- **🛡️ Native Wayland**: Built with `smithay-client-toolkit` for direct, high-performance Wayland integration.
- **🪶 Ultra-Lightweight**: Software rendering via `tiny-skia` into shared memory buffers (`wl_shm`). No heavy UI toolkits or GPU dependencies.
- **⚡ Performance Optimized**: Implements partial screen damage tracking. Instead of redrawing the entire 4K screen on every frame, it only calculates and updates the precise bounding boxes of your strokes.
- **🖌️ Smooth Strokes**: Multi-pass Laplacian smoothing combined with Quadratic Bézier curves for professional-looking freehand drawing.

## 📋 Prerequisites

- A Wayland compositor that supports `wlr-layer-shell` (e.g., **Sway**, **Hyprland**). 🖥️
- Rust toolchain (`cargo`). 🦀

## 🛠️ Building and Running

Clone the repository and run:

```bash
# Clone the repo
git clone https://github.com/vedanshbodkhe21/sway-draw.git
cd sway-draw

# Build and run directly
cargo run --release
```

## ⌨️ Usage & Shortcuts

- **Launch**: Bind `sway-draw` to a key in your compositor config for instant access.
- **Draw**: Left click and drag to draw.
- **Tools**:
    - `Ctrl + 1`: Freehand Tool 🖋️
    - `Ctrl + 2`: Rectangle Tool ⬛
    - `Ctrl + 3`: Line & Arrow Tool ↗️ (Right-click button to toggle)
- **Actions**:
    - `Ctrl + 4`: Toggle Smoothing 🌊
    - `Ctrl + Z`: Undo last stroke 🔙
    - `Ctrl + D`: Clear Screen 🗑️
    - `Esc`: Exit and clear all annotations ❌

## 🏗️ Architecture

For more details on the internal design, rendering engine, and module structure, please see [Architecture.md](./Architecture.md).

---
Built with ❤️ for the Wayland ecosystem.
