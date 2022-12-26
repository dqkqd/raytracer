use std::ops::{Add, BitAnd, Div, Mul, Sub};

use crate::util::equal;

pub(super) const MAX_COLOR: u8 = 255;

// some useful color
pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        let convert = |p: f64| -> u8 {
            let p = p * MAX_COLOR as f64;
            let c = p.round() as i32;
            let c = std::cmp::max(c, 0);
            let c = std::cmp::min(c, MAX_COLOR as i32);
            c as u8
        };
        (convert(self.r), convert(self.g), convert(self.b))
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        equal(self.r, other.r) && equal(self.g, other.g) && equal(self.b, other.b)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Color {
        Color {
            r: value.0 as f64 / MAX_COLOR as f64,
            g: value.1 as f64 / MAX_COLOR as f64,
            b: value.2 as f64 / MAX_COLOR as f64,
        }
    }
}

type HexString = str;
impl From<&HexString> for Color {
    fn from(value: &HexString) -> Color {
        assert_eq!(value.len(), 7);
        assert!(value.starts_with('#'));
        let convert_to_hex = |src| u8::from_str_radix(src, 16).expect("Expect hex string");
        let r = convert_to_hex(&value[1..3]);
        let g = convert_to_hex(&value[3..5]);
        let b = convert_to_hex(&value[5..7]);
        Color::from((r, g, b))
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Div<f64> for Color {
    type Output = Color;
    fn div(self, rhs: f64) -> Self::Output {
        Color::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl BitAnd<Color> for Color {
    type Output = Color;
    fn bitand(self, rhs: Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let c = Color::new(0.1, 0.2, 0.3);
        assert_eq!(c.r(), 0.1);
        assert_eq!(c.g(), 0.2);
        assert_eq!(c.b(), 0.3);
    }

    #[test]
    fn test_add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_sub() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn test_mul_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn test_wise_color() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(c1 & c2, Color::new(0.9, 0.2, 0.04));
    }

    #[test]
    fn test_convert_from_u8() {
        let color = Color::from((100, 150, 200));
        let expected = Color::new(0.39215687, 0.5882353, 0.78431374);
        assert_eq!(color, expected);
    }

    #[test]
    fn test_convert_from_hex() {
        let color = Color::from("#6496C8");
        let expected = Color::new(0.39215687, 0.5882353, 0.78431374);
        assert_eq!(color, expected);
    }
}
