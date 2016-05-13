#[derive(Debug, Copy, Clone)]
pub struct Rgba { pub r: u8, pub g: u8, pub b: u8, pub a: u8 }

impl Rgba {
    pub fn from_i32(rgba: i32) -> Rgba {
        Rgba {
            r: ((rgba >> 24) & 0xFF) as u8,
            g: ((rgba >> 16) & 0xFF) as u8,
            b: ((rgba >>  8) & 0xFF) as u8,
            a: ((rgba >>  0) & 0xFF) as u8,
        }
    }
}

pub trait ToLuma {
    fn to_luma(&self) -> u8;
}

pub trait ToRgba {
    fn to_rgba(&self) -> Rgba;
}

impl ToLuma for u8 {
    fn to_luma(&self) -> u8 {
        *self
    }
}

impl ToRgba for u8 {
    fn to_rgba(&self) -> Rgba {
        let v = *self;
        Rgba { r: v, g: v, b: v, a: v }
    }
}
