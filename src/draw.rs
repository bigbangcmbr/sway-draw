use crate::types::{Point, Shape, Tool, Toolbar};

fn smooth_points(points: &[Point], level: u32) -> Vec<Point> {
    if level == 0 || points.len() < 3 {
        return points.to_vec();
    }
    
    // Level 1: 3 iterations, Level 2: 8 iterations for extra smoothness
    let iterations = if level == 1 { 3 } else { 8 };
    
    let mut current = points.to_vec();
    for _ in 0..iterations {
        let mut next = Vec::with_capacity(current.len());
        next.push(current[0].clone());
        for i in 1..current.len() - 1 {
            let p_prev = &current[i - 1];
            let p_curr = &current[i];
            let p_next = &current[i + 1];
            
            // Stronger smoothing kernel for level 2
            let weight = if level == 1 { 2.0 } else { 1.0 }; 
            let total_weight = 2.0 + weight;
            
            next.push(Point {
                x: (p_prev.x + p_curr.x * weight + p_next.x) / total_weight,
                y: (p_prev.y + p_curr.y * weight + p_next.y) / total_weight,
            });
        }
        next.push(current[current.len() - 1].clone());
        current = next;
    }
    current
}

pub fn render_toolbar(
    pixmap: &mut tiny_skia::PixmapMut,
    toolbar: &Toolbar,
    current_tool: Tool,
    smoothness: u32,
    smooth_menu_open: bool,
    thickness: f32,
    thickness_menu_open: bool,
) {
    let mut paint = tiny_skia::Paint::default();
    paint.set_color(tiny_skia::Color::from_rgba8(40, 44, 52, 230)); // Dark background with alpha

    let rect = tiny_skia::Rect::from_xywh(
        toolbar.rect.x as f32,
        toolbar.rect.y as f32,
        toolbar.rect.w as f32,
        toolbar.rect.h as f32,
    )
    .unwrap();

    pixmap.fill_rect(rect, &paint, tiny_skia::Transform::identity(), None);

    for button in &toolbar.buttons {
        let mut button_paint = tiny_skia::Paint::default();
        let is_active = if button.icon == Tool::Smooth {
            smoothness > 0
        } else if button.icon == Tool::Thickness {
            true // Thickness is always "active" in terms of showing a level
        } else {
            button.icon == current_tool
        };

        if is_active {
            let alpha = if button.icon == Tool::Smooth {
                if smoothness == 1 { 150 } else { 255 }
            } else if button.icon == Tool::Thickness {
                150 // Constant light highlight for thickness
            } else {
                255
            };
            button_paint.set_color(tiny_skia::Color::from_rgba8(80, 80, 200, alpha));
        } else {
            button_paint.set_color(tiny_skia::Color::from_rgba8(60, 64, 72, 255));
        }

        let button_rect = tiny_skia::Rect::from_xywh(
            button.rect.x as f32,
            button.rect.y as f32,
            button.rect.w as f32,
            button.rect.h as f32,
        )
        .unwrap();

        pixmap.fill_rect(
            button_rect,
            &button_paint,
            tiny_skia::Transform::identity(),
            None,
        );

        // Render SVG icon
        let icon_padding = 8.0;
        let icon_size = button.rect.w as f32 - (icon_padding * 2.0);

        let ts = tiny_skia::Transform::from_translate(
            button.rect.x as f32 + icon_padding,
            button.rect.y as f32 + icon_padding,
        )
        .pre_scale(icon_size / 24.0, icon_size / 24.0); // SVGs are 24x24

        resvg::render(&button.svg_tree, ts, pixmap);

        // Render Flyout for Smooth
        if button.icon == Tool::Smooth && smooth_menu_open {
            let flyout_x = toolbar.rect.x as f32 + toolbar.rect.w as f32 + 10.0;
            let flyout_y = button.rect.y as f32;
            let flyout_w = 120.0; // 40px * 3 levels
            let flyout_h = 40.0;

            let mut flyout_paint = tiny_skia::Paint::default();
            flyout_paint.set_color(tiny_skia::Color::from_rgba8(40, 44, 52, 230));

            let flyout_rect = tiny_skia::Rect::from_xywh(flyout_x, flyout_y, flyout_w, flyout_h).unwrap();
            pixmap.fill_rect(flyout_rect, &flyout_paint, tiny_skia::Transform::identity(), None);

            // Level Buttons (0: None, 1: Low, 2: High)
            for level in 0..3 {
                let level_x = flyout_x + (level as f32 * 40.0);
                let is_level_active = level == smoothness;

                let mut level_paint = tiny_skia::Paint::default();
                if is_level_active {
                    level_paint.set_color(tiny_skia::Color::from_rgba8(80, 80, 200, 255));
                } else {
                    level_paint.set_color(tiny_skia::Color::from_rgba8(70, 74, 82, 255));
                }

                let level_rect = tiny_skia::Rect::from_xywh(level_x + 2.0, flyout_y + 2.0, 36.0, 36.0).unwrap();
                pixmap.fill_rect(level_rect, &level_paint, tiny_skia::Transform::identity(), None);

                // Render level SVG icon centered in the 36x36 button
                if let Some(tree) = toolbar.smooth_level_icons.get(level as usize) {
                    let target_size = 24.0;
                    let scale = target_size / 24.0;
                    let offset = (36.0 - target_size) / 2.0;
                    let ts = tiny_skia::Transform::from_translate(level_x + 2.0 + offset, flyout_y + 2.0 + offset)
                        .pre_scale(scale, scale);
                    
                    resvg::render(tree, ts, pixmap);
                }
            }
        }

        // Render Flyout for Thickness
        if button.icon == Tool::Thickness && thickness_menu_open {
            let flyout_x = toolbar.rect.x as f32 + toolbar.rect.w as f32 + 10.0;
            let flyout_y = button.rect.y as f32;
            let flyout_w = 160.0; // 40px * 4 levels
            let flyout_h = 40.0;

            let mut flyout_paint = tiny_skia::Paint::default();
            flyout_paint.set_color(tiny_skia::Color::from_rgba8(40, 44, 52, 230));

            let flyout_rect = tiny_skia::Rect::from_xywh(flyout_x, flyout_y, flyout_w, flyout_h).unwrap();
            pixmap.fill_rect(flyout_rect, &flyout_paint, tiny_skia::Transform::identity(), None);

            let thickness_values = [2.0, 4.0, 8.0, 16.0];
            for (idx, &val) in thickness_values.iter().enumerate() {
                let level_x = flyout_x + (idx as f32 * 40.0);
                let is_level_active = (val - thickness).abs() < 0.1;

                let mut level_paint = tiny_skia::Paint::default();
                if is_level_active {
                    level_paint.set_color(tiny_skia::Color::from_rgba8(80, 80, 200, 255));
                } else {
                    level_paint.set_color(tiny_skia::Color::from_rgba8(70, 74, 82, 255));
                }

                let level_rect = tiny_skia::Rect::from_xywh(level_x + 2.0, flyout_y + 2.0, 36.0, 36.0).unwrap();
                pixmap.fill_rect(level_rect, &level_paint, tiny_skia::Transform::identity(), None);

                if let Some(tree) = toolbar.thickness_icons.get(idx) {
                    let target_size = 24.0;
                    let scale = target_size / 24.0;
                    let offset = (36.0 - target_size) / 2.0;
                    let ts = tiny_skia::Transform::from_translate(level_x + 2.0 + offset, flyout_y + 2.0 + offset)
                        .pre_scale(scale, scale);
                    
                    resvg::render(tree, ts, pixmap);
                }
            }
        }
    }
}

pub fn render_shape(pixmap: &mut tiny_skia::PixmapMut, shape: &Shape) {
    let mut pb = tiny_skia::PathBuilder::new();
    let (color, thickness) = match shape {
        Shape::Freehand {
            points,
            color,
            thickness,
            smoothness,
        } => {
            if points.len() < 2 {
                return;
            }
            if *smoothness > 0 && points.len() >= 3 {
                // Apply iterative smoothing filter first
                let points = smooth_points(points, *smoothness);

                pb.move_to(points[0].x, points[0].y);

                for i in 1..points.len() - 1 {
                    let p1 = &points[i];
                    let p2 = &points[i + 1];

                    let mid_x = (p1.x + p2.x) / 2.0;
                    let mid_y = (p1.y + p2.y) / 2.0;

                    pb.quad_to(p1.x, p1.y, mid_x, mid_y);
                }

                let last = &points[points.len() - 1];
                pb.line_to(last.x, last.y);
            } else {
                pb.move_to(points[0].x, points[0].y);
                for p in &points[1..] {
                    pb.line_to(p.x, p.y);
                }
            }
            (*color, *thickness)
        }
        Shape::Rectangle {
            start,
            end,
            color,
            thickness,
        } => {
            let min_x = start.x.min(end.x);
            let min_y = start.y.min(end.y);
            let w = (start.x - end.x).abs();
            let h = (start.y - end.y).abs();

            pb.move_to(min_x, min_y);
            pb.line_to(min_x + w, min_y);
            pb.line_to(min_x + w, min_y + h);
            pb.line_to(min_x, min_y + h);
            pb.close();

            (*color, *thickness)
        }
        Shape::Arrow {
            start,
            end,
            color,
            thickness,
        } => {
            // Main line
            pb.move_to(start.x, start.y);
            pb.line_to(end.x, end.y);

            // Arrowhead
            let dx = end.x - start.x;
            let dy = end.y - start.y;
            let len = (dx * dx + dy * dy).sqrt();

            if len > 0.1 {
                let head_len = 15.0 + (*thickness * 1.5); // Adjust size as needed
                let head_angle = std::f32::consts::PI / 6.0; // 30 degrees

                let angle = dy.atan2(dx);

                let x1 = end.x - head_len * (angle - head_angle).cos();
                let y1 = end.y - head_len * (angle - head_angle).sin();

                let x2 = end.x - head_len * (angle + head_angle).cos();
                let y2 = end.y - head_len * (angle + head_angle).sin();

                pb.move_to(end.x, end.y);
                pb.line_to(x1, y1);
                pb.move_to(end.x, end.y);
                pb.line_to(x2, y2);
            }

            (*color, *thickness)
        }
    };

    if let Some(path) = pb.finish() {
        let mut paint = tiny_skia::Paint::default();
        paint.set_color(color);
        let stroke_opts = tiny_skia::Stroke {
            width: thickness,
            line_cap: tiny_skia::LineCap::Round,
            line_join: tiny_skia::LineJoin::Round,
            ..Default::default()
        };
        pixmap.stroke_path(
            &path,
            &paint,
            &stroke_opts,
            tiny_skia::Transform::identity(),
            None,
        );
    }
}
