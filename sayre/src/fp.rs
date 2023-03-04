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

#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct i8p8(i16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct u8p8(u16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct u16p16(u32);

impl i8p8 {
    pub fn from_raw(r: i16) -> i8p8 {
        i8p8(r)
    }
}

impl u8p8 {
    pub fn from_raw(r: u16) -> u8p8 {
        u8p8(r)
    }
}

impl u16p16 {
    pub fn from_raw(r: u32) -> u16p16 {
        u16p16(r)
    }
}

pub fn u8p8_full_mul(a: u8p8, b: u8p8) -> u16p16 {
    u16p16(u32::from(a.0) * u32::from(b.0))
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
