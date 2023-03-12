/*

number types we might end up using?

integers:
    u8      for indices
    i8?     not sure?
    u16     for indices
    i16     for bigger calculations?
    i32?    added because it's weird to have a 32 bit fixed point type but no 32-bit integer

fixed point numbers:
    u8.8    for typical unsigned fixed point math
    i8.8    for typical signed fixed point math
    u0.16   for higher-precision fixed point math
    i16.16? for high precision with an integer component?
*/

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct i8p8(i16);

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct u8p8(u16);

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct u16p16(u32);

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct i16p16(i32);

impl i8p8 {
    pub fn from_raw(r: i16) -> i8p8 {
        i8p8(r)
    }

    pub fn from_float(f: f64) -> i8p8 {
        i8p8((f * 2.0_f64.powf(8.0)) as i16)
    }

    pub fn into_float(self) -> f64 {
        self.0 as f64 / 2.0_f64.powf(8.0)
    }

    pub fn from_i16p16_truncated(a: i16p16) -> i8p8 {
        i8p8(((a.0 >> 8) & 0xffff) as i16)
    }

    pub fn full_mul(self, b: i8p8) -> i16p16 {
        i16p16(i32::from(self.0) * i32::from(b.0))
    }
}

impl u8p8 {
    pub fn from_raw(r: u16) -> u8p8 {
        u8p8(r)
    }

    pub fn from_u16p16_truncated(a: u16p16) -> u8p8 {
        u8p8(((a.0 >> 8) & 0xffff) as u16)
    }

    pub fn full_mul(self, b: u8p8) -> u16p16 {
        u16p16(u32::from(self.0) * u32::from(b.0))
    }
}

impl u16p16 {
    pub fn from_raw(r: u32) -> u16p16 {
        u16p16(r)
    }
}

// unary op(s)

use std::ops::*;

impl Neg for i8p8 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        i8p8(-self.0)
    }
}

// binary op(s)

impl Add for i8p8 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        i8p8(self.0 + other.0)
    }
}

impl Sub for i8p8 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        i8p8(self.0 - other.0)
    }
}

impl Mul for i8p8 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        i8p8::from_i16p16_truncated(self.full_mul(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        show_float_as_fixed(0.25);
        show_float_as_fixed(0.5);
        show_float_as_fixed(1.0);
        show_float_as_fixed(2.0);
        show_float_as_fixed(39.0);
        show_float_as_fixed(0.1005);
        show_float_as_fixed(0.1016);
        show_float_as_fixed(-0.1016);
    }

    #[test]
    fn test_1() {
        let n = 15;
        let f = n as f64;
        for i in 0..=n {
            show_float_as_fixed(i as f64 / f);
        }
    }

    fn show_float_as_fixed(f: f64) {
        let a = i8p8::from_float(f);

        let q = ((a.0 as u16 & 0xff00) >> 8) as i16;

        let r = a.0 & 0xff;

        let o = a.into_float();

        eprintln!("0b{q:08b}.{r:08b}\t0x{q:02x}.{r:02x}\t{f:?} -> {o:?}");
    }
}
