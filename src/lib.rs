#![allow(dead_code, non_camel_case_types, clippy::excessive_precision)]
//! this crate simplifies the process of working with different color spaces. it consists of two
//! parts: color structs, and the color trait.
//!
//! # structs
//!
//! the structs in this module represent a color in a particular color space. all of them can be
//! cast into each other, as well as casting to and from a [f32; 3]
//!
//! ```
//! use tinycolor::srgb;
//!
//! let color0 = srgb{r: 1.0, g: 0.5, b: 0.25};
//! let color1: srgb = [1.0, 0.5, 0.25].into();
//!
//! assert_eq!(color0, color1);
//! ```
//!
//! # the color trait
//!
//! every color struct implements the color trait. the color trait ensures that a struct that
//! implements it can be cast to all the colors. when writing functions that take a color as an argument,
//! using a generic color allows the caller to store their colors in whatever format they would
//! like.
//!
//! ```
//! use tinycolor::{Color, srgb, rgb};
//!
//! fn any_color_as_rgb<T: Color>(color: T) -> rgb {
//!     color.into()
//! }
//!
//! let color_srgb = srgb::WHITE;
//! let color_rgb = any_color_as_rgb(color_srgb);
//!
//! assert_eq!(color_rgb, rgb::from(color_srgb));
//! ```

/// any struct that implements this trait must implement Into for all color structs in this
/// module.
pub trait Color: Into<srgb> + Into<rgb> + Into<oklab> {}

/// a color in the srgb color space
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct srgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl srgb {
    pub const WHITE: srgb = srgb {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    pub const BLACK: srgb = srgb {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    pub const RED: srgb = srgb {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };

    pub const YELLOW: srgb = srgb {
        r: 1.0,
        g: 1.0,
        b: 0.0,
    };

    pub const GREEN: srgb = srgb {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };

    pub const AQUA: srgb = srgb {
        r: 0.0,
        g: 1.0,
        b: 1.0,
    };

    pub const BLUE: srgb = srgb {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };

    pub const PURPLE: srgb = srgb {
        r: 1.0,
        g: 0.0,
        b: 1.0,
    };
}
impl Color for srgb {}

impl From<[f32; 3]> for srgb {
    fn from(value: [f32; 3]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

impl From<srgb> for [f32; 3] {
    fn from(value: srgb) -> Self {
        [value.r, value.g, value.b]
    }
}

impl From<rgb> for srgb {
    fn from(value: rgb) -> Self {
        Self {
            r: rgb::to_linear(value.r),
            g: rgb::to_linear(value.g),
            b: rgb::to_linear(value.b),
        }
    }
}

impl From<oklab> for srgb {
    fn from(value: oklab) -> Self {
        let linear: rgb = value.into();
        linear.into()
    }
}

/// a color in the linear rgb color space
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl rgb {
    fn from_linear(x: f32) -> f32 {
        if x >= 0.0031308 {
            (1.055) * x.powf(1.0 / 2.4) - 0.055
        } else {
            12.92 * x
        }
    }

    fn to_linear(x: f32) -> f32 {
        if x >= 0.04045 {
            ((x + 0.055) / (1.0 + 0.055)).powf(2.4)
        } else {
            x / 12.92
        }
    }
}
impl Color for rgb {}

impl From<[f32; 3]> for rgb {
    fn from(value: [f32; 3]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

impl From<rgb> for [f32; 3] {
    fn from(value: rgb) -> Self {
        [value.r, value.g, value.b]
    }
}

impl From<srgb> for rgb {
    fn from(value: srgb) -> Self {
        Self {
            r: rgb::from_linear(value.r),
            g: rgb::from_linear(value.g),
            b: rgb::from_linear(value.b),
        }
    }
}

impl From<oklab> for rgb {
    fn from(value: oklab) -> Self {
        let l_ = value.l + 0.3963377774 * value.a + 0.2158037573 * value.b;
        let m_ = value.l - 0.1055613458 * value.a - 0.0638541728 * value.b;
        let s_ = value.l - 0.0894841775 * value.a - 1.2914855480 * value.b;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        Self {
            r: 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            g: -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            b: -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
        }
    }
}

/// a color in the oklab color space
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct oklab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}
impl Color for oklab {}

impl From<[f32; 3]> for oklab {
    fn from(value: [f32; 3]) -> Self {
        Self {
            l: value[0],
            a: value[1],
            b: value[2],
        }
    }
}

impl From<oklab> for [f32; 3] {
    fn from(value: oklab) -> Self {
        [value.l, value.a, value.b]
    }
}

impl From<rgb> for oklab {
    fn from(value: rgb) -> Self {
        let l = 0.4122214708 * value.r + 0.5363325363 * value.g + 0.0514459929 * value.b;
        let m = 0.2119034982 * value.r + 0.6806995451 * value.g + 0.1073969566 * value.b;
        let s = 0.0883024619 * value.r + 0.2817188376 * value.g + 0.6299787005 * value.b;

        let l_ = f32::cbrt(l);
        let m_ = f32::cbrt(m);
        let s_ = f32::cbrt(s);

        Self {
            l: 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
            a: 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
            b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
        }
    }
}

impl From<srgb> for oklab {
    fn from(value: srgb) -> Self {
        let linear: rgb = value.into();
        linear.into()
    }
}
