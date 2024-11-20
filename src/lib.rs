pub mod path;
pub mod shape;
pub mod svg;
pub mod text;

#[cfg(test)]
mod tests {
    use path::Path;
    use shape::Bezier;
    use shape::Circle;
    use shape::Line;
    use shape::Rectangle;
    use std::fs::File;
    use std::io::{self, Read};
    use super::*;
    use svg::Svg;
    use text::Text;

    #[test]
    fn hello_world() {
        let svg = Svg::new(50.0, 50.0);

        svg.to_file("target/hello_world.svg").unwrap();

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 50 50\" \
            xmlns=\"http://www.w3.org/2000/svg\"></svg>"
        );
    }

    #[test]
    fn circle() {
        let mut svg = Svg::new(50.0, 50.0);

        let circle = Circle::new(50.0, 50.0, 40.0, "", "fill:black");

        svg.add_shape(circle);

        svg.to_file("target/circle.svg").unwrap();

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 50 50\" xmlns=\"http://www.w3.org/2000/svg\">\n\
            <circle cx=\"50\" cy=\"50\" r=\"40\" transform=\"\" style=\"fill:black\" />\n\
            </svg>"
        );
    }

    #[test]
    fn rectangle() {
        let mut svg = Svg::new(50.0, 50.0);

        svg.add_shape(Rectangle::new(30.0, 30.0, 10.0, 10.0, "", "fill:black"));

        svg.to_file("target/rectangle.svg").unwrap();

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 50 50\" xmlns=\"http://www.w3.org/2000/svg\">\n\
            <rect x=\"30\" y=\"30\" width=\"10\" height=\"10\" \
            rx=\"0\" ry=\"0\" transform=\"\" style=\"fill:black\" />\n\
            </svg>"
        );
    }

    #[test]
    fn rectangle_rounded() {
        let mut svg = Svg::new(50.0, 50.0);

        let mut rect = Rectangle::new(30.0, 30.0, 10.0, 10.0, "", "fill:black");

        rect.rounded(2.0, 2.0);

        svg.add_shape(rect);

        svg.to_file("target/rectangle_rounded.svg").unwrap();

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 50 50\" xmlns=\"http://www.w3.org/2000/svg\">\n\
            <rect x=\"30\" y=\"30\" width=\"10\" height=\"10\" \
            rx=\"2\" ry=\"2\" transform=\"\" style=\"fill:black\" />\n\
            </svg>"
        );
    }

    #[test]
    fn line() {
        let mut svg = Svg::new(50.0, 50.0);

        let line = Line::new(10.0, 10.0, 20.0, 20.0, "", "stroke:black");

        svg.add_shape(line);

        svg.to_file("target/line.svg").unwrap();

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 50 50\" xmlns=\"http://www.w3.org/2000/svg\">\n\
            <line x1=\"10\" y1=\"10\" x2=\"20\" y2=\"20\" \
            transform=\"\" style=\"stroke:black\" />\n\
            </svg>"
        );
    }

    #[test]
    fn bezier() {
        let mut svg = Svg::new(50.0, 50.0);

        let bezier = Bezier::new(
            10.0,
            10.0,
            20.0,
            10.0,
            10.0,
            20.0,
            20.0,
            20.0,
            "",
            "stroke:black; fill:none; stroke-width:.1",
        );

        svg.add_shape(bezier);

        svg.to_file("target/bezier.svg").unwrap();

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 50 50\" xmlns=\"http://www.w3.org/2000/svg\">\n\
            <path d=\"M 10 10 C 20 10 10 20 20 20\" \
            transform=\"\" style=\"stroke:black; fill:none; \
            stroke-width:.1\" />\n</svg>",
        );
    }

    #[test]
    fn text() {
        let mut svg = Svg::new(50.0, 50.0);

        svg.add_shape(Text::new(
            10.0,
            10.0,
            2.0,
            "",
            "fill:black",
            "Hello, World!",
        ));

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 50 50\" xmlns=\"http://www.w3.org/2000/svg\">\n\
            <text x=\"10\" y=\"10\" font-size=\"2\" \
            transform=\"\" style=\"fill:black\">Hello, World!</text>\n\
            </svg>",
        );
    }

    #[test]
    fn double_circle() {
        let mut svg = Svg::new(100.0, 100.0);

        svg.add_shape(Circle::new(50.0, 50.0, 40.0, "", "fill:black"));
        svg.add_shape(Circle::new(50.0, 50.0, 40.0, "", "fill:black"));

        svg.to_file("target/double_circle.svg").unwrap();

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\">\n\
            <circle cx=\"50\" cy=\"50\" r=\"40\" transform=\"\" style=\"fill:black\" />\n\
            <circle cx=\"50\" cy=\"50\" r=\"40\" transform=\"\" style=\"fill:black\" />\n\
            </svg>"
        );
    }

    #[test]
    fn transform() {
        let mut svg = Svg::new(100.0, 100.0);

        svg.add_shape(Rectangle::new(30.0, 30.0, 10.0, 10.0, "", "fill:black"));

        svg.add_shape(Rectangle::new(
            30.0,
            30.0,
            10.0,
            10.0,
            "rotate(40 20 40)",
            "fill:red",
        ));

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\">\n\
            <rect x=\"30\" y=\"30\" width=\"10\" height=\"10\" rx=\"0\" ry=\"0\" \
            transform=\"\" style=\"fill:black\" />\n\
            <rect x=\"30\" y=\"30\" width=\"10\" height=\"10\" rx=\"0\" ry=\"0\" \
            transform=\"rotate(40 20 40)\" style=\"fill:red\" />\n\
            </svg>"
        );
    }

    fn read_file_to_string(filename: &str) -> io::Result<String> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    #[test]
    fn misc() {
        let mut svg = Svg::new(400.0, 260.0);

        let mut path = Path::new(50.0, 0.0);
        path.line_to(0.0, 100.0);
        path.line_to(100.0, 100.0);
        svg.add_element(format!("<path d=\"{}\" />", path).as_str());

        let circle = Circle::new(150.0, 50.0, 40.0, "", "fill:red");
        svg.add_shape(circle);

        svg.add_shape(Rectangle::new(230.0, 30.0, 50.0, 50.0, "", "fill:orange"));

        let mut rect = Rectangle::new(330.0, 30.0, 50.0, 50.0, "", "fill:yellow");
        rect.rounded(8.0, 8.0);
        svg.add_shape(rect);

        let line = Line::new(10.0, 110.0, 80.0, 180.0, "", "stroke:green");
        svg.add_shape(line);

        svg.add_shape(Rectangle::new(130.0, 130.0, 50.0, 50.0, "", "fill:blue"));

        svg.add_shape(Rectangle::new(
            30.0,
            30.0,
            10.0,
            10.0,
            "translate(200 100) rotate(40 20 40)",
            "fill:purple",
        ));

        svg.add_shape(Text::new(
            310.0,
            110.0,
            10.0,
            "",
            "fill:violet",
            "Hello, World!",
        ));

        let bezier = Bezier::new(
            40.0,
            200.0,
            200.0,
            200.0,
            140.0,
            240.0,
            300.0,
            240.0,
            "",
            "stroke:black; fill:none; stroke-width:1",
        );
        svg.add_shape(bezier);

        svg.to_file("target/shapes.svg").unwrap();

        let contents=read_file_to_string("sample/shapes.svg").unwrap();
        assert_eq!(contents, svg.to_string());
    }
}
