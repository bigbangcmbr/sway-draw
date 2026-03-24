use resvg::usvg;

const SVG_LASER: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="M5 5l1.5 1.5"/><path d="M17.5 17.5L19 19"/><path d="M19 5l-1.5 1.5"/><path d="M5 19l1.5-1.5"/></svg>"#;
const SVG_FREEHAND: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 19l7-7 3 3-7 7-3-3z"/><path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z"/><path d="M2 2l8 8"/><path d="M2 22l5-5"/></svg>"#;
const SVG_RECTANGLE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/></svg>"#;
const SVG_LINE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/></svg>"#;
const SVG_ARROW: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>"#;
const SVG_SMOOTH: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3V5"/><path d="M5 8H3"/><path d="M21 8H19"/><path d="M18 15L16 13L14 15"/><path d="M16 13V21"/><path d="M12 21H16"/><path d="M7 21H11"/><path d="M9 21V13L7 15"/><path d="M2 13L4 15L6 13"/><path d="M18 13L20 15L22 13"/></svg>"#;
const SVG_THICKNESS: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="4" y1="12" x2="20" y2="12" stroke-width="1"/><line x1="4" y1="16" x2="20" y2="16" stroke-width="4"/><line x1="4" y1="20" x2="20" y2="20" stroke-width="8"/></svg>"#;
const SVG_CLEAR: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/><line x1="10" y1="11" x2="10" y2="17"/><line x1="14" y1="11" x2="14" y2="17"/></svg>"#;
const SVG_UNDO: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 7v6h6"/><path d="M21 17a9 9 0 00-9-9 9 9 0 00-6 2.3L3 13"/></svg>"#;

const SVG_SMOOTH_0: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12h18"/><path d="M12 3v18" opacity="0.1"/></svg>"#;
const SVG_SMOOTH_1: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 16c3-2 6-2 9 0s6 2 9 0"/></svg>"#;
const SVG_SMOOTH_2: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12c6-8 12 8 18 0"/></svg>"#;

const SVG_THICKNESS_1: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="2" fill="currentColor"/></svg>"#;
const SVG_THICKNESS_2: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4" fill="currentColor"/></svg>"#;
const SVG_THICKNESS_3: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="6" fill="currentColor"/></svg>"#;
const SVG_THICKNESS_4: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="8" fill="currentColor"/></svg>"#;

fn get_tool_svg(tool: Tool) -> &'static str {
    match tool {
        Tool::Laser => SVG_LASER,
        Tool::Freehand => SVG_FREEHAND,
        Tool::Rectangle => SVG_RECTANGLE,
        Tool::Line => SVG_LINE,
        Tool::Smooth => SVG_SMOOTH,
        Tool::Thickness => SVG_THICKNESS,
        Tool::Clear => SVG_CLEAR,
        Tool::Undo => SVG_UNDO,
    }
}


#[derive(Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x as f32
            && x <= (self.x + self.w as i32) as f32
            && y >= self.y as f32
            && y <= (self.y + self.h as i32) as f32
    }

    pub fn union(&self, other: &Rect) -> Rect {
        let max_x = std::cmp::max(self.x + self.w as i32, other.x + other.w as i32);
        let max_y = std::cmp::max(self.y + self.h as i32, other.y + other.h as i32);
        let min_x = std::cmp::min(self.x, other.x);
        let min_y = std::cmp::min(self.y, other.y);

        Rect {
            x: min_x,
            y: min_y,
            w: (max_x - min_x) as u32,
            h: (max_y - min_y) as u32,
        }
    }

    pub fn intersect(&self, other: &Rect) -> Option<Rect> {
        let max_x = std::cmp::min(self.x + self.w as i32, other.x + other.w as i32);
        let max_y = std::cmp::min(self.y + self.h as i32, other.y + other.h as i32);
        let min_x = std::cmp::max(self.x, other.x);
        let min_y = std::cmp::max(self.y, other.y);

        if min_x < max_x && min_y < max_y {
            Some(Rect {
                x: min_x,
                y: min_y,
                w: (max_x - min_x) as u32,
                h: (max_y - min_y) as u32,
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tool {
    Rectangle,
    Line,
    Freehand,
    Laser,
    Smooth,
    Thickness,
    Clear,
    Undo,
}

#[derive(Clone, Debug)]
pub enum Shape {
    LaserLine {
        points: Vec<(Point, std::time::Instant)>,
        _color: tiny_skia::Color,
        thickness: f32,
    },
    Freehand {
        points: Vec<Point>,
        color: tiny_skia::Color,
        thickness: f32,
        smoothness: u32,
    },
    Rectangle {
        start: Point,
        end: Point,
        color: tiny_skia::Color,
        thickness: f32,
    },
    Line {
        start: Point,
        end: Point,
        color: tiny_skia::Color,
        thickness: f32,
        has_arrow: bool,
    },
}

impl Shape {
    pub fn bounding_box(&self) -> Option<Rect> {
        let (mut min_x, mut min_y, mut max_x, mut max_y, thickness) = match self {
            Shape::LaserLine {
                points, thickness, ..
            } => {
                if points.is_empty() {
                    return None;
                }
                let mut min_x = points[0].0.x;
                let mut min_y = points[0].0.y;
                let mut max_x = points[0].0.x;
                let mut max_y = points[0].0.y;

                for (p, _) in &points[1..] {
                    min_x = min_x.min(p.x);
                    min_y = min_y.min(p.y);
                    max_x = max_x.max(p.x);
                    max_y = max_y.max(p.y);
                }
                (min_x, min_y, max_x, max_y, *thickness)
            }
            Shape::Freehand {
                points, thickness, ..
            } => {
                if points.is_empty() {
                    return None;
                }
                let mut min_x = points[0].x;
                let mut min_y = points[0].y;
                let mut max_x = points[0].x;
                let mut max_y = points[0].y;

                for p in &points[1..] {
                    min_x = min_x.min(p.x);
                    min_y = min_y.min(p.y);
                    max_x = max_x.max(p.x);
                    max_y = max_y.max(p.y);
                }
                (min_x, min_y, max_x, max_y, *thickness)
            }
            Shape::Rectangle {
                start,
                end,
                thickness,
                ..
            }
            | Shape::Line {
                start,
                end,
                thickness,
                ..
            } => {
                let min_x = start.x.min(end.x);
                let min_y = start.y.min(end.y);
                let max_x = start.x.max(end.x);
                let max_y = start.y.max(end.y);
                (min_x, min_y, max_x, max_y, *thickness)
            }
        };

        // Pad by thickness
        let pad = thickness / 2.0 + 2.0; // slight extra padding for anti-aliasing edge cases

        // For Arrow/Line with arrow, pad a bit more to account for arrowhead which can extend slightly beyond bounds
        let pad = match self {
            Shape::Line { thickness, has_arrow: true, .. } => pad + (*thickness * 3.0),
            Shape::LaserLine { thickness, .. } => pad + (*thickness * 1.5) + 1.0, // Laser has a 4x thickness max bloom
            _ => pad,
        };

        min_x -= pad;
        min_y -= pad;
        max_x += pad;
        max_y += pad;

        Some(Rect {
            x: min_x.floor() as i32,
            y: min_y.floor() as i32,
            w: (max_x.ceil() - min_x.floor()) as u32,
            h: (max_y.ceil() - min_y.floor()) as u32,
        })
    }
}

pub struct Button {
    pub rect: Rect,
    pub icon: Tool,
    pub svg_tree: usvg::Tree,
}

pub struct Toolbar {
    pub rect: Rect,
    pub buttons: Vec<Button>,
    pub smooth_level_icons: Vec<usvg::Tree>,
    pub thickness_icons: Vec<usvg::Tree>,
    pub line_icons: Vec<usvg::Tree>,
}

impl Toolbar {
    pub fn new(_screen_width: u32, screen_height: u32) -> Self {
        let width = 60;
        let button_size = 40;
        let padding = 10;
        let x = 20; // Positioned 20px from left
        let y = (screen_height as i32 - (8 * (button_size + padding))) / 2; // Centered vertically, 8 buttons now

        let mut buttons = Vec::new();
        let tools = [
            Tool::Laser,
            Tool::Freehand,
            Tool::Rectangle,
            Tool::Line,
            Tool::Smooth,
            Tool::Thickness,
            Tool::Clear,
            Tool::Undo,
        ];

        let mut opt = usvg::Options::default();
        opt.font_family = "sans-serif".to_string();

        for (i, tool) in tools.iter().enumerate() {
            let svg_str = get_tool_svg(*tool);
            let svg_tree = usvg::Tree::from_str(svg_str, &opt).unwrap();

            // Add extra space after the third tool (Line) and fifth tool (Thickness) for separators
            // Current padding is 10px. Adding 10px more creates a 20px total gap.
            let mut extra_y = 0;
            if i >= 4 { extra_y += 10; }
            if i >= 6 { extra_y += 10; }

            buttons.push(Button {
                rect: Rect {
                    x: x + (width - button_size) / 2,
                    y: y + padding + (i as i32 * (button_size + padding)) + extra_y,
                    w: button_size as u32,
                    h: button_size as u32,
                },
                icon: *tool,
                svg_tree,
            });
        }

        let mut smooth_level_icons = Vec::new();
        for svg_str in [SVG_SMOOTH_0, SVG_SMOOTH_1, SVG_SMOOTH_2] {
            smooth_level_icons.push(usvg::Tree::from_str(svg_str, &opt).unwrap());
        }

        let mut thickness_icons = Vec::new();
        for svg_str in [SVG_THICKNESS_1, SVG_THICKNESS_2, SVG_THICKNESS_3, SVG_THICKNESS_4] {
            thickness_icons.push(usvg::Tree::from_str(svg_str, &opt).unwrap());
        }

        let mut line_icons = Vec::new();
        for svg_str in [SVG_LINE, SVG_ARROW] {
            line_icons.push(usvg::Tree::from_str(svg_str, &opt).unwrap());
        }

        Toolbar {
            rect: Rect {
                x,
                y,
                w: width as u32,
                h: (buttons.len() as i32 * (button_size + padding) + padding + 20) as u32,
            },
            buttons,
            smooth_level_icons,
            thickness_icons,
            line_icons,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_laser_line_bounding_box() {
        let now = std::time::Instant::now();
        let shape = Shape::LaserLine {
            points: vec![
                (Point { x: 10.0, y: 10.0 }, now),
                (Point { x: 20.0, y: 20.0 }, now),
            ],
            _color: tiny_skia::Color::from_rgba8(255, 0, 0, 255),
            thickness: 2.0,
        };
        let bb = shape.bounding_box().unwrap();
        // min_x=10, min_y=10, max_x=20, max_y=20, pad = (2/2 + 2) + (1.5*2 + 1) = 7
        // So x = 10 - 7 = 3, y = 10 - 7 = 3
        // w = (20 + 7) - 3 = 24
        // h = (20 + 7) - 3 = 24
        assert_eq!(bb.x, 3);
        assert_eq!(bb.y, 3);
        assert_eq!(bb.w, 24);
        assert_eq!(bb.h, 24);
    }
}

