pub trait Shape {
    fn to_svg(&self) -> String;
}

pub struct Circle<'a> {
    pos: (f32, f32),
    r: f32,
    style: &'a str,
    transform: &'a str,
}

impl Shape for Circle<'_> {
    fn to_svg(&self) -> String {
        format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" \
            transform=\"{}\" style=\"{}\" />",
            self.pos.0, self.pos.1, self.r, self.transform, self.style
        )
    }
}

impl<'a> Circle<'a> {
    #[must_use]
    pub fn new(pos: (f32, f32), r: f32, transform: &'a str, style: &'a str) -> Self {
        Self {
            pos,
            r,
            style,
            transform,
        }
    }
}

pub struct Rectangle<'a> {
    pos: (f32, f32),
    rx: f32,
    ry: f32,
    width: f32,
    height: f32,
    style: &'a str,
    transform: &'a str,
}

impl Shape for Rectangle<'_> {
    fn to_svg(&self) -> String {
        format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" \
            rx=\"{}\" ry=\"{}\" \
            transform=\"{}\" style=\"{}\" />",
            self.pos.0,
            self.pos.1,
            self.width,
            self.height,
            self.rx,
            self.ry,
            self.transform,
            self.style
        )
    }
}

impl<'a> Rectangle<'a> {
    #[must_use]
    pub fn new(
        pos: (f32, f32),
        width: f32,
        height: f32,
        transform: &'a str,
        style: &'a str,
    ) -> Self {
        Self {
            pos,
            rx: 0.0,
            ry: 0.0,
            width,
            height,
            style,
            transform,
        }
    }

    pub fn rounded(&mut self, rx: f32, ry: f32) {
        self.rx = rx;
        self.ry = ry;
    }
}

pub struct Line<'a> {
    a: (f32, f32),
    b: (f32, f32),
    style: &'a str,
    transform: &'a str,
}

impl Shape for Line<'_> {
    fn to_svg(&self) -> String {
        format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" \
            transform=\"{}\" style=\"{}\" />",
            self.a.0, self.a.1, self.b.0, self.b.1, self.transform, self.style
        )
    }
}

impl<'a> Line<'a> {
    #[must_use]
    pub fn new(a: (f32, f32), b: (f32, f32), transform: &'a str, style: &'a str) -> Self {
        Self {
            a,
            b,
            style,
            transform,
        }
    }
}

pub struct Bezier<'a> {
    a: (f32, f32),
    b: (f32, f32),
    c: (f32, f32),
    d: (f32, f32),
    style: &'a str,
    transform: &'a str,
}

impl Shape for Bezier<'_> {
    fn to_svg(&self) -> String {
        format!(
            "<path d=\"M {} {} C {} {} \
            {} {} {} {}\" \
            transform=\"{}\" style=\"{}\" />",
            self.a.0,
            self.a.1,
            self.b.0,
            self.b.1,
            self.c.0,
            self.c.1,
            self.d.0,
            self.d.1,
            self.transform,
            self.style
        )
    }
}

impl<'a> Bezier<'a> {
    #[must_use]
    pub fn new(
        a: (f32, f32),
        b: (f32, f32),
        c: (f32, f32),
        d: (f32, f32),
        transform: &'a str,
        style: &'a str,
    ) -> Self {
        Self {
            a,
            b,
            c,
            d,
            style,
            transform,
        }
    }
}

pub struct Polygon<'a> {
    points: &'a str,
    style: &'a str,
    transform: &'a str,
}

impl Shape for Polygon<'_> {
    fn to_svg(&self) -> String {
        format!(
            "<polygon points=\"{}\" \
            transform=\"{}\" style=\"{}\" />",
            self.points, self.transform, self.style
        )
    }
}

impl<'a> Polygon<'a> {
    #[must_use]
    pub fn new(points: &'a str, transform: &'a str, style: &'a str) -> Self {
        Self {
            points,
            style,
            transform,
        }
    }
}
