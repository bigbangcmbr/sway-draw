use resvg::usvg;

const SVG_FREEHAND: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 19l7-7 3 3-7 7-3-3z"/><path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z"/><path d="M2 2l8 8"/><path d="M2 22l5-5"/></svg>"#;
const SVG_RECTANGLE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/></svg>"#;
const SVG_ARROW: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>"#;

fn get_tool_svg(tool: Tool) -> &'static str {
    match tool {
        Tool::Freehand => SVG_FREEHAND,
        Tool::Rectangle => SVG_RECTANGLE,
        Tool::Arrow => SVG_ARROW,
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
    Arrow,
    Freehand,
}

#[derive(Clone, Debug)]
pub enum Shape {
    Freehand {
        points: Vec<Point>,
        color: tiny_skia::Color,
        thickness: f32,
    },
    Rectangle {
        start: Point,
        end: Point,
        color: tiny_skia::Color,
        thickness: f32,
    },
    Arrow {
        start: Point,
        end: Point,
        color: tiny_skia::Color,
        thickness: f32,
    },
}

impl Shape {
    pub fn bounding_box(&self) -> Option<Rect> {
        let (mut min_x, mut min_y, mut max_x, mut max_y, thickness) = match self {
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
            | Shape::Arrow {
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

        // For Arrow, pad a bit more to account for arrowhead which can extend slightly beyond bounds
        let pad = match self {
            Shape::Arrow { thickness, .. } => pad + (*thickness * 3.0),
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
}

impl Toolbar {
    pub fn new(_screen_width: u32, screen_height: u32) -> Self {
        let width = 60;
        let button_size = 40;
        let padding = 10;
        let x = 20; // Positioned 20px from left
        let y = (screen_height as i32 - (4 * (button_size + padding))) / 2; // Centered vertically

        let mut buttons = Vec::new();
        let tools = [Tool::Freehand, Tool::Rectangle, Tool::Arrow];

        let mut opt = usvg::Options::default();
        opt.font_family = "sans-serif".to_string();

        for (i, tool) in tools.iter().enumerate() {
            let svg_str = get_tool_svg(*tool);
            let svg_tree = usvg::Tree::from_str(svg_str, &opt).unwrap();

            buttons.push(Button {
                rect: Rect {
                    x: x + (width - button_size) / 2,
                    y: y + padding + (i as i32 * (button_size + padding)),
                    w: button_size as u32,
                    h: button_size as u32,
                },
                icon: *tool,
                svg_tree,
            });
        }

        Toolbar {
            rect: Rect {
                x,
                y,
                w: width as u32,
                h: (buttons.len() as i32 * (button_size + padding) + padding) as u32,
            },
            buttons,
        }
    }
}
