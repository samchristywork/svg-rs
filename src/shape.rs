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
    #[must_use]
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
    #[must_use]
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

pub struct Line<'a> {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    style: &'a str,
}

impl Shape for Line<'_> {
    fn to_svg(&self) -> String {
        format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"{}\" />",
            self.x1, self.y1, self.x2, self.y2, self.style
        )
    }
}

impl<'a> Line<'a> {
    #[must_use]
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32, style: &'a str) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            style,
        }
    }
}

pub struct Bezier<'a> {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
    style: &'a str,
}

impl Shape for Bezier<'_> {
    fn to_svg(&self) -> String {
        format!(
            "<path d=\"M {} {} C {} {} \
            {} {} {} {}\" \
            style=\"{}\" />",
            self.x1, self.y1, self.x2, self.y2, self.x3, self.y3, self.x4, self.y4, self.style
        )
    }
}

impl<'a> Bezier<'a> {
    #[must_use]
    pub fn new(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
        style: &'a str,
    ) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            x4,
            y4,
            style,
        }
    }
}
