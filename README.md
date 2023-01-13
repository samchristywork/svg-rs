![Banner](https://s-christy.com/status-banner-service/svg-rs/banner-slim.svg)

## Overview

## Examples

## Features

- API for creating common shapes/primitives in SVG
- Support for transforms
- Support for paths and Beziér curves
- Function to write SVG data to file
- Optional style and transform for each element
- Tree-based storage for nested structures
- Test suite

## Usage

```svg
let mut svg = Svg::new(100.0, 100.0);

let circle = Circle::new(50.0, 50.0, 10.0, "", "fill:black");

svg.add_shape(circle);
```

See the `playground` test in `src/lib.rs` for further example usage.

## License

This work is licensed under the GNU General Public License version 3 (GPLv3).

[<img src="https://s-christy.com/status-banner-service/GPLv3_Logo.svg" width="150" />](https://www.gnu.org/licenses/gpl-3.0.en.html)
