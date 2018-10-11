use std::fmt;

pub mod angle;
use angle::Angle;


/// A trait that can be used for converting between different color models
/// and performing various transformations on them.
pub trait Color {
    /// Converts `self` to its CSS string format.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let salmon = RGB { r: 250, g: 128, b: 114 };
    /// let opaque_salmon = RGBA { r: 250, g: 128, b: 114, a: 128 };
    ///
    /// assert_eq!(salmon.to_css(), "rgb(250, 128, 114)");
    /// assert_eq!(opaque_salmon.to_css(), "rgba(250, 128, 114, 0.50)");
    /// ```
    fn to_css(self) -> String;

    /// Converts `self` into its RGB representation.
    /// When converting from a color model that supports an alpha channel
    /// (e.g. RGBA), the alpha value will not be preserved.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let opaque_tomato = RGBA { r: 255, g: 99, b: 71, a: 128 };
    ///
    /// assert_eq!(opaque_tomato.to_rgb(), RGB { r: 255, g: 99, b: 71 });
    /// ```
    fn to_rgb(self) -> RGB;

    /// Converts `self` into its RGBA representation.
    /// When converting from a color model that does not supports an alpha channel
    /// (e.g. RGB), it will be treated as fully opaque.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let tomato = RGB { r: 255, g: 99, b: 71 };
    ///
    /// assert_eq!(tomato.to_rgba(), RGBA { r: 255, g: 99, b: 71, a: 255 });
    /// ```
    fn to_rgba(self) -> RGBA;

    /// Converts `self` into its HSL representation.
    /// When converting from a color model that supports an alpha channel
    /// (e.g. RGBA), the alpha value will not be preserved.
    ///
    /// # TODO: Examples
    fn to_hsl(self) -> HSL;

    /// Converts `self` into its HSLA representation.
    /// When converting from a color model that does not supports an alpha channel
    /// (e.g. RGB), it will be treated as fully opaque.
    ///
    /// # TODO: Examples
    fn to_hsla(self) -> HSLA;
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much red, green, and blue should be added to create a color.
///
/// Valid values for r, g, and b must fall between `0-255`.
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#rgb-color).
pub struct RGB {
    // red
    pub r: u8,

    // green
    pub g: u8,

    // blue
    pub b: u8,
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl RGB {
    /// Transforms numerical values into an RGB struct.
    ///
    /// # Example
    /// ```
    /// use css_colors::RGB;
    ///
    /// let salmon = RGB::new(250, 128, 114);
    ///
    /// assert_eq!(salmon, RGB { r: 250, g: 128, b: 114 });
    /// ```
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r, g, b }
    }
}

impl Color for RGB {
    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        self
    }

    fn to_rgba(self) -> RGBA {
        RGBA::new(self.r, self.g, self.b, 255)
    }

    fn to_hsl(self) -> HSL {
        let RGB { r, g, b } = self;

        if r == g && g == b {
            return HSL::new(
                Angle::new(0),
                0,
                // TODO Ratio::from_u8(r)
                // or just `r` (if `r` is a Ratio already)
                (100.0 * (r as f32) / 255.0).round() as u8,
            );
        }

        // To determine luminosity: `(min(RGB) + max(RGB)) / 2`
        // 1. convert the RGB values into a range from `0-1`

        // let r = self.r.to_f32() if r is a Ratio
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        // let max = vec![r, g, b].iter().max().to_f32()
        let max = if r > g && r > b {
            r
        } else if g > b {
            g
        } else {
            b
        };

        let min = if r < g && r < b {
            r
        } else if g < b {
            g
        } else {
            b
        };

        // 2. find the max and min value of the converted values and sum them together and divide by 2
        let luminosity = (max + min) / 2.0;

        // If Luminance is smaller then 0.5, then Saturation = (max-min)/(max+min)
        // If Luminance is bigger then 0.5. then Saturation = ( max-min)/(2.0-max-min)
        let saturation = if luminosity < 0.5 {
            (max - min) / (max + min)
        } else {
            (max - min) / (2.0 - max - min)
        };

        // If Red is max, then Hue = (G-B)/(max-min)
        // If Green is max, then Hue = 2.0 + (B-R)/(max-min)
        // If Blue is max, then Hue = 4.0 + (R-G)/(max-min)

        let mut hue = if max == r {
            (g - b) / (max - min)
        } else if max == g {
            2.0 + (b - r) / (max - min)
        } else {
            4.0 + (r - g) / (max - min)
        };

        // The Hue value you get needs to be multiplied by 60 to convert it to degrees on the color circle
        // If Hue becomes negative you need to add 360 to, because a circle has 360 degrees.
        hue *= 60.0;

        assert!(hue >= 0.0, "oops, forgot to handle negative");

        HSL::new(
            Angle::new(hue.round() as u16),
            (100.0 * saturation).round() as u8, // Ratio::from_f32(saturation)
            (100.0 * luminosity).round() as u8,
        )
    }

    fn to_hsla(self) -> HSLA {
        let HSL { h, s, l } = self.to_hsl();
        HSLA::new(h, s, l, 255)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much red, green, and blue should be added to create a color.
/// Also handles alpha specifications.
///
/// Valid values for r, g, b, and a must fall between `0-255`.
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#rgba-color).
pub struct RGBA {
    // red
    pub r: u8,

    // green
    pub g: u8,

    // blue
    pub b: u8,

    // alpha
    pub a: u8,
}

impl fmt::Display for RGBA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rgba({}, {}, {}, {:.02})",
            self.r,
            self.g,
            self.b,
            self.a as f32 / 255.0
        )
    }
}

impl RGBA {
    /// Transforms numerical values into an RGBA struct.
    ///
    /// # Example
    /// ```
    /// use css_colors::RGBA;
    ///
    /// let light_salmon = RGBA::new(250, 128, 114, 128);
    ///
    /// assert_eq!(light_salmon, RGBA { r: 250, g: 128, b: 114, a: 128 });
    /// ```
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
        RGBA { r, g, b, a }
    }
}

impl Color for RGBA {
    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        RGB::new(self.r, self.g, self.b)
    }

    fn to_rgba(self) -> RGBA {
        self
    }

    fn to_hsl(self) -> HSL {
        HSL::new(Angle::new(0), 0, 0)
    }

    fn to_hsla(self) -> HSLA {
        HSLA::new(Angle::new(0), 0, 0, 0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much hue, saturation, and luminosity should be added to create a color.
/// The hue is a degree on the color wheel; 0 (or 360) is red, 120 is green, 240 is blue.
/// A valid value for `h` must range between `0-360`.
/// The saturation ranges between `0-100`, where `0` is completely desaturated, and `100` is full saturation.
/// The luminosity ranges between `0-100`, where `0` is no light (black), and `100` is full light (white).
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#hsl-color).
pub struct HSL {
    // hue
    pub h: Angle,

    // saturation
    pub s: u8,

    // luminosity
    pub l: u8,
}

impl fmt::Display for HSL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hsl({}, {}%, {}%)", self.h.degrees(), self.s, self.l)
    }
}

impl HSL {
    /// Transforms numerical values into a HSL struct.
    ///
    /// # Example
    /// ```
    /// use {css_colors::HSL, css_colors::angle::Angle as Angle};
    ///
    /// let salmon = HSL::new(Angle::new(6), 93, 71);
    ///
    /// assert_eq!(salmon, HSL { h: Angle::new(6), s: 93, l: 71 });
    /// ```
    pub fn new(h: Angle, s: u8, l: u8) -> HSL {
        HSL { h, s, l }
    }
}

impl Color for HSL {
    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        // FIXME: create impl, add tests for this
        RGB::new(0, 0, 0)
    }

    fn to_rgba(self) -> RGBA {
        // FIXME: create impl, add tests for this
        RGBA::new(0, 0, 0, 0)
    }

    fn to_hsl(self) -> HSL {
        self
    }

    fn to_hsla(self) -> HSLA {
        HSLA::new(self.h, self.s, self.l, 255)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much hue, saturation, and luminosity should be added to create a color.
/// Also handles alpha specifications.
///
/// A valid value for `h` must range between `0-360`.
/// The saturation ranges between `0-100`, where `0` is completely desaturated, and `100` is full saturation.
/// The luminosity ranges between `0-100`, where `0` is no light (black), and `100` is full light (white).
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#hsla-color).
pub struct HSLA {
    // hue
    pub h: Angle,

    // saturation
    pub s: u8,

    // luminosity
    pub l: u8,

    // alpha
    pub a: u8,
}

impl fmt::Display for HSLA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "hsla({}, {}%, {}%, {:.02})",
            self.h,
            self.s,
            self.l,
            self.a as f32 / 255.0
        )
    }
}

impl HSLA {
    /// Transforms numerical values into a HSL struct.
    ///
    /// # Example
    /// ```
    /// use {css_colors::HSLA, css_colors::angle::Angle as Angle};
    /// let light_salmon = HSLA::new(Angle::new(6), 93, 71, 128);
    ///
    /// assert_eq!(light_salmon, HSLA { h: Angle::new(6), s: 93, l: 71, a: 128 });
    /// ```
    pub fn new(h: Angle, s: u8, l: u8, a: u8) -> HSLA {
        HSLA { h, s, l, a }
    }
}

impl Color for HSLA {
    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        // FIXME: create impl, add tests for this
        RGB::new(0, 0, 0)
    }

    fn to_rgba(self) -> RGBA {
        // FIXME: create impl, add tests for this
        RGBA::new(0, 0, 0, 0)
    }

    fn to_hsl(self) -> HSL {
        HSL::new(self.h, self.s, self.l)
    }

    fn to_hsla(self) -> HSLA {
        self
    }
}


#[cfg(test)]
mod css_color_tests {
    use {Angle, Color, HSL, HSLA, RGB, RGBA};

    #[test]
    fn can_create_color_structs() {
        assert_eq!(RGB::new(5, 10, 15), RGB { r: 5, g: 10, b: 15 });
        assert_eq!(
            RGBA::new(5, 10, 15, 255),
            RGBA {
                r: 5,
                g: 10,
                b: 15,
                a: 255
            }
        );
        assert_eq!(
            HSL::new(Angle::new(6), 93, 71),
            HSL {
                h: Angle::new(6),
                s: 93,
                l: 71
            }
        );
        assert_eq!(
            HSLA::new(Angle::new(6), 93, 71, 255),
            HSLA {
                h: Angle::new(6),
                s: 93,
                l: 71,
                a: 255
            }
        );
    }

    #[test]
    fn can_convert_to_rgb_notations() {
        let rgb_color = RGB { r: 5, g: 10, b: 15 };
        let rgba_color = RGBA {
            r: 5,
            g: 10,
            b: 15,
            a: 255,
        };
        let hsl_color = HSL {
            h: Angle::new(6),
            s: 93,
            l: 71,
        };
        let hsla_color = HSLA {
            h: Angle::new(6),
            s: 93,
            l: 71,
            a: 255,
        };

        assert_eq!(
            rgb_color.to_rgba(),
            RGBA {
                r: 5,
                g: 10,
                b: 15,
                a: 255,
            }
        );
        assert_eq!(rgba_color.to_rgb(), RGB { r: 5, g: 10, b: 15 });

        // FIXME: update these tests once HSL <-> RBG impl exists
        assert_eq!(hsl_color.to_rgb(), RGB { r: 0, g: 0, b: 0 });
        assert_eq!(
            hsl_color.to_rgba(),
            RGBA {
                r: 0,
                g: 0,
                b: 0,
                a: 0
            }
        );
        assert_eq!(hsla_color.to_rgb(), RGB { r: 0, g: 0, b: 0 });
        assert_eq!(
            hsla_color.to_rgba(),
            RGBA {
                r: 0,
                g: 0,
                b: 0,
                a: 0
            }
        );
    }

    #[test]
    fn can_convert_to_hsl_notations() {
        let rgb_rust = RGB {
            r: 172,
            g: 95,
            b: 82,
        };
        let rgba_rust = RGBA {
            r: 172,
            g: 95,
            b: 82,
            a: 128,
        };
        let hsl_rust = HSL {
            h: Angle::new(9),
            s: 35,
            l: 50,
        };
        let hsla_rust = HSLA {
            h: Angle::new(9),
            s: 35,
            l: 50,
            a: 128,
        };

        // FIXME: update these tests once HSL <-> RBG impl exists
        assert_eq!(rgb_rust.to_hsl(), hsl_rust);
        assert_eq!(rgba_rust.to_hsla(), hsla_rust);
    }

    #[test]
    fn can_clone() {
        let rgb_color = RGB { r: 5, g: 10, b: 15 };
        let rgba_color = RGBA {
            r: 5,
            g: 10,
            b: 15,
            a: 255,
        };
        let hsl_color = HSL {
            h: Angle::new(6),
            s: 93,
            l: 71,
        };
        let hsla_color = HSLA {
            h: Angle::new(6),
            s: 93,
            l: 71,
            a: 255,
        };

        assert_eq!(rgb_color, rgb_color.clone());
        assert_eq!(rgba_color, rgba_color.clone());
        assert_eq!(hsl_color, hsl_color.clone());
        assert_eq!(hsla_color, hsla_color.clone());
    }

    #[test]
    fn can_copy() {
        let rgb_color = RGBA {
            r: 5,
            g: 10,
            b: 15,
            a: 255,
        };
        let copied_rgb_color = rgb_color;
        let rgba_color = RGBA {
            r: 5,
            g: 10,
            b: 15,
            a: 255,
        };
        let copied_rgba_color = rgba_color;
        let hsl_color = HSL {
            h: Angle::new(6),
            s: 93,
            l: 71,
        };
        let copied_hsl_color = hsl_color;
        let hsla_color = HSLA {
            h: Angle::new(6),
            s: 93,
            l: 71,
            a: 255,
        };
        let copied_hsla_color = hsla_color;

        assert_eq!(rgb_color, copied_rgb_color);
        assert_eq!(rgba_color, copied_rgba_color);
        assert_eq!(hsl_color, copied_hsl_color);
        assert_eq!(hsla_color, copied_hsla_color);
    }

    #[test]
    fn can_debug() {
        let rgb_value = format!("{:?}", RGB { r: 5, g: 10, b: 15 });
        let rgba_value = format!(
            "{:?}",
            RGBA {
                r: 5,
                g: 10,
                b: 15,
                a: 255
            }
        );
        let hsl_value = format!(
            "{:?}",
            HSL {
                h: Angle::new(6),
                s: 93,
                l: 71,
            }
        );
        let hsla_value = format!(
            "{:?}",
            HSLA {
                h: Angle::new(6),
                s: 93,
                l: 71,
                a: 255
            }
        );

        assert_eq!(rgb_value, "RGB { r: 5, g: 10, b: 15 }");
        assert_eq!(rgba_value, "RGBA { r: 5, g: 10, b: 15, a: 255 }");
        assert_eq!(hsl_value, "HSL { h: Angle { degrees: 6 }, s: 93, l: 71 }");
        assert_eq!(
            hsla_value,
            "HSLA { h: Angle { degrees: 6 }, s: 93, l: 71, a: 255 }"
        );
    }

    #[test]
    fn can_convert_to_css() {
        let rgb = RGB {
            r: 5,
            g: 10,
            b: 255,
        };
        let rgba = RGBA {
            r: 5,
            g: 10,
            b: 255,
            a: 255,
        };
        let hsl = HSL {
            h: Angle::new(6),
            s: 93,
            l: 71,
        };
        let hsla = HSLA {
            h: Angle::new(6),
            s: 93,
            l: 71,
            a: 255,
        };

        assert_eq!(rgb.to_css(), "rgb(5, 10, 255)");
        assert_eq!(rgba.to_css(), "rgba(5, 10, 255, 1.00)");
        assert_eq!(hsl.to_css(), "hsl(6, 93%, 71%)");
        assert_eq!(hsla.to_css(), "hsla(6, 93%, 71%, 1.00)");
    }

    #[test]
    fn can_print_in_css() {
        let printed_rgb = format!(
            "{}",
            RGB {
                r: 5,
                g: 10,
                b: 255
            }
        );
        let printed_rgba = format!(
            "{}",
            RGBA {
                r: 5,
                g: 10,
                b: 255,
                a: 255,
            }
        );
        let printed_hsl = format!(
            "{}",
            HSL {
                h: Angle::new(6),
                s: 93,
                l: 71,
            }
        );
        let printed_hsla = format!(
            "{}",
            HSLA {
                h: Angle::new(6),
                s: 93,
                l: 71,
                a: 255,
            }
        );

        assert_eq!(printed_rgb, "rgb(5, 10, 255)");
        assert_eq!(printed_rgba, "rgba(5, 10, 255, 1.00)");
        assert_eq!(printed_hsl, "hsl(6, 93%, 71%)");
        assert_eq!(printed_hsla, "hsla(6, 93%, 71%, 1.00)");
    }

    #[test]
    fn can_be_displayed() {
        let rgb = RGB {
            r: 5,
            g: 10,
            b: 255,
        };
        let rgba = RGBA {
            r: 5,
            g: 10,
            b: 255,
            a: 190,
        };
        let hsl = HSL {
            h: Angle::new(6),
            s: 93,
            l: 71,
        };
        let hsla = HSLA {
            h: Angle::new(6),
            s: 93,
            l: 71,
            a: 255,
        };

        assert_eq!("rgb(5, 10, 255)".to_owned(), format!("{}", rgb));
        assert_eq!("rgba(5, 10, 255, 0.75)".to_owned(), format!("{}", rgba));
        assert_eq!("hsl(6, 93%, 71%)".to_owned(), format!("{}", hsl));
        assert_eq!("hsla(6, 93%, 71%, 1.00)".to_owned(), format!("{}", hsla));
    }

    #[test]
    fn can_be_stringified() {
        let rgb = RGB {
            r: 5,
            g: 10,
            b: 255,
        };
        let rgba = RGBA {
            r: 5,
            g: 10,
            b: 255,
            a: 128,
        };
        let hsl = HSL {
            h: Angle::new(6),
            s: 93,
            l: 71,
        };
        let hsla = HSLA {
            h: Angle::new(6),
            s: 93,
            l: 71,
            a: 128,
        };

        assert_eq!(String::from("rgb(5, 10, 255)"), rgb.to_string());
        assert_eq!(String::from("rgba(5, 10, 255, 0.50)"), rgba.to_string());
        assert_eq!(String::from("hsl(6, 93%, 71%)"), hsl.to_string());
        assert_eq!(String::from("hsla(6, 93%, 71%, 0.50)"), hsla.to_string());
    }
}
