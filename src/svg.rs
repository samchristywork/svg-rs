use std::fmt;

pub struct Svg {}

impl Svg {
    pub fn new() -> Self {
        Self {}
    }
}

impl fmt::Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "hi")
    }
}
