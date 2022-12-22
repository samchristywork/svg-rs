use crate::shape;
use std::fmt;
use std::fs::File;
use std::io::Write;
use trees;
use trees::tr;

fn tree_to_string(node: &trees::Node<Node>) -> String {
    if node.has_no_child() {
        node.data().begin.clone() + node.data().end.clone().as_str()
    } else {
        format!(
            "{}\n{}{}",
            node.data().begin,
            node.iter()
                .fold(String::new(), |s, c| s + &tree_to_string(c) + &"\n"),
            node.data().end,
        )
    }
}

#[derive(Clone)]
pub struct Node {
    pub begin: String,
    pub end: String,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.begin, self.end)
    }
}

pub struct Svg {
    width: f32,
    height: f32,
    pub nodes: trees::Tree<Node>,
}

impl Svg {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            nodes: tr(Node {
                begin: String::new()
                    + format!(
                        "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
                        width, height
                    )
                    .as_str(),
                end: String::new() + "</svg>",
            }),
        }
    }

    pub fn add_element(&mut self, s: &str) {
        self.nodes = self.nodes.clone()
            / tr(Node {
                begin: s.to_string(),
                end: String::new(),
            });
    }

    pub fn add_shape<S: shape::Shape>(&mut self, s: S) {
        self.nodes = self.nodes.clone()
            / tr(Node {
                begin: s.to_svg(),
                end: String::new(),
            });
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    pub fn to_file(&self, path: &str) -> std::io::Result<()> {
        let mut output = File::create(path)?;
        write!(output, "{}", self.to_string())
    }
}

impl fmt::Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", tree_to_string(&self.nodes))
    }
}
