use crate::types::{Shape, Tool, Toolbar};

pub fn render_toolbar(pixmap: &mut tiny_skia::PixmapMut, toolbar: &Toolbar, current_tool: Tool) {
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
        let is_active = button.icon == current_tool;
        if is_active {
            button_paint.set_color(tiny_skia::Color::from_rgba8(80, 80, 200, 255)); // Highlighted
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
    }
}

pub fn render_shape(pixmap: &mut tiny_skia::PixmapMut, shape: &Shape) {
    let mut pb = tiny_skia::PathBuilder::new();
    let (color, thickness) = match shape {
        Shape::Freehand {
            points,
            color,
            thickness,
        } => {
            if points.len() < 2 {
                return;
            }
            pb.move_to(points[0].x, points[0].y);
            for p in &points[1..] {
                pb.line_to(p.x, p.y);
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
