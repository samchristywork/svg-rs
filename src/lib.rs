use trees::Node;

pub mod svg;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn tree_to_string<T>(node: &Node<T>) -> String
where
    T: std::fmt::Display,
{
    if node.has_no_child() {
        node.data().to_string()
    } else {
        format!(
            "{}( {})",
            node.data(),
            node.iter()
                .fold(String::new(), |s, c| s + &tree_to_string(c) + &" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svg::Svg;
    use trees::tr;

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

    #[test]
    fn asdf() {
        let mut scattered_tree = tr(0);
        scattered_tree = scattered_tree / tr(1);
        scattered_tree = scattered_tree / tr(2);
        scattered_tree = scattered_tree / tr(3);

        let x = tree_to_string(&scattered_tree);
        println!("{}", x);

        assert_eq!(scattered_tree, tr(0) / tr(1) / tr(2) / tr(3));
    }
}
