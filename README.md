![Banner](https://s-christy.com/sbs/status-banner.svg?icon=maps/category&hue=100&title=SVG&description=A%20Rust%20library%20for%20creating%20SVG%20images)

## Overview

This is a Rust library for creating SVG files. The library supports many of the
basic shapes, text, and attributes like style and transform. This library is not
feature complete.

## Examples

<p align="center">
  <img src="./sample/shapes.svg" width=500 />
</p>

The above was created with this code:

```rust
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
```

## Features

- API for creating common shapes/primitives in SVG
- Support for transforms
- Support for paths and Beziér curves
- Function to write SVG data to file
- Optional style and transform for each element
- Tree-based storage for nested structures
- Test suite

## Usage

```rust
// Create a new Svg object
let mut svg = Svg::new(100.0, 100.0);

// Create shapes
let circle = Circle::new(50.0, 50.0, 10.0, "", "fill:black");

// Add shapes to the object
svg.add_shape(circle);

// Write the data to a file
svg.to_file("some/file/path.svg").unwrap();
```

See the `misc` test in `src/lib.rs` for further example usage.

## License

This work is licensed under the GNU General Public License version 3 (GPLv3).

[<img src="https://s-christy.com/status-banner-service/GPLv3_Logo.svg" width="150" />](https://www.gnu.org/licenses/gpl-3.0.en.html)
