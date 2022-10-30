# farver

[![css-colors](https://docs.rs/farver/badge.svg)](https://docs.rs/farver)

A Rust converter to transform CSS colors. 🎨

## Installation

Add the `farver` crate to your `Cargo.toml`'s list of dependencies:
```rust
[dependencies]
farver = "3.1.0"
```

## What is farver?

This crate allows you to create and manipulate colors using `Less` functions, and to be able to use a common color type if you need 
to convert to interact with multiple crates

The RGB color model is often useful when you'd like to represent a color using a certain amount of red, green, and blue.
```css
background-color: rgb(255, 99, 71); // tomato
```
However, it is also possible to represent the same color using the HSL color model, which specifies the hue, saturation, and luminosity of a color:
```css
background-color: hsl(9, 100%, 64%); // also tomato!
```

You can also use CSS preprocessors like [Less](http://lesscss.org) to manipulate these colors in interesting ways.
```css
$tomato: hsl(9, 100%, 64%); // equivalent to rgb(255, 99, 71)
$dark-tomato: darken($tomato, 20%); // hsl(9, 100%, 44%)
$desaturated-tomato: desaturate($tomato, 40%); // hsl(9, 60%, 64%)
```

This crate allows you to perform operations that map to Less' [color operations API](http://lesscss.org/functions/#color-operations). These operations can be applied on both RGB & HSL color models.

## Examples

Represent colors as a valid CSS string:
```rust
use farver::{Color, rgb, hsla};

let salmon = rgb(250, 128, 114);
let chartreuse = hsla(90, 100, 50, 1.0);

assert_eq!(salmon.to_css(), "rgb(250, 128, 114)");
assert_eq!(chartreuse.to_css(), "hsla(90, 100%, 50%, 1.00)");
```

Convert between different color model representations:
```rust
use farver::{Color, rgb, rgba, hsl, hsla};

let chartreuse = rgb(127, 255, 0);

assert_eq!(chartreuse.to_hsl(), hsl(90, 100, 50));
assert_eq!(chartreuse.to_hsla(), hsla(90, 100, 50, 1.0));
assert_eq!(chartreuse.to_rgba(), rgba(127, 255, 0, 1.0));
```

Manipulate single colors to create new color model representations:
```rust
use farver::{Color, hsl, percent};

let chartreuse = hsl(90, 100, 50);

assert_eq!(chartreuse.darken(percent(20)), hsl(90, 100, 30));
assert_eq!(chartreuse.desaturate(percent(20)), hsl(90, 80, 50));
assert_eq!(chartreuse.greyscale(), hsl(90, 0, 50));
```

Manipulate multiple colors to create new color model representations:
```rust
use farver::{Color, rgb, rgba, hsl, hsla, percent};

let chartreuse = hsl(90, 100, 50);
let red = rgba(100, 0, 0, 1.0);

assert_eq!(
    chartreuse.mix(red, percent(50)).to_css(),
    "hsla(67, 98%, 25%, 1.00)"
);
assert_eq!(chartreuse.tint(percent(50)).to_css(), "hsl(90, 100%, 75%)");
assert_eq!(chartreuse.shade(percent(50)).to_css(), "hsl(90, 98%, 25%)");
```

Check out the [documentation](https://docs.rs/farver) to learn more about what color operations are available to use!

## Helpful Links

The following links may be helpful while using this crate.

1. CSS3's [RGB color model](https://www.w3.org/TR/css-color-3/#rgb-color)
2. CSS3's [HSL color model](https://www.w3.org/TR/css-color-3/#hsl-color)
3. Less' [color operation functions](http://lesscss.org/functions/#color-operations)

## Contributing

### Installation

* `git clone https://github.com/nyxkrage/farver`
* `cd farver`
* `cargo build`

### Linting + plugins

Please use the below tools to ensure code consistency when contributing to this crate.
* [Rustfmt](https://github.com/rust-lang-nursery/rustfmt) for formatting code style

### Building + testing

* `cargo build` – Builds the crate
* `cargo test` – Runs the test suite

## License

This project is licensed under the [ISC License](LICENSE).
