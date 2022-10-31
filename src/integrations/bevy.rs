use crate::{hsl, hsla, rgb, rgba, Color};
use bevy::render::color::Color as BevyColor;

impl Into<BevyColor> for crate::RGB {
    fn into(self) -> BevyColor {
        self.to_rgba().into()
    }
}
impl Into<BevyColor> for crate::RGBA {
    fn into(self) -> BevyColor {
        BevyColor::Rgba {
            red: self.r.as_f32(),
            green: self.g.as_f32(),
            blue: self.b.as_f32(),
            alpha: self.a.as_f32(),
        }
    }
}
impl Into<BevyColor> for crate::HSL {
    fn into(self) -> BevyColor {
        self.to_hsla().into()
    }
}
impl Into<BevyColor> for crate::HSLA {
    fn into(self) -> BevyColor {
        BevyColor::Hsla {
            hue: self.h.degrees() as f32,
            saturation: self.s.as_f32(),
            lightness: self.l.as_f32(),
            alpha: self.a.as_f32(),
        }
    }
}

#[cfg(test)]
fn test() {
    let expected_hsla = BevyColor::hsla(128., 1., 1., 1.);
    let actual_hsl = hsl(128, 100, 100);
    let actual_hsla = hsla(128, 100, 100, 1.);
    let expected_rgba = BevyColor::rgba(1., 1., 1., 1.);
    let actual_rgb = rgb(255, 255, 255);
    let actual_rgba = rgba(255, 255, 255, 1.);

    assert_eq!(expected_hsla, actual_hsla.into());
    assert_eq!(expected_hsla, actual_hsl.into());
    assert_eq!(expected_rgba, actual_rgba.into());
    assert_eq!(expected_rgba, actual_rgb.into());
}
