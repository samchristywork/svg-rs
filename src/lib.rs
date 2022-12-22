pub mod svg;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
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

        svg.add_element("<circle cx=\"50\" cy=\"50\" r=\"40\" stroke=\"black\" />");

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 50 50\" xmlns=\"http://www.w3.org/2000/svg\">\n\
            <circle cx=\"50\" cy=\"50\" r=\"40\" stroke=\"black\" />\n\
            </svg>"
        );
    }

    #[test]
    fn double_circle() {
        let mut svg = Svg::new(100.0, 100.0);

        svg.add_element("<circle cx=\"50\" cy=\"50\" r=\"40\" stroke=\"black\" />");
        svg.add_element("<circle cx=\"50\" cy=\"50\" r=\"40\" stroke=\"black\" />");

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\">\n\
            <circle cx=\"50\" cy=\"50\" r=\"40\" stroke=\"black\" />\n\
            <circle cx=\"50\" cy=\"50\" r=\"40\" stroke=\"black\" />\n\
            </svg>"
        );
    }

    #[test]
    fn asdf() {
        let mut scattered_tree = tr(0);
        scattered_tree = scattered_tree / tr(1);
        scattered_tree = scattered_tree / tr(2);
        scattered_tree = scattered_tree / tr(3);

        assert_eq!(scattered_tree, tr(0) / tr(1) / tr(2) / tr(3));
    }
}
