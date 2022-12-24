use crate::shape::Shape;

pub struct Text<'a> {
    x: f32,
    y: f32,
    size: f32,
    style: &'a str,
    transform: &'a str,
    text: &'a str,
}

impl Shape for Text<'_> {
    fn to_svg(&self) -> String {
        format!(
            "<text x=\"{}\" y=\"{}\" font-size=\"{}\" \
            transform=\"{}\" style=\"{}\">{}</text>",
            self.x, self.y, self.size, self.transform, self.style, self.text
        )
    }
}

impl<'a> Text<'a> {
    #[must_use]
    pub fn new(
        x: f32,
        y: f32,
        size: f32,
        transform: &'a str,
        style: &'a str,
        text: &'a str,
    ) -> Self {
        Self {
            x,
            y,
            size,
            style,
            transform,
            text,
        }
    }
}
