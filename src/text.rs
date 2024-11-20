use crate::shape::Shape;

pub struct Text<'a> {
    pos: (f32, f32),
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
            self.pos.0, self.pos.1, self.size, self.transform, self.style, self.text
        )
    }
}

impl<'a> Text<'a> {
    #[must_use]
    pub fn new(
        pos: (f32, f32),
        size: f32,
        transform: &'a str,
        style: &'a str,
        text: &'a str,
    ) -> Self {
        Self {
            pos,
            size,
            style,
            transform,
            text,
        }
    }
}
