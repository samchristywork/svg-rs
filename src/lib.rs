pub mod path;
pub mod shape;
pub mod svg;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use path::Path;
    use shape::Circle;
    use shape::Rectangle;
    use svg::Svg;

    #[test]
    fn hello_world() {
        let svg = Svg::new(50.0, 50.0);

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 50 50\" \
            xmlns=\"http://www.w3.org/2000/svg\"></svg>"
        );
    }

    #[test]
    fn circle() {
        let mut svg = Svg::new(50.0, 50.0);

        let circle = Circle::new(50.0, 50.0, 40.0, "fill:black");

        svg.add_shape(circle);

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 50 50\" xmlns=\"http://www.w3.org/2000/svg\">\n\
            <circle cx=\"50\" cy=\"50\" r=\"40\" style=\"fill:black\" />\n\
            </svg>"
        );
    }

    #[test]
    fn rectangle() {
        let mut svg = Svg::new(50.0, 50.0);

        svg.add_shape(Rectangle::new(50.0, 50.0, 10.0, 10.0, "fill:black"));

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 50 50\" xmlns=\"http://www.w3.org/2000/svg\">\n\
            <rectangle x=\"50\" y=\"50\" width=\"10\" height=\"10\" style=\"fill:black\" />\n\
            </svg>"
        );
    }

    #[test]
    fn double_circle() {
        let mut svg = Svg::new(100.0, 100.0);

        svg.add_shape(Circle::new(50.0, 50.0, 40.0, "fill:black"));
        svg.add_shape(Circle::new(50.0, 50.0, 40.0, "fill:black"));

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\">\n\
            <circle cx=\"50\" cy=\"50\" r=\"40\" style=\"fill:black\" />\n\
            <circle cx=\"50\" cy=\"50\" r=\"40\" style=\"fill:black\" />\n\
            </svg>"
        );
    }

    #[test]
    fn playground() {
        let mut svg = Svg::new(200.0, 200.0);

        let mut path = Path::new(150.0, 0.0);

        path.line_to(75.0, 200.0);

        path.line_to(225.0, 200.0);

        svg.add_element(format!("<path d=\"{}\" />", path.to_string()).as_str());

        //println!("\n\n{}\n\n", svg);

        //svg.to_file("out.svg").unwrap();

        assert_eq!(1, 1);
    }
}
