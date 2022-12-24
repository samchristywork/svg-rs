use crate::shape::Shape;

pub struct Text<'a> {
    x: f32,
    y: f32,
    size: f32,
    style: &'a str,
    text: &'a str,
}

impl Shape for Text<'_> {
    fn to_svg(&self) -> String {
        format!(
            "<text x=\"{}\" y=\"{}\" font-size=\"{}\" style=\"{}\">{}</text>",
            self.x, self.y, self.size, self.style, self.text
        )
    }
}

impl<'a> Text<'a> {
    #[must_use]
    pub fn new(x: f32, y: f32, size: f32, style: &'a str, text: &'a str) -> Self {
        Self {
            x,
            y,
            size,
            style,
            text,
        }
    }
}
