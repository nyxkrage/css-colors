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
                >>::into(self))
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

#[cfg(test)]
mod tests {
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
