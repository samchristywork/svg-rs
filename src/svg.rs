use std::fmt;

pub struct Svg {
    width: f32,
    height: f32,
}

impl Svg {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    fn svg_tag(&self) -> String {
        format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\"></svg>",
            self.width, self.height
        )
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }
}

impl fmt::Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.svg_tag())
    }
}
