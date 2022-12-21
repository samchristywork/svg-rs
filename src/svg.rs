use std::fmt;
use trees::tr;

pub struct Svg {
    width: f32,
    height: f32,
    nodes: trees::Tree<String>,
}

impl Svg {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            nodes: tr(String::new()),
        }
    }

    fn svg_tag(&self) -> String {
        format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\"></svg>",
            self.width, self.height
        )
    }

    pub fn add_element(&mut self, s: &str) {
        self.nodes = self.nodes.clone() / tr(String::new() + s);
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
