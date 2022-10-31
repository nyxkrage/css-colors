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
        formatter.write_str("a string in the format of rrggbb")
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
        formatter.write_str("a string in the format of rrggbbaa")
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
mod tests {
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
