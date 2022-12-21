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
        let svg = Svg::new();
        assert_eq!(svg.to_string(), "hi");
    }
}
