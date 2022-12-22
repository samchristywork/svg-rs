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
    width: f32,
    height: f32,
    style: &'a str,
}

impl Shape for Rectangle<'_> {
    fn to_svg(&self) -> String {
        format!(
            "<rectangle x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" style=\"{}\" />",
            self.x, self.y, self.width, self.height, self.style
        )
    }
}

impl<'a> Rectangle<'a> {
    pub fn new(x: f32, y: f32, width: f32, height: f32, style: &'a str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            style,
        }
    }
}
