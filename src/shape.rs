pub trait Shape {
    fn to_svg(&self) -> String;
}

pub struct Circle<'a> {
    x: f32,
    y: f32,
    r: f32,
    style: &'a str,
}

impl Shape for Circle<'_> {
    fn to_svg(&self) -> String {
        format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" style=\"{}\" />",
            self.x, self.y, self.r, self.style
        )
    }
}

impl<'a> Circle<'a> {
    pub fn new(x: f32, y: f32, r: f32, style: &'a str) -> Self {
        Self { x, y, r, style }
    }
}

pub struct Rectangle<'a> {
    x: f32,
    y: f32,
    rx: f32,
    ry: f32,
    width: f32,
    height: f32,
    style: &'a str,
}

impl Shape for Rectangle<'_> {
    fn to_svg(&self) -> String {
        format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" \
            rx=\"{}\" ry=\"{}\" \
            style=\"{}\" />",
            self.x, self.y, self.width, self.height, self.rx, self.ry, self.style
        )
    }
}

impl<'a> Rectangle<'a> {
    pub fn new(x: f32, y: f32, width: f32, height: f32, style: &'a str) -> Self {
        Self {
            x,
            y,
            rx: 0.0,
            ry: 0.0,
            width,
            height,
            style,
        }
    }

    pub fn rounded(&mut self, rx: f32, ry: f32) {
        self.rx = rx;
        self.ry = ry;
    }
}
