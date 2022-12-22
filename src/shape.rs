pub trait Shape {
    fn hello(&self);
    fn to_svg(&self) -> String;
}

pub struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

impl Shape for Circle {
    fn hello(&self) {
        println!("I am a circle");
    }

    fn to_svg(&self) -> String {
        format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"black\" />",
            self.x, self.y, self.r
        )
    }
}

impl Circle {
    pub fn new(x: f32, y: f32, r: f32) -> Self {
        Self { x, y, r }
    }
}
