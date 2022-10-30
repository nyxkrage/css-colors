mod angle;
mod hsl;
mod ratio;
mod rgb;

pub use angle::*;
pub use hsl::*;
pub use ratio::*;
pub use rgb::*;

/// A trait that can be used for converting between different color models
/// and performing various transformations on them.
pub trait Color {
    type Alpha: Color;

    /// Converts `self` to its CSS string format.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba};
    ///
    /// let salmon = rgb(250, 128, 114);
    /// let opaque_salmon = rgba(250, 128, 114, 0.50);
    ///
    /// assert_eq!(salmon.to_css(), "rgb(250, 128, 114)");
    /// assert_eq!(opaque_salmon.to_css(), "rgba(250, 128, 114, 0.50)");
    /// ```
    fn to_css(self) -> String;

    /// Converts `self` to a hex string in the format #rrggbb without
    /// alpha, and #rrggbbaa with alpha.
    ///
    /// This will always be lowercase.
    ///
    /// # Example
    /// ```
    /// use css_colors::{Color, rgb, rgba};
    ///
    /// let salmon = rgb(250, 128, 114);
    /// let opaque_salmon = rgba(250, 128, 114, 0.50);
    ///
    /// assert_eq!(salmon.to_hex(), "#fa8072");
    /// assert_eq!(opaque_salmon.to_hex(), "#fa807280");
    /// ```
    fn to_hex(self) -> String;

    /// Converts `self` into its RGB representation.
    /// When converting from a color model that supports an alpha channel
    /// (e.g. RGBA), the alpha value will not be preserved.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba};
    ///
    /// let opaque_tomato = rgba(255, 99, 71, 0.5);
    ///
    /// assert_eq!(opaque_tomato.to_rgb(), rgb(255, 99, 71));
    /// ```
    fn to_rgb(self) -> RGB;

    /// Converts `self` into its RGBA representation.
    /// When converting from a color model that does not supports an alpha channel
    /// (e.g. RGB), it will be treated as fully opaque.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba};
    ///
    /// let tomato = rgb(255, 99, 71);
    ///
    /// assert_eq!(tomato.to_rgba(), rgba(255, 99, 71, 1.0));
    /// ```
    fn to_rgba(self) -> RGBA;

    /// Converts `self` into its HSL representation.
    /// When converting from a color model that supports an alpha channel
    /// (e.g. RGBA), the alpha value will not be preserved.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba, hsl};
    ///
    /// let tomato = rgb(255, 99, 71);
    /// let opaque_tomato = rgba(255, 99, 71, 0.5);
    ///
    /// assert_eq!(tomato.to_hsl(), hsl(9, 100, 64));
    /// assert_eq!(opaque_tomato.to_hsl(), hsl(9, 100, 64));
    /// ```
    fn to_hsl(self) -> HSL;

    /// Converts `self` into its HSLA representation.
    /// When converting from a color model that does not supports an alpha channel
    /// (e.g. RGB), it will be treated as fully opaque.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba, hsl, hsla};
    ///
    /// let tomato = rgb(255, 99, 71);
    /// let opaque_tomato = rgba(255, 99, 71, 0.5);
    ///
    /// assert_eq!(tomato.to_hsla(), hsla(9, 100, 64, 1.0));
    /// assert_eq!(opaque_tomato.to_hsla(), hsla(9, 100, 64, 0.5));
    /// ```
    fn to_hsla(self) -> HSLA;

    /// Increases the saturation of `self` by an absolute amount.
    /// Operates on the color within its HSL representation and preserves any existing alpha channel.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-saturate).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, hsla, percent};
    ///
    /// let salmon = hsla(6, 93, 71, 1.0);
    /// let cornflower_blue = rgb(100, 149, 237);
    ///
    /// assert_eq!(salmon.saturate(percent(7)), hsla(6, 100, 71, 1.0));
    /// assert_eq!(cornflower_blue.saturate(percent(10)), rgb(92, 146, 246));
    /// ```
    fn saturate(self, amount: Ratio) -> Self;

    /// Decreases the saturation of `self` by an absolute amount.
    /// Operates on the color within its HSL representation and preserves any existing alpha channel.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-desaturate).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba, percent};
    ///
    /// let tomato = rgba(255, 99, 71, 1.0);
    /// let cornflower_blue = rgb(100, 149, 237);
    ///
    /// assert_eq!(tomato.desaturate(percent(10)), rgba(246, 105, 80, 1.0));
    /// assert_eq!(cornflower_blue.desaturate(percent(33)), rgb(129, 157, 209));
    /// ```
    fn desaturate(self, amount: Ratio) -> Self;

    /// Increases the lightness of `self` by an absolute amount.
    /// Operates on the color within its HSL representation and preserves any existing alpha channel.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-lighten).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba, percent};
    ///
    /// let tomato = rgba(255, 99, 71, 1.0);
    /// let cornflower_blue = rgb(100, 149, 237);
    ///
    /// assert_eq!(tomato.lighten(percent(20)), rgba(255, 185, 173, 1.0));
    /// assert_eq!(cornflower_blue.lighten(percent(33)), rgb(251, 253, 255));
    /// ```
    fn lighten(self, amount: Ratio) -> Self;

    /// Decreases the lightness of `self` by an absolute amount.
    /// Operates on the color within its HSL representation and preserves any existing alpha channel.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-darken).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba, percent};
    ///
    /// let tomato = rgba(255, 99, 71, 1.0);
    /// let cornflower_blue = rgb(100, 149, 237);
    ///
    /// assert_eq!(tomato.darken(percent(20)), rgba(224, 34, 0, 1.0));
    /// assert_eq!(cornflower_blue.darken(percent(33)), rgb(18, 65, 152));
    /// ```
    fn darken(self, amount: Ratio) -> Self;

    /// Decreases the transparency (or increase the opacity) of `self`, making it more opaque.
    /// For opqaue colors, converts into the alpha equivalent of `self`, and then increases the opacity.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-fadein).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba, percent};
    ///
    /// let tomato = rgba(255, 99, 71, 0.25);
    /// let cornflower_blue = rgb(100, 149, 237);
    ///
    /// assert_eq!(tomato.fadein(percent(25)), rgba(255, 99, 71, 0.5));
    /// assert_eq!(cornflower_blue.fadein(percent(75)), rgba(100, 149, 237, 1.0));
    /// ```
    fn fadein(self, amount: Ratio) -> Self::Alpha;

    /// Increases the transparency (or decrease the opacity) of `self`, making it less opaque.
    /// For opqaue colors, converts into the alpha equivalent of `self`, and then decreases the opacity.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-fadeout).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba, percent};
    ///
    /// let tomato = rgba(255, 99, 71, 0.5);
    /// let cornflower_blue = rgb(100, 149, 237);
    ///
    /// assert_eq!(tomato.fadeout(percent(25)), rgba(255, 99, 71, 0.25));
    /// assert_eq!(cornflower_blue.fadeout(percent(75)), rgba(100, 149, 237, 0.25));
    /// ```
    fn fadeout(self, amount: Ratio) -> Self::Alpha;

    /// Sets the absolute opacity of `self`, and returns the alpha equivalent.
    /// Can be applied to colors whether they already have an opacity value or not.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-fade).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba, percent};
    ///
    /// let tomato = rgba(255, 99, 71, 0.5);
    /// let cornflower_blue = rgb(100, 149, 237);
    ///
    /// assert_eq!(tomato.fade(percent(25)), rgba(255, 99, 71, 0.25));
    /// assert_eq!(cornflower_blue.fade(percent(50)), rgba(100, 149, 237, 0.5));
    /// ```
    fn fade(self, amount: Ratio) -> Self::Alpha;

    /// Rotate the hue angle of `self` in either direction.
    /// Returns the appropriate `RGB` representation of the color once it has been spun.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-spin).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, hsl, deg};
    ///
    /// let red = hsl(10, 90, 50);
    /// let pink = rgb(243, 13, 90);
    ///
    /// assert_eq!(red.spin(deg(30)), hsl(40, 90, 50));
    /// assert_eq!(pink.spin(deg(-30)), rgb(243, 13, 205));
    /// ```
    fn spin(self, amount: Angle) -> Self;

    /// Mixes two colors (`self` and any other `Color`) together in variable proportion.
    /// Takes opacity into account in the calculations.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-mix).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba, hsl, hsla, percent};
    ///
    /// let red = hsl(10, 90, 50);
    /// let golden = rgb(243, 166, 13);
    /// let navy = rgba(0, 0, 80, 1.0);
    ///
    /// assert_eq!(red.mix(navy, percent(50)).to_string(), "hsla(347, 65%, 29%, 1.00)");
    /// assert_eq!(golden.mix(navy, percent(25)), rgba(61, 42, 63, 1.0));
    /// ```
    fn mix<T: Color>(self, other: T, weight: Ratio) -> Self::Alpha;

    /// Mixes `self` with white in variable proportion.
    /// Equivalent to calling `mix()` with `white` (`rgb(255, 255, 255)`).
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-tint).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba, hsl, percent};
    ///
    /// let red = hsl(10, 90, 50);
    /// let golden = rgb(243, 166, 13);
    ///
    /// assert_eq!(red.tint(percent(10)), hsl(10, 92, 95));
    /// assert_eq!(golden.tint(percent(25)), rgb(252, 233, 194));
    /// ```
    fn tint(self, weight: Ratio) -> Self;

    /// Mixes `self` with white in variable proportion.
    /// Equivalent to calling `mix()` with `black` (`rgb(0, 0, 0)`).
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-shade).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba, hsl, percent};
    ///
    /// let red = hsl(10, 90, 50);
    /// let golden = rgb(243, 166, 13);
    ///
    /// assert_eq!(red.shade(percent(10)), hsl(10, 92, 5));
    /// assert_eq!(golden.shade(percent(25)), rgb(61, 42, 3));
    /// ```
    fn shade(self, weight: Ratio) -> Self;

    /// Remove all saturation from `self` in the HSL color space.
    /// Equivalent to calling `desaturate(0)` on a color.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-greyscale).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, rgb, rgba};
    ///
    /// let tomato = rgba(255, 99, 71, 1.0);
    /// let cornflower_blue = rgb(100, 149, 237);
    ///
    /// assert_eq!(tomato.greyscale(), rgba(163, 163, 163, 1.0));
    /// assert_eq!(cornflower_blue.greyscale(), rgb(169, 169, 169));
    /// ```
    fn greyscale(self) -> Self;
}

#[cfg(feature = "serde")]
mod serde_integration {
    use serde::de::Error;
    use serde::{de::Visitor, Deserialize, Serialize, Serializer};
    use std::num::ParseIntError;

    use crate::Color;

    macro_rules! impl_serialize {
        ($x:ident) => (
            impl Serialize for crate::$x
            {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.serialize_str(&self.to_hex())
                }
            }
        );
        ($x:ident, $($y:ident),+ $(,)?) => (
            impl_serialize!($x);

            impl_serialize!($($y),+);
        );
    }

    impl_serialize!(RGB, RGBA, HSL, HSLA);

    struct RgbVisitor;
    impl<'de> Visitor<'de> for RgbVisitor {
        type Value = crate::RGB;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string in the format of #rrggbb")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let err = Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            ));
            if v.len() != 7 {
                return err;
            }

            if let Some('#') = v.chars().next() {
                let values: Vec<u8> = match (1..v.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&v[i..i + 2], 16))
                    .collect::<Result<Vec<u8>, ParseIntError>>()
                {
                    Ok(v) => v,
                    Err(_) => return err,
                };
                unsafe {
                    Ok(crate::rgb(
                        *values.get_unchecked(0),
                        *values.get_unchecked(1),
                        *values.get_unchecked(2),
                    ))
                }
            } else {
                err
            }
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }
    }
    struct RgbaVisitor;
    impl<'de> Visitor<'de> for RgbaVisitor {
        type Value = crate::RGBA;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string in the format of #rrggbbaa")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let err = Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            ));
            if v.len() != 9 {
                return err;
            }

            if let Some('#') = v.chars().next() {
                let values: Vec<u8> = match (1..v.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&v[i..i + 2], 16))
                    .collect::<Result<Vec<u8>, ParseIntError>>()
                {
                    Ok(v) => v,
                    Err(_) => return err,
                };
                unsafe {
                    Ok(crate::rgba(
                        *values.get_unchecked(0),
                        *values.get_unchecked(1),
                        *values.get_unchecked(2),
                        *values.get_unchecked(3) as f32 / 255.,
                    ))
                }
            } else {
                err
            }
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }
    }

    impl<'de> Deserialize<'de> for crate::RGB {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_string(RgbVisitor)
        }
    }
    impl<'de> Deserialize<'de> for crate::HSL {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer
                .deserialize_string(RgbVisitor)
                .map(|c| c.to_hsl())
        }
    }
    impl<'de> Deserialize<'de> for crate::RGBA {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_string(RgbaVisitor)
        }
    }
    impl<'de> Deserialize<'de> for crate::HSLA {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer
                .deserialize_string(RgbaVisitor)
                .map(|c| c.to_hsla())
        }
    }

    #[cfg(test)]
    #[test]
    fn no_alpha_json_deserializing() {
        let input_str = r##"{"color": "#010203"}"##;
        #[derive(Deserialize, Debug, PartialEq)]
        struct Test {
            color: crate::RGB,
        }
        let t: Test = serde_json::from_str(input_str).unwrap();
        assert_eq!(
            t,
            Test {
                color: crate::rgb(1, 2, 3)
            }
        )
    }

    #[cfg(test)]
    #[test]
    fn alpha_json_deserializing() {
        let input_str = r##"{"color": "#010203FF"}"##;
        #[derive(Deserialize, Debug, PartialEq)]
        struct Test {
            color: crate::RGBA,
        }
        let t: Test = serde_json::from_str(input_str).unwrap();
        assert_eq!(
            t,
            Test {
                color: crate::rgba(1, 2, 3, 1.)
            }
        )
    }
}

#[cfg(feature = "palette")]
mod palette_integration {
    use palette::RgbHue;
    use palette::Srgb;
    impl Into<Srgb> for crate::RGB {
        fn into(self) -> Srgb {
            Srgb::new(self.r.as_f32(), self.g.as_f32(), self.b.as_f32())
        }
    }

    use palette::Srgba;
    impl Into<Srgba> for crate::RGBA {
        fn into(self) -> Srgba {
            Srgba::new(
                self.r.as_f32(),
                self.g.as_f32(),
                self.b.as_f32(),
                self.a.as_f32(),
            )
        }
    }

    use palette::Hsl;
    impl Into<Hsl> for crate::HSL {
        fn into(self) -> Hsl {
            Hsl::new(
                RgbHue::from_degrees(self.h.degrees().into()),
                self.s.as_f32(),
                self.l.as_f32(),
            )
        }
    }

    use palette::Hsla;
    impl Into<Hsla> for crate::HSLA {
        fn into(self) -> Hsla {
            Hsla::new(
                RgbHue::from_degrees(self.h.degrees().into()),
                self.s.as_f32(),
                self.l.as_f32(),
                self.a.as_f32(),
            )
        }
    }

    macro_rules! from_css_to_palette {
        ($crate_color:ty, $temp_color:ty, $out_color:ty) => {
            impl Into<$out_color> for $crate_color {
                fn into(self) -> $out_color {
                    <$out_color as palette::FromColor<$temp_color>>::from_color(<Self as Into<
                        $temp_color,
                    >>::into(
                        self
                    ))
                }
            }
        };
        (RGB, $t:ty) => {
            from_css_to_palette!(crate::RGB, palette::Srgb, $t);
        };
        (RGBA, $t:ty) => {
            from_css_to_palette!(crate::RGBA, palette::Srgba, $t);
        };
        (HSL, $t:ty) => {
            from_css_to_palette!(crate::HSL, palette::Hsl, $t);
        };
        (HSLA, $t:ty) => {
            from_css_to_palette!(crate::HSLA, palette::Hsla, $t);
        };
        (ALL, $t:ty) => {
            from_css_to_palette!(RGB, $t);
            from_css_to_palette!(RGBA, $t);
            from_css_to_palette!(HSL, $t);
            from_css_to_palette!(HSLA, $t);
        };
    }

    from_css_to_palette!(HSL, palette::Srgb);
    from_css_to_palette!(HSLA, palette::Srgb);
    from_css_to_palette!(HSLA, palette::Srgba);
    from_css_to_palette!(HSLA, palette::Hsl);
    from_css_to_palette!(RGB, palette::Hsl);
    from_css_to_palette!(RGBA, palette::Hsla);
    from_css_to_palette!(RGBA, palette::Hsl);
    from_css_to_palette!(RGBA, palette::Srgb);
    from_css_to_palette!(ALL, palette::Hsluva);
    from_css_to_palette!(ALL, palette::Hsva);
    from_css_to_palette!(ALL, palette::Hwba);
    from_css_to_palette!(ALL, palette::Laba);
    from_css_to_palette!(ALL, palette::Lcha);
    from_css_to_palette!(ALL, palette::Lchuva);
    from_css_to_palette!(ALL, palette::Luva);
    from_css_to_palette!(ALL, palette::Oklaba);
    from_css_to_palette!(ALL, palette::Oklcha);
    from_css_to_palette!(ALL, palette::Xyza);
    from_css_to_palette!(ALL, palette::Yxya);
    from_css_to_palette!(ALL, palette::Hsluv);
    from_css_to_palette!(ALL, palette::Hsv);
    from_css_to_palette!(ALL, palette::Hwb);
    from_css_to_palette!(ALL, palette::Lab);
    from_css_to_palette!(ALL, palette::Lch);
    from_css_to_palette!(ALL, palette::Lchuv);
    from_css_to_palette!(ALL, palette::Luv);
    from_css_to_palette!(ALL, palette::Oklab);
    from_css_to_palette!(ALL, palette::Oklch);
    from_css_to_palette!(ALL, palette::Xyz);
    from_css_to_palette!(ALL, palette::Yxy);
}

#[cfg(test)]
mod css_color_tests {
    use crate::{hsl, hsla, rgb, rgba, percent, deg, Angle, Color, Ratio, HSL, HSLA, RGB, RGBA};

    pub trait ApproximatelyEq {
        fn approximately_eq(self, other: Self) -> bool;
    }

    impl ApproximatelyEq for u8 {
        fn approximately_eq(self, other: Self) -> bool {
            self == other || self + 1 == other || self - 1 == other
        }
    }

    impl ApproximatelyEq for u16 {
        fn approximately_eq(self, other: Self) -> bool {
            self == other || self + 1 == other || self - 1 == other
        }
    }

    impl ApproximatelyEq for Angle {
        fn approximately_eq(self, other: Self) -> bool {
            self.degrees().approximately_eq(other.degrees())
        }
    }

    impl ApproximatelyEq for Ratio {
        fn approximately_eq(self, other: Self) -> bool {
            self.as_u8().approximately_eq(other.as_u8())
        }
    }

    impl ApproximatelyEq for RGB {
        fn approximately_eq(self, other: Self) -> bool {
            self.to_css() == other.to_css()
                || self.r.approximately_eq(other.r)
                    && self.g.approximately_eq(other.g)
                    && self.b.approximately_eq(other.b)
        }
    }

    impl ApproximatelyEq for RGBA {
        fn approximately_eq(self, other: Self) -> bool {
            self.to_css() == other.to_css()
                || self.r.approximately_eq(other.r)
                    && self.g.approximately_eq(other.g)
                    && self.b.approximately_eq(other.b)
                    && self.a == other.a
        }
    }

    impl ApproximatelyEq for HSL {
        fn approximately_eq(self, other: Self) -> bool {
            self.to_css() == other.to_css()
                || self.h.approximately_eq(other.h)
                    && self
                        .s
                        .as_percentage()
                        .approximately_eq(other.s.as_percentage())
                    && self
                        .l
                        .as_percentage()
                        .approximately_eq(other.l.as_percentage())
        }
    }

    impl ApproximatelyEq for HSLA {
        fn approximately_eq(self, other: Self) -> bool {
            self.to_css() == other.to_css()
                || self.h.approximately_eq(other.h)
                    && self
                        .s
                        .as_percentage()
                        .approximately_eq(other.s.as_percentage())
                    && self
                        .l
                        .as_percentage()
                        .approximately_eq(other.l.as_percentage())
                    && self.a == other.a
        }
    }

    macro_rules! assert_approximately_eq {
        ($lhs:expr, $rhs:expr) => {
            let lhs = $lhs;
            let rhs = $rhs;

            assert!(lhs.approximately_eq(rhs), "lhs: {}, rhs: {}", lhs, rhs);
        };
    }

    #[test]
    fn can_create_color_structs() {
        assert_eq!(
            rgb(5, 10, 15),
            RGB {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(15),
            }
        );
        assert_eq!(
            rgba(5, 10, 15, 1.0),
            RGBA {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(15),
                a: Ratio::from_u8(255),
            }
        );
        assert_eq!(
            hsl(6, 93, 71),
            HSL {
                h: Angle::new(6),
                s: Ratio::from_percentage(93),
                l: Ratio::from_percentage(71)
            }
        );
        assert_eq!(
            hsla(6, 93, 71, 1.0),
            HSLA {
                h: Angle::new(6),
                s: Ratio::from_percentage(93),
                l: Ratio::from_percentage(71),
                a: Ratio::from_u8(255),
            }
        );
    }

    #[macro_use]
    mod conversions {
        macro_rules! conversion_test {
            (
                $color_name:ident,
                rgb($r:expr, $g:expr, $b:expr),
                hsl($h:expr, $s:expr, $l:expr)
            ) => {
                mod $color_name {
                    use super::super::ApproximatelyEq;
                    use $crate::{hsl, hsla, rgb, rgba, Color};

                    #[test]
                    fn rgb_to_rgb() {
                        assert_eq!(rgb($r, $g, $b).to_rgb(), rgb($r, $g, $b));
                    }

                    #[test]
                    fn rgb_to_rgba() {
                        assert_eq!(rgb($r, $g, $b).to_rgba(), rgba($r, $g, $b, 1.0));
                    }

                    #[test]
                    fn rgba_to_rgb() {
                        assert_eq!(rgba($r, $g, $b, 1.0).to_rgb(), rgb($r, $g, $b));
                        assert_eq!(rgba($r, $g, $b, 0.78).to_rgb(), rgb($r, $g, $b));
                        assert_eq!(rgba($r, $g, $b, 0.0).to_rgb(), rgb($r, $g, $b));
                    }

                    #[test]
                    fn rgba_to_rgba() {
                        assert_eq!(rgba($r, $g, $b, 1.0).to_rgba(), rgba($r, $g, $b, 1.0));

                        assert_eq!(rgba($r, $g, $b, 0.78).to_rgba(), rgba($r, $g, $b, 0.78));

                        assert_eq!(rgba($r, $g, $b, 0.0).to_rgba(), rgba($r, $g, $b, 0.0));
                    }

                    #[test]
                    fn rgb_to_hsl() {
                        assert_approximately_eq!(rgb($r, $g, $b).to_hsl(), hsl($h, $s, $l));
                    }

                    #[test]
                    fn rgb_to_hsla() {
                        assert_approximately_eq!(rgb($r, $g, $b).to_hsla(), hsla($h, $s, $l, 1.0));
                    }

                    #[test]
                    fn rgba_to_hsl() {
                        assert_approximately_eq!(rgba($r, $g, $b, 1.0).to_hsl(), hsl($h, $s, $l));

                        assert_approximately_eq!(rgba($r, $g, $b, 0.78).to_hsl(), hsl($h, $s, $l));

                        assert_approximately_eq!(rgba($r, $g, $b, 0.0).to_hsl(), hsl($h, $s, $l));
                    }

                    #[test]
                    fn rgba_to_hsla() {
                        assert_approximately_eq!(
                            rgba($r, $g, $b, 1.0).to_hsla(),
                            hsla($h, $s, $l, 1.0)
                        );

                        assert_approximately_eq!(
                            rgba($r, $g, $b, 0.78).to_hsla(),
                            hsla($h, $s, $l, 0.78)
                        );

                        assert_approximately_eq!(
                            rgba($r, $g, $b, 0.0).to_hsla(),
                            hsla($h, $s, $l, 0.0)
                        );
                    }

                    #[test]
                    fn hsl_to_hsl() {
                        assert_eq!(hsl($h, $s, $l).to_hsl(), hsl($h, $s, $l));
                    }

                    #[test]
                    fn hsl_to_hsla() {
                        assert_eq!(hsl($h, $s, $l).to_hsla(), hsla($h, $s, $l, 1.0));
                    }

                    #[test]
                    fn hsla_to_hsl() {
                        assert_eq!(hsla($h, $s, $l, 1.0).to_hsl(), hsl($h, $s, $l));

                        assert_eq!(hsla($h, $s, $l, 0.78).to_hsl(), hsl($h, $s, $l));

                        assert_eq!(hsla($h, $s, $l, 0.0).to_hsl(), hsl($h, $s, $l));
                    }

                    #[test]
                    fn hsla_to_hsla() {
                        assert_eq!(hsla($h, $s, $l, 1.0).to_hsla(), hsla($h, $s, $l, 1.0));

                        assert_eq!(hsla($h, $s, $l, 0.78).to_hsla(), hsla($h, $s, $l, 0.78));

                        assert_eq!(hsla($h, $s, $l, 0.0).to_hsla(), hsla($h, $s, $l, 0.0));
                    }

                    #[test]
                    fn hsl_to_rgb() {
                        assert_approximately_eq!(hsl($h, $s, $l).to_rgb(), rgb($r, $g, $b));
                    }

                    #[test]
                    fn hsl_to_rgba() {
                        assert_approximately_eq!(hsl($h, $s, $l).to_rgba(), rgba($r, $g, $b, 1.0));
                    }

                    #[test]
                    fn hsla_to_rgb() {
                        assert_approximately_eq!(hsla($h, $s, $l, 1.0).to_rgb(), rgb($r, $g, $b));

                        assert_approximately_eq!(hsla($h, $s, $l, 0.78).to_rgb(), rgb($r, $g, $b));

                        assert_approximately_eq!(hsla($h, $s, $l, 0.0).to_rgb(), rgb($r, $g, $b));
                    }

                    #[test]
                    fn hsla_to_rgba() {
                        assert_approximately_eq!(
                            hsla($h, $s, $l, 1.0).to_rgba(),
                            rgba($r, $g, $b, 1.0)
                        );

                        assert_approximately_eq!(
                            hsla($h, $s, $l, 0.78).to_rgba(),
                            rgba($r, $g, $b, 0.78)
                        );

                        assert_approximately_eq!(
                            hsla($h, $s, $l, 0.0).to_rgba(),
                            rgba($r, $g, $b, 0.0)
                        );
                    }
                }
            };
        }

        conversion_test!(black, rgb(0, 0, 0), hsl(0, 0, 0));
        conversion_test!(grey, rgb(230, 230, 230), hsl(0, 0, 90));
        conversion_test!(white, rgb(255, 255, 255), hsl(0, 0, 100));
        conversion_test!(pink, rgb(253, 216, 229), hsl(339, 90, 92));
        conversion_test!(brown, rgb(172, 96, 83), hsl(9, 35, 50));
        conversion_test!(teal, rgb(23, 98, 119), hsl(193, 68, 28));
        conversion_test!(green, rgb(89, 161, 54), hsl(100, 50, 42));
        conversion_test!(pale_blue, rgb(148, 189, 209), hsl(200, 40, 70));
        conversion_test!(mauve, rgb(136, 102, 153), hsl(280, 20, 50));
        conversion_test!(cherry, rgb(230, 25, 60), hsl(350, 80, 50));
        conversion_test!(tomato, rgb(255, 99, 71), hsl(9, 100, 64));
        conversion_test!(light_salmon, rgb(255, 160, 122), hsl(17, 100, 74));
        conversion_test!(blue_violet, rgb(138, 43, 226), hsl(271, 76, 53));
        conversion_test!(dark_orange, rgb(255, 140, 0), hsl(33, 100, 50));
        conversion_test!(deep_pink, rgb(255, 20, 147), hsl(328, 100, 54));
        conversion_test!(chartreuse, rgb(127, 255, 0), hsl(90, 100, 50));
    }

    #[test]
    fn can_saturate() {
        assert_approximately_eq!(hsl(9, 35, 50).saturate(percent(20)), hsl(9, 55, 50));
        assert_approximately_eq!(
            hsla(9, 35, 50, 1.0).saturate(percent(20)),
            hsla(9, 55, 50, 1.0)
        );

        assert_approximately_eq!(rgb(172, 96, 83).saturate(percent(20)), rgb(197, 78, 57));
        assert_approximately_eq!(
            rgba(172, 96, 83, 1.0).saturate(percent(20)),
            rgba(197, 78, 57, 1.0)
        );
    }

    #[test]
    fn can_desaturate() {
        assert_approximately_eq!(hsl(9, 55, 50).desaturate(percent(20)), hsl(9, 35, 50));
        assert_approximately_eq!(
            hsla(9, 55, 50, 1.0).desaturate(percent(20)),
            hsla(9, 35, 50, 1.0)
        );
        assert_approximately_eq!(rgb(197, 78, 57).desaturate(percent(20)), rgb(172, 96, 83));
        assert_approximately_eq!(
            rgba(197, 78, 57, 1.0).desaturate(percent(20)),
            rgba(172, 96, 83, 1.0)
        );
    }

    #[test]
    fn can_lighten() {
        assert_approximately_eq!(hsl(9, 35, 50).lighten(percent(20)), hsl(9, 35, 70));
        assert_approximately_eq!(
            hsla(9, 35, 50, 1.0).lighten(percent(20)),
            hsla(9, 35, 70, 1.0)
        );
        assert_approximately_eq!(rgb(172, 96, 83).lighten(percent(20)), rgb(205, 160, 152));
        assert_approximately_eq!(
            rgba(172, 96, 83, 1.0).lighten(percent(20)),
            rgba(205, 160, 152, 1.0)
        );
    }

    #[test]
    fn can_darken() {
        assert_approximately_eq!(hsl(9, 35, 70).darken(percent(20)), hsl(9, 35, 50));
        assert_approximately_eq!(
            hsla(9, 35, 70, 1.0).darken(percent(20)),
            hsla(9, 35, 50, 1.0)
        );
        assert_approximately_eq!(rgb(205, 160, 152).darken(percent(20)), rgb(172, 96, 83));
        assert_approximately_eq!(
            rgba(205, 160, 152, 1.0).darken(percent(20)),
            rgba(172, 96, 83, 1.0)
        );
    }

    #[test]
    fn can_fadein() {
        assert_approximately_eq!(hsl(9, 35, 50).fadein(percent(25)), hsla(9, 35, 50, 1.0));
        assert_approximately_eq!(
            hsla(9, 35, 50, 0.5).fadein(percent(25)),
            hsla(9, 35, 50, 0.75)
        );
        assert_approximately_eq!(rgb(172, 96, 83).fadein(percent(25)), rgba(172, 96, 83, 1.0));
        assert_approximately_eq!(
            rgba(172, 96, 83, 0.50).fadein(percent(25)),
            rgba(172, 96, 83, 0.75)
        );
    }

    #[test]
    fn can_fadeout() {
        assert_approximately_eq!(hsl(9, 35, 50).fadeout(percent(25)), hsla(9, 35, 50, 0.75));
        assert_approximately_eq!(
            rgb(172, 96, 83).fadeout(percent(25)),
            rgba(172, 96, 83, 0.75)
        );
        assert_approximately_eq!(
            hsla(9, 35, 50, 0.60).fadeout(percent(25)),
            hsla(9, 35, 50, 0.35)
        );
        assert_approximately_eq!(
            rgba(172, 96, 83, 0.60).fadeout(percent(25)),
            rgba(172, 96, 83, 0.35)
        );
    }

    #[test]
    fn can_fade() {
        let faded_color = rgba(23, 98, 119, 0.5);

        assert_approximately_eq!(rgb(23, 98, 119).fade(percent(50)), faded_color);
        assert_approximately_eq!(rgba(23, 98, 119, 1.0).fade(percent(50)), faded_color);
        assert_approximately_eq!(hsl(193, 67, 28).fade(percent(50)), faded_color.to_hsla());
        assert_approximately_eq!(
            hsla(193, 67, 28, 1.0).fade(percent(50)),
            faded_color.to_hsla()
        );
    }

    #[test]
    fn can_spin_forward() {
        assert_approximately_eq!(rgb(75, 207, 23).spin(deg(100)), rgb(23, 136, 207));
        assert_approximately_eq!(
            rgba(75, 207, 23, 1.0).spin(deg(100)),
            rgba(23, 136, 207, 1.0)
        );
        assert_approximately_eq!(hsl(10, 90, 50).spin(deg(30)), hsl(40, 90, 50));
        assert_approximately_eq!(hsla(10, 90, 50, 1.0).spin(deg(30)), hsla(40, 90, 50, 1.0));
    }

    #[test]
    fn can_spin_backwards() {
        assert_approximately_eq!(rgb(75, 207, 23).spin(deg(-100)), rgb(207, 32, 23));
        assert_approximately_eq!(
            rgba(75, 207, 23, 1.0).spin(deg(-100)),
            rgba(207, 32, 23, 1.0)
        );
        assert_approximately_eq!(hsl(10, 90, 50).spin(deg(-30)), hsl(340, 90, 50));
        assert_approximately_eq!(hsla(10, 90, 50, 1.0).spin(deg(-30)), hsla(340, 90, 50, 1.0));
    }

    #[test]
    fn can_mix() {
        let brown_rgba = rgba(50, 50, 0, 1.0);
        let brown_hsla = hsla(60, 100, 10, 1.0);

        assert_approximately_eq!(
            rgba(100, 0, 0, 1.0).mix(rgba(0, 100, 0, 1.0), percent(50)),
            brown_rgba
        );
        assert_approximately_eq!(rgb(100, 0, 0).mix(rgb(0, 100, 0), percent(50)), brown_rgba);
        assert_approximately_eq!(
            hsl(0, 100, 20).mix(hsl(120, 100, 20), percent(50)),
            brown_hsla
        );
        assert_approximately_eq!(
            hsla(0, 100, 20, 1.0).mix(hsla(120, 100, 20, 1.0), percent(50)),
            brown_hsla
        );
    }

    #[test]
    fn can_mix_single_color() {
        let rgba_red = rgba(100, 0, 0, 1.0);
        let rgba_green = rgba(0, 100, 0, 0.5);
        let hsla_red = hsla(120, 100, 20, 1.0);
        let hsla_green = hsla(0, 100, 20, 0.5);

        assert_approximately_eq!(rgba_red.mix(rgba_green, percent(100)), rgba_red);
        assert_approximately_eq!(rgba_red.mix(rgba_green, percent(0)), rgba_green);
        assert_approximately_eq!(rgba_green.mix(rgba_red, percent(100)), rgba_green);
        assert_approximately_eq!(rgba_green.mix(rgba_red, percent(0)), rgba_red);
        assert_approximately_eq!(rgba_red.mix(rgba_green, percent(0)), rgba_green);

        assert_approximately_eq!(hsla_red.mix(hsla_green, percent(100)), hsla_red);
        assert_approximately_eq!(hsla_red.mix(hsla_green, percent(0)), hsla_green);
        assert_approximately_eq!(hsla_green.mix(hsla_red, percent(100)), hsla_green);
        assert_approximately_eq!(hsla_green.mix(hsla_red, percent(0)), hsla_red);
        assert_approximately_eq!(hsla_red.mix(hsla_green, percent(0)), hsla_green);
    }

    #[test]
    fn can_mix_with_alpha() {
        let red_rgba = rgba(100, 0, 0, 1.0);
        let green_rgba = rgba(0, 100, 0, 0.5);
        let brown_rgba = rgba(75, 25, 0, 0.75);
        let green_hsla = hsla(120, 100, 20, 1.0);
        let red_hsla = hsla(0, 100, 20, 1.0);
        let brown_hsla = hsla(60, 100, 10, 1.0);

        assert_approximately_eq!(red_rgba.mix(green_rgba, percent(50)), brown_rgba);
        assert_approximately_eq!(green_rgba.mix(red_rgba, percent(50)), brown_rgba);
        assert_approximately_eq!(red_hsla.mix(green_hsla, percent(50)), brown_hsla);
        assert_approximately_eq!(green_hsla.mix(red_hsla, percent(50)), brown_hsla);
    }

    #[test]
    fn can_tint() {
        assert_approximately_eq!(
            rgba(0, 0, 255, 0.5).tint(percent(50)),
            rgba(191, 191, 255, 0.75)
        );
        assert_approximately_eq!(rgb(0, 0, 255).tint(percent(50)), rgb(128, 128, 255));
        assert_approximately_eq!(hsl(6, 93, 71).tint(percent(50)), hsl(6, 92, 85));
        assert_approximately_eq!(
            hsla(6, 93, 71, 0.5).tint(percent(50)),
            hsla(6, 95, 93, 0.75)
        );
    }

    #[test]
    fn can_shade() {
        assert_approximately_eq!(
            rgba(0, 0, 255, 0.5).shade(percent(50)),
            rgba(0, 0, 64, 0.75)
        );
        assert_approximately_eq!(rgb(0, 0, 255).shade(percent(50)), rgb(0, 0, 128));
        assert_approximately_eq!(hsl(6, 93, 71).shade(percent(50)), hsl(6, 38, 36));
        assert_approximately_eq!(
            hsla(6, 93, 71, 0.5).shade(percent(50)),
            hsla(7, 38, 18, 0.75)
        );
    }

    #[test]
    fn can_greyscale() {
        assert_approximately_eq!(rgb(128, 242, 13).greyscale(), rgb(128, 128, 128));
        assert_approximately_eq!(
            rgba(128, 242, 13, 1.0).greyscale(),
            rgba(128, 128, 128, 1.0)
        );
        assert_approximately_eq!(hsl(90, 90, 50).greyscale(), hsl(90, 0, 50));
        assert_approximately_eq!(hsla(90, 90, 50, 1.0).greyscale(), hsla(90, 0, 50, 1.0));
    }

    #[test]
    fn can_clone() {
        let rgb_color = rgb(5, 10, 15);
        let rgba_color = rgba(5, 10, 15, 1.0);
        let hsl_color = hsl(6, 93, 71);
        let hsla_color = hsla(6, 93, 71, 1.0);

        assert_eq!(rgb_color, rgb_color.clone());
        assert_eq!(rgba_color, rgba_color.clone());
        assert_eq!(hsl_color, hsl_color.clone());
        assert_eq!(hsla_color, hsla_color.clone());
    }

    #[test]
    fn can_copy() {
        let rgb_color = rgb(172, 95, 82);
        let rgba_color = rgba(172, 95, 82, 1.0);
        let hsl_color = hsl(9, 35, 50);
        let hsla_color = hsla(9, 35, 50, 1.0);

        let copied_rgb_color = rgb_color;
        let copied_rgba_color = rgba_color;
        let copied_hsl_color = hsl_color;
        let copied_hsla_color = hsla_color;

        assert_eq!(rgb_color, copied_rgb_color);
        assert_eq!(rgba_color, copied_rgba_color);
        assert_eq!(hsl_color, copied_hsl_color);
        assert_eq!(hsla_color, copied_hsla_color);
    }

    #[test]
    fn can_debug() {
        let rgb_value = format!("{:?}", rgb(5, 10, 15));
        let rgba_value = format!("{:?}", rgba(5, 10, 15, 1.0));
        let hsl_value = format!("{:?}", hsl(6, 93, 71));
        let hsla_value = format!("{:?}", hsla(6, 93, 71, 1.0));

        assert_eq!(rgb_value, "RGB { r: Ratio(5), g: Ratio(10), b: Ratio(15) }");
        assert_eq!(
            rgba_value,
            "RGBA { r: Ratio(5), g: Ratio(10), b: Ratio(15), a: Ratio(255) }"
        );
        assert_eq!(
            hsl_value,
            "HSL { h: Angle { degrees: 6 }, s: Ratio(237), l: Ratio(181) }"
        );
        assert_eq!(
            hsla_value,
            "HSLA { h: Angle { degrees: 6 }, s: Ratio(237), l: Ratio(181), a: Ratio(255) }"
        );
    }

    #[test]
    fn can_convert_to_css() {
        let rgb = rgb(5, 10, 255);
        let rgba = rgba(5, 10, 255, 1.0);
        let hsl = hsl(6, 93, 71);
        let hsla = hsla(6, 93, 71, 1.0);

        assert_eq!(rgb.to_css(), "rgb(5, 10, 255)");
        assert_eq!(rgba.to_css(), "rgba(5, 10, 255, 1.00)");
        assert_eq!(hsl.to_css(), "hsl(6, 93%, 71%)");
        assert_eq!(hsla.to_css(), "hsla(6, 93%, 71%, 1.00)");
    }

    #[test]
    fn can_convert_to_hex() {
        let rgb = rgb(5, 10, 255);
        let rgba = rgba(5, 10, 255, 1.0);
        let hsl = hsl(6, 93, 71);
        let hsla = hsla(6, 93, 71, 1.0);

        assert_eq!(rgb.to_hex(), "#050aff");
        assert_eq!(rgba.to_hex(), "#050affff");
        assert_eq!(hsl.to_hex(), "#fa7e70");
        assert_eq!(hsla.to_hex(), "#fa7e70ff");
    }

    #[test]
    fn can_print_in_css() {
        let printed_rgb = format!("{}", rgb(5, 10, 255));
        let printed_rgba = format!("{}", rgba(5, 10, 255, 1.0));
        let printed_hsl = format!("{}", hsl(6, 93, 71));
        let printed_hsla = format!("{}", hsla(6, 93, 71, 1.0));

        assert_eq!(printed_rgb, "rgb(5, 10, 255)");
        assert_eq!(printed_rgba, "rgba(5, 10, 255, 1.00)");
        assert_eq!(printed_hsl, "hsl(6, 93%, 71%)");
        assert_eq!(printed_hsla, "hsla(6, 93%, 71%, 1.00)");
    }

    #[test]
    fn can_be_displayed() {
        let rgb = rgb(5, 10, 255);
        let rgba = rgba(5, 10, 255, 0.75);
        let hsl = hsl(6, 93, 71);
        let hsla = hsla(6, 93, 71, 1.0);

        assert_eq!("rgb(5, 10, 255)".to_owned(), format!("{}", rgb));
        assert_eq!("rgba(5, 10, 255, 0.75)".to_owned(), format!("{}", rgba));
        assert_eq!("hsl(6, 93%, 71%)".to_owned(), format!("{}", hsl));
        assert_eq!("hsla(6, 93%, 71%, 1.00)".to_owned(), format!("{}", hsla));
    }

    #[test]
    fn can_be_stringified() {
        let rgb = rgb(5, 10, 255);
        let rgba = rgba(5, 10, 255, 0.5);
        let hsl = hsl(6, 93, 71);
        let hsla = hsla(6, 93, 71, 0.5);

        assert_eq!(String::from("rgb(5, 10, 255)"), rgb.to_string());
        assert_eq!(String::from("rgba(5, 10, 255, 0.50)"), rgba.to_string());
        assert_eq!(String::from("hsl(6, 93%, 71%)"), hsl.to_string());
        assert_eq!(String::from("hsla(6, 93%, 71%, 0.50)"), hsla.to_string());
    }
    #[cfg(feature = "palette")]
    mod palette_integration {
        #[test]
        fn rgb() {
            let expected = palette::Srgb::new(1.0, 1.0, 1.0);
            let css_value = crate::rgb(255, 255, 255);
            let actual = css_value.into();

            assert_eq!(expected, actual);
        }

        #[test]
        fn rgba() {
            let expected = palette::Srgba::new(1.0, 1.0, 1.0, 1.0);
            let css_value = crate::rgba(255, 255, 255, 1.0);
            let actual = css_value.into();

            assert_eq!(expected, actual);
        }

        #[test]
        fn hsl() {
            let expected = palette::Hsl::new(180.0, 1.0, 1.0);
            let css_value = crate::hsl(180, 100, 100);
            let actual = css_value.into();

            assert_eq!(expected, actual);
        }

        #[test]
        fn hsla() {
            let expected = palette::Hsla::new(180.0, 1.0, 1.0, 1.0);
            let css_value = crate::hsla(180, 100, 100, 1.0);
            let actual = css_value.into();

            assert_eq!(expected, actual);
        }

        #[test]
        fn conversion_methods_exists() {
            let _srgb: palette::Srgb = crate::rgb(255, 255, 255).into();
            let _hsl: palette::Hsl = crate::rgb(255, 255, 255).into();
            let _hsluv: palette::Hsluv = crate::rgb(255, 255, 255).into();
            let _hsv: palette::Hsv = crate::rgb(255, 255, 255).into();
            let _hwb: palette::Hwb = crate::rgb(255, 255, 255).into();
            let _lab: palette::Lab = crate::rgb(255, 255, 255).into();
            let _lch: palette::Lch = crate::rgb(255, 255, 255).into();
            let _lchuv: palette::Lchuv = crate::rgb(255, 255, 255).into();
            let _luv: palette::Luv = crate::rgb(255, 255, 255).into();
            let _oklab: palette::Oklab = crate::rgb(255, 255, 255).into();
            let _oklch: palette::Oklch = crate::rgb(255, 255, 255).into();
            let _xyz: palette::Xyz = crate::rgb(255, 255, 255).into();
            let _yxy: palette::Yxy = crate::rgb(255, 255, 255).into();
            let _srgb: palette::Srgb = crate::hsl(180, 100, 100).into();
            let _hsl: palette::Hsl = crate::hsl(180, 100, 100).into();
            let _hsluv: palette::Hsluv = crate::hsl(180, 100, 100).into();
            let _hsv: palette::Hsv = crate::hsl(180, 100, 100).into();
            let _hwb: palette::Hwb = crate::hsl(180, 100, 100).into();
            let _lab: palette::Lab = crate::hsl(180, 100, 100).into();
            let _lch: palette::Lch = crate::hsl(180, 100, 100).into();
            let _lchuv: palette::Lchuv = crate::hsl(180, 100, 100).into();
            let _luv: palette::Luv = crate::hsl(180, 100, 100).into();
            let _oklab: palette::Oklab = crate::hsl(180, 100, 100).into();
            let _oklch: palette::Oklch = crate::hsl(180, 100, 100).into();
            let _xyz: palette::Xyz = crate::hsl(180, 100, 100).into();
            let _yxy: palette::Yxy = crate::hsl(180, 100, 100).into();
            let _srgb: palette::Srgb = crate::rgba(255, 255, 255, 1.0).into();
            let _hsl: palette::Hsl = crate::rgba(255, 255, 255, 1.0).into();
            let _hsluv: palette::Hsluv = crate::rgba(255, 255, 255, 1.0).into();
            let _hsv: palette::Hsv = crate::rgba(255, 255, 255, 1.0).into();
            let _hwb: palette::Hwb = crate::rgba(255, 255, 255, 1.0).into();
            let _lab: palette::Lab = crate::rgba(255, 255, 255, 1.0).into();
            let _lch: palette::Lch = crate::rgba(255, 255, 255, 1.0).into();
            let _lchuv: palette::Lchuv = crate::rgba(255, 255, 255, 1.0).into();
            let _luv: palette::Luv = crate::rgba(255, 255, 255, 1.0).into();
            let _oklab: palette::Oklab = crate::rgba(255, 255, 255, 1.0).into();
            let _oklch: palette::Oklch = crate::rgba(255, 255, 255, 1.0).into();
            let _xyz: palette::Xyz = crate::rgba(255, 255, 255, 1.0).into();
            let _yxy: palette::Yxy = crate::rgba(255, 255, 255, 1.0).into();
            let _srgb: palette::Srgb = crate::hsla(180, 100, 100, 1.0).into();
            let _hsluv: palette::Hsluv = crate::hsla(180, 100, 100, 1.0).into();
            let _hsv: palette::Hsv = crate::hsla(180, 100, 100, 1.0).into();
            let _hwb: palette::Hwb = crate::hsla(180, 100, 100, 1.0).into();
            let _lab: palette::Lab = crate::hsla(180, 100, 100, 1.0).into();
            let _lch: palette::Lch = crate::hsla(180, 100, 100, 1.0).into();
            let _lchuv: palette::Lchuv = crate::hsla(180, 100, 100, 1.0).into();
            let _luv: palette::Luv = crate::hsla(180, 100, 100, 1.0).into();
            let _oklab: palette::Oklab = crate::hsla(180, 100, 100, 1.0).into();
            let _oklch: palette::Oklch = crate::hsla(180, 100, 100, 1.0).into();
            let _xyz: palette::Xyz = crate::hsla(180, 100, 100, 1.0).into();
            let _yxy: palette::Yxy = crate::hsla(180, 100, 100, 1.0).into();

            assert!(true)
        }
    }
}
