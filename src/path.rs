use std::fmt;

pub struct Path {
    points: Vec<(f32, f32)>,
}

impl Path {
    #[must_use]
    pub fn new(pos: (f32, f32)) -> Self {
        Self { points: vec![pos] }
    }

    pub fn line_to(&mut self, pos: (f32, f32)) {
        self.points.push(pos);
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();

        let mut first = true;

        for point in &self.points {
            if first {
                ret += "M ";
                first = false;
            } else {
                ret += "L ";
            }
            ret += point.0.to_string().as_str();
            ret += " ";
            ret += point.1.to_string().as_str();
            ret += " ";
        }

        ret += "Z";
        write!(f, "{}", ret)
    }
}
