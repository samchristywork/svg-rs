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
        let mut svg = Svg::new(50.0, 50.0);

        svg.set_height(100.0);

        assert_eq!(
            svg.to_string(),
            "<svg viewBox=\"0 0 50 100\" \
            xmlns=\"http://www.w3.org/2000/svg\"></svg>"
        );
    }
}
