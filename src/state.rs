use std::num::NonZeroU32;

use smithay_client_toolkit::{
    compositor::CompositorHandler,
    output::{OutputHandler, OutputState},
    registry::RegistryState,
    seat::{
        keyboard::{KeyEvent, KeyboardHandler, Keysym, Modifiers, RawModifiers},
        pointer::{PointerEvent, PointerEventKind, PointerHandler},
        Capability, SeatHandler, SeatState,
    },
    shell::{
        wlr_layer::{LayerShellHandler, LayerSurface, LayerSurfaceConfigure},
        WaylandSurface,
    },
    shm::{slot::SlotPool, Shm, ShmHandler},
    seat::pointer::cursor_shape::CursorShapeManager,
};
use wayland_client::{
    protocol::{wl_keyboard, wl_output, wl_pointer, wl_seat, wl_surface},
    Connection, QueueHandle,
};
use wayland_protocols::wp::cursor_shape::v1::client::wp_cursor_shape_device_v1::Shape as CursorShape;

use crate::draw::{render_shape, render_toolbar};
use crate::types::{Point, Rect, Shape, Tool, Toolbar};

pub struct AppState {
    pub registry_state: RegistryState,
    pub seat_state: SeatState,
    pub output_state: OutputState,
    pub shm: Shm,
    pub cursor_shape_manager: Option<CursorShapeManager>,

    pub exit: bool,
    pub first_configure: bool,
    pub pool: SlotPool,
    pub width: u32,
    pub height: u32,
    pub layer: LayerSurface,
    pub keyboard: Option<wl_keyboard::WlKeyboard>,
    pub keyboard_focus: bool,
    pub modifiers: Modifiers,
    pub pointer: Option<wl_pointer::WlPointer>,

    pub toolbar: Toolbar,
    pub current_tool: Tool,
    pub smoothness: u32,
    pub last_non_zero_smoothness: u32,
    pub smooth_menu_open: bool,
    pub thickness: f32,
    pub last_non_zero_thickness: f32,
    pub thickness_menu_open: bool,
    pub active_shape: Option<Shape>,
    pub completed_shapes: Vec<Shape>,

    pub completed_canvas: tiny_skia::Pixmap,
    pub last_active_stroke_rect: Option<Rect>,
    pub pending_damage: Option<Rect>,
    pub needs_redraw: bool,
    pub frame_pending: bool,
}

impl CompositorHandler for AppState {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _new_factor: i32,
    ) {
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _new_transform: wl_output::Transform,
    ) {
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _time: u32,
    ) {
        self.frame_pending = false;
        if self.needs_redraw {
            self.draw(qh);
        }
    }

    fn surface_enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
    }

    fn surface_leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
    }
}

impl OutputHandler for AppState {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }
}

impl LayerShellHandler for AppState {
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _layer: &LayerSurface) {
        self.exit = true;
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        _layer: &LayerSurface,
        configure: LayerSurfaceConfigure,
        _serial: u32,
    ) {
        let width = NonZeroU32::new(configure.new_size.0).map_or(256, NonZeroU32::get);
        let height = NonZeroU32::new(configure.new_size.1).map_or(256, NonZeroU32::get);

        if width != self.width || height != self.height {
            self.width = width;
            self.height = height;
            // Re-create the completed canvas if size changes
            self.completed_canvas = tiny_skia::Pixmap::new(self.width, self.height).unwrap();
            self.toolbar = Toolbar::new(self.width, self.height);
            self.pending_damage = Some(Rect {
                x: 0,
                y: 0,
                w: self.width,
                h: self.height,
            });
        }

        if self.first_configure {
            self.first_configure = false;
            self.needs_redraw = true;
            if !self.frame_pending {
                self.draw(qh);
            }
        }
    }
}

impl SeatHandler for AppState {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn new_seat(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_seat::WlSeat) {}

    fn new_capability(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        seat: wl_seat::WlSeat,
        capability: Capability,
    ) {
        if capability == Capability::Keyboard && self.keyboard.is_none() {
            let keyboard = self.seat_state.get_keyboard(qh, &seat, None).unwrap();
            self.keyboard = Some(keyboard);
        }

        if capability == Capability::Pointer && self.pointer.is_none() {
            let pointer = self.seat_state.get_pointer(qh, &seat).unwrap();
            self.pointer = Some(pointer);
        }
    }

    fn remove_capability(
        &mut self,
        _conn: &Connection,
        _: &QueueHandle<Self>,
        _: wl_seat::WlSeat,
        capability: Capability,
    ) {
        if capability == Capability::Keyboard && self.keyboard.is_some() {
            self.keyboard.take().unwrap().release();
        }

        if capability == Capability::Pointer && self.pointer.is_some() {
            self.pointer.take().unwrap().release();
        }
    }

    fn remove_seat(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_seat::WlSeat) {}
}

impl KeyboardHandler for AppState {
    fn enter(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        surface: &wl_surface::WlSurface,
        _: u32,
        _: &[u32],
        _keysyms: &[Keysym],
    ) {
        if self.layer.wl_surface() == surface {
            self.keyboard_focus = true;
        }
    }

    fn leave(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        surface: &wl_surface::WlSurface,
        _: u32,
    ) {
        if self.layer.wl_surface() == surface {
            self.keyboard_focus = false;
        }
    }

    fn press_key(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _: u32,
        event: KeyEvent,
    ) {
        let is_ctrl = self.modifiers.ctrl;
        if event.keysym == Keysym::Escape {
            self.exit = true;
        } else if is_ctrl && event.keysym == Keysym::_1 {
            self.current_tool = Tool::Freehand;
            self.mark_toolbar_dirty();
        } else if is_ctrl && event.keysym == Keysym::_2 {
            self.current_tool = Tool::Rectangle;
            self.mark_toolbar_dirty();
        } else if is_ctrl && event.keysym == Keysym::_3 {
            self.current_tool = Tool::Arrow;
            self.mark_toolbar_dirty();
        } else if is_ctrl && event.keysym == Keysym::_4 {
            if self.smoothness > 0 {
                self.last_non_zero_smoothness = self.smoothness;
                self.smoothness = 0;
            } else {
                self.smoothness = self.last_non_zero_smoothness;
            }
            self.mark_toolbar_dirty();
            if !self.frame_pending {
                self.draw(qh);
            }
        }
 else if is_ctrl && event.keysym == Keysym::z {
            self.undo();
        }

        if self.needs_redraw && !self.frame_pending {
            self.draw(qh);
        }
    }

    fn repeat_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &wl_keyboard::WlKeyboard,
        _serial: u32,
        _event: KeyEvent,
    ) {
    }

    fn release_key(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _: u32,
        _event: KeyEvent,
    ) {
    }

    fn update_modifiers(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _serial: u32,
        modifiers: Modifiers,
        _raw_modifiers: RawModifiers,
        _layout: u32,
    ) {
        self.modifiers = modifiers;
    }
}

impl PointerHandler for AppState {
    fn pointer_frame(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        pointer: &wl_pointer::WlPointer,
        events: &[PointerEvent],
    ) {
        use PointerEventKind::*;
        let mut needs_redraw = false;

        for event in events {
            if &event.surface != self.layer.wl_surface() {
                continue;
            }
            match event.kind {
                Enter { serial, .. } => {
                    log::debug!("Pointer entered");
                    if let Some(manager) = &self.cursor_shape_manager {
                        let device = manager.get_shape_device(pointer, qh);
                        device.set_shape(serial, CursorShape::Default);
                        device.destroy();
                    }
                }
                Leave { .. } => {
                    if let Some(shape) = self.active_shape.take() {
                        if let Some(bounds) = shape.bounding_box() {
                            // Bake into completed canvas
                            render_shape(&mut self.completed_canvas.as_mut(), &shape);
                            self.completed_shapes.push(shape);
                            self.pending_damage = match &self.pending_damage {
                                Some(d) => Some(d.union(&bounds)),
                                None => Some(bounds),
                            };
                        }
                        self.needs_redraw = true;
                    }
                }
                Motion { .. } => {
                    if let Some(shape) = &mut self.active_shape {
                        let current_point = Point {
                            x: event.position.0 as f32,
                            y: event.position.1 as f32,
                        };
                        match shape {
                            Shape::Freehand { points, .. } => {
                                points.push(current_point);
                            }
                            Shape::Rectangle { end, .. } => {
                                *end = current_point;
                            }
                            Shape::Arrow { end, .. } => {
                                *end = current_point;
                            }
                        }
                        needs_redraw = true;
                    }
                }
                Press { button, .. } => {
                    if button == 272 || button == 273 {
                        let current_point = Point {
                            x: event.position.0 as f32,
                            y: event.position.1 as f32,
                        };

                        // 1. Check if we clicked in the flyout menus (if open)
                        if self.smooth_menu_open {
                            let flyout_x = self.toolbar.rect.x + self.toolbar.rect.w as i32 + 10;
                            let mut smooth_y = self.toolbar.rect.y;
                            for b in &self.toolbar.buttons {
                                if b.icon == Tool::Smooth {
                                    smooth_y = b.rect.y;
                                    break;
                                }
                            }

                            let flyout_rect = Rect {
                                x: flyout_x,
                                y: smooth_y,
                                w: 120, // 3 levels * 40px
                                h: 40,
                            };

                            if flyout_rect.contains(current_point.x, current_point.y) {
                                let local_x = current_point.x - flyout_x as f32;
                                let level = (local_x / 40.0).floor() as u32;
                                self.smoothness = level.min(2);
                                if self.smoothness > 0 {
                                    self.last_non_zero_smoothness = self.smoothness;
                                }
                                self.smooth_menu_open = false;
                                self.mark_toolbar_dirty();
                                if !self.frame_pending {
                                    self.draw(qh);
                                }
                                return;
                            }
                        }

                        if self.thickness_menu_open {
                            let flyout_x = self.toolbar.rect.x + self.toolbar.rect.w as i32 + 10;
                            let mut thickness_y = self.toolbar.rect.y;
                            for b in &self.toolbar.buttons {
                                if b.icon == Tool::Thickness {
                                    thickness_y = b.rect.y;
                                    break;
                                }
                            }

                            let flyout_rect = Rect {
                                x: flyout_x,
                                y: thickness_y,
                                w: 160, // 4 levels * 40px
                                h: 40,
                            };

                            if flyout_rect.contains(current_point.x, current_point.y) {
                                let local_x = current_point.x - flyout_x as f32;
                                let idx = (local_x / 40.0).floor() as usize;
                                let values = [2.0, 4.0, 8.0, 16.0];
                                self.thickness = values[idx.min(3)];
                                self.last_non_zero_thickness = self.thickness;
                                self.thickness_menu_open = false;
                                self.mark_toolbar_dirty();
                                if !self.frame_pending {
                                    self.draw(qh);
                                }
                                return;
                            }
                        }

                        // 2. Check if we clicked on the toolbar
                        let mut ui_clicked = false;
                        let mut smooth_button_clicked = false;
                        let mut thickness_button_clicked = false;
                        for btn in &self.toolbar.buttons {
                            if btn.rect.contains(current_point.x, current_point.y) {
                                ui_clicked = true;
                                if btn.icon == Tool::Undo {
                                    if button == 272 { self.undo(); }
                                } else if btn.icon == Tool::Smooth {
                                    smooth_button_clicked = true;
                                } else if btn.icon == Tool::Thickness {
                                    thickness_button_clicked = true;
                                } else if self.current_tool != btn.icon {
                                    if button == 272 { self.current_tool = btn.icon; }
                                }
                                break;
                            }
                        }

                        if smooth_button_clicked {
                            if button == 273 {
                                // Right click toggles menu
                                self.smooth_menu_open = !self.smooth_menu_open;
                                self.thickness_menu_open = false;
                            } else {
                                // Left click toggles on/off
                                if self.smoothness > 0 {
                                    self.last_non_zero_smoothness = self.smoothness;
                                    self.smoothness = 0;
                                } else {
                                    self.smoothness = self.last_non_zero_smoothness;
                                }
                                self.smooth_menu_open = false;
                                self.thickness_menu_open = false;
                            }
                            self.mark_toolbar_dirty();
                            if !self.frame_pending {
                                self.draw(qh);
                            }
                            return;
                        }

                        if thickness_button_clicked {
                            if button == 273 {
                                // Right click toggles menu
                                self.thickness_menu_open = !self.thickness_menu_open;
                                self.smooth_menu_open = false;
                            } else {
                                // Left click cycles thickness
                                let values = [2.0, 4.0, 8.0, 16.0];
                                let current_idx = values.iter().position(|&v| (v - self.thickness).abs() < 0.1).unwrap_or(1);
                                self.thickness = values[(current_idx + 1) % values.len()];
                                self.last_non_zero_thickness = self.thickness;
                                self.thickness_menu_open = false;
                                self.smooth_menu_open = false;
                            }
                            self.mark_toolbar_dirty();
                            if !self.frame_pending {
                                self.draw(qh);
                            }
                            return;
                        }

                        if ui_clicked {
                            if button == 272 {
                                self.smooth_menu_open = false;
                                self.thickness_menu_open = false;
                                self.mark_toolbar_dirty();
                                if !self.frame_pending {
                                    self.draw(qh);
                                }
                            }
                            return;
                        }

                        // If menu was open but we clicked elsewhere, close it and continue (might start drawing)
                        if self.smooth_menu_open || self.thickness_menu_open {
                            self.smooth_menu_open = false;
                            self.thickness_menu_open = false;
                            self.mark_toolbar_dirty();
                            // Don't return, we might want to start drawing a stroke
                        }

                        if !self.toolbar.rect.contains(current_point.x, current_point.y) {
                            // Only start drawing if NOT on toolbar
                            let shape = match self.current_tool {
                                Tool::Rectangle => Shape::Rectangle {
                                    start: current_point.clone(),
                                    end: current_point,
                                    color: tiny_skia::Color::from_rgba8(255, 0, 0, 255),
                                    thickness: self.thickness,
                                },
                                Tool::Arrow => Shape::Arrow {
                                    start: current_point.clone(),
                                    end: current_point,
                                    color: tiny_skia::Color::from_rgba8(255, 0, 0, 255),
                                    thickness: self.thickness,
                                },
                                Tool::Freehand => Shape::Freehand {
                                    points: vec![current_point],
                                    color: tiny_skia::Color::from_rgba8(255, 0, 0, 255),
                                    thickness: self.thickness,
                                    smoothness: self.smoothness,
                                },
                                Tool::Undo => unreachable!("Undo is an action, not a drawable tool"),
                                Tool::Smooth => unreachable!("Smooth is a toggle action"),
                                Tool::Thickness => unreachable!("Thickness is a toggle action"),
                            };
                            self.active_shape = Some(shape);
                            needs_redraw = true;
                        }
                    }
                }
                Release { button, .. } => {
                    if button == 272 {
                        if let Some(shape) = self.active_shape.take() {
                            if let Some(bounds) = shape.bounding_box() {
                                // Bake into completed canvas
                                render_shape(&mut self.completed_canvas.as_mut(), &shape);
                                self.completed_shapes.push(shape);
                                self.pending_damage = match &self.pending_damage {
                                    Some(d) => Some(d.union(&bounds)),
                                    None => Some(bounds),
                                };
                            }
                            self.needs_redraw = true;
                        }
                    }
                }
                Axis { .. } => {}
            }
        }

        if needs_redraw {
            self.needs_redraw = true;
            if !self.frame_pending {
                self.draw(qh);
            }
        }
    }
}

impl ShmHandler for AppState {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}

impl AppState {
    pub fn mark_toolbar_dirty(&mut self) {
        let mut area = self.toolbar.rect.clone();
        // Always include the flyout area in damage calculations to ensure it is cleared
        // even if it just closed this frame.
        area.w += 170; // Max flyout width + gap
        
        self.pending_damage = match &self.pending_damage {
            Some(d) => Some(d.union(&area)),
            None => Some(area),
        };
        self.needs_redraw = true;
    }

    pub fn undo(&mut self) {
        if self.completed_shapes.pop().is_some() {
            // 1. Clear the bitmap
            self.completed_canvas.fill(tiny_skia::Color::TRANSPARENT);

            // 2. Re-render all remaining shapes
            let mut pixmap = self.completed_canvas.as_mut();
            for shape in &self.completed_shapes {
                render_shape(&mut pixmap, shape);
            }

            // 3. Damage full screen to ensure the removed shape is cleared
            self.pending_damage = Some(Rect {
                x: 0,
                y: 0,
                w: self.width,
                h: self.height,
            });
            self.needs_redraw = true;
        }
    }

    pub fn draw(&mut self, qh: &QueueHandle<Self>) {
        let width = self.width;
        let height = self.height;
        let stride = width as i32 * 4;

        let (buffer, canvas) = self
            .pool
            .create_buffer(
                width as i32,
                height as i32,
                stride,
                wayland_client::protocol::wl_shm::Format::Argb8888,
            )
            .expect("create buffer");

        // 1. Calculate the final dirty rect for this frame
        let mut dirty_rect = self.pending_damage.take();

        // Add last frame's active stroke area so we erase it
        if let Some(r) = &self.last_active_stroke_rect {
            dirty_rect = match dirty_rect {
                Some(d) => Some(d.union(r)),
                None => Some(r.clone()),
            };
        }

        // Add current frame's active stroke
        let current_active_rect = self.active_shape.as_ref().and_then(|s| s.bounding_box());
        if let Some(r) = &current_active_rect {
            dirty_rect = match dirty_rect {
                Some(d) => Some(d.union(r)),
                None => Some(r.clone()),
            };
        }

        // Always ensure toolbar is redrawn if it's in the dirty area, or force it
        // For simplicity, let's just union the toolbar rect with dirty rect if anything changed
        if dirty_rect.is_some() {
            let mut ui_area = self.toolbar.rect.clone();
            if self.smooth_menu_open || self.thickness_menu_open {
                ui_area.w += 170;
            }
            dirty_rect = match dirty_rect {
                Some(d) => Some(d.union(&ui_area)),
                None => Some(ui_area),
            };
        }

        self.last_active_stroke_rect = current_active_rect;

        // If nothing needs to be redrawn, we just attach buffer and commit (or we could even skip committing entirely,
        // but smithay might expect a frame callback response. Safe bet is to draw nothing and commit).
        let dirty = match dirty_rect {
            Some(r) => {
                // Constrain the dirty rect to the actual window bounds
                let screen_bound = Rect {
                    x: 0,
                    y: 0,
                    w: width,
                    h: height,
                };
                r.intersect(&screen_bound)
            }
            None => None,
        };

        if let Some(dirty) = dirty {
            // 2. Clear only the dirty part of our wayland buffer and composite the 'done' strokes
            for y in dirty.y..(dirty.y + dirty.h as i32) {
                let y = y as usize;
                let start = (y * width as usize + dirty.x as usize) * 4;
                let len = (dirty.w as usize) * 4;
                if start + len <= canvas.len() {
                    canvas[start..start + len]
                        .copy_from_slice(&self.completed_canvas.data()[start..start + len]);
                }
            }

            {
                let mut pixmap = tiny_skia::PixmapMut::from_bytes(canvas, width, height).unwrap();
                // 3. Render the active stroke on top (it inherently clips if handled correctly by skia, or it falls within dirty bounds)
                if let Some(active) = &self.active_shape {
                    render_shape(&mut pixmap, active);
                }

                // Render the toolbar on top of EVERYTHING
                render_toolbar(
                    &mut pixmap,
                    &self.toolbar,
                    self.current_tool,
                    self.smoothness,
                    self.smooth_menu_open,
                    self.thickness,
                    self.thickness_menu_open,
                );
            }

            // 4. Convert RGBA to BGRA only in the dirty region
            for y in dirty.y..(dirty.y + dirty.h as i32) {
                let y = y as usize;
                let start = (y * width as usize + dirty.x as usize) * 4;
                let len = (dirty.w as usize) * 4;
                if start + len <= canvas.len() {
                    for chunk in canvas[start..start + len].chunks_exact_mut(4) {
                        chunk.swap(0, 2);
                    }
                }
            }

            self.layer
                .wl_surface()
                .damage_buffer(dirty.x, dirty.y, dirty.w as i32, dirty.h as i32);
            self.layer
                .wl_surface()
                .frame(qh, self.layer.wl_surface().clone());
            self.frame_pending = true;
            buffer
                .attach_to(self.layer.wl_surface())
                .expect("buffer attach");
            self.layer.commit();
            self.needs_redraw = false;
        } else {
            // Nothing to draw
            // We just don't commit a new buffer or a new frame callback!
            // This halts the loop until `needs_redraw` becomes true again.
        }
    }
}
