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


mod gfx;
mod mob;
mod snd;
mod vector;

use vfc::Vfc;

fn main() {
    let mut vfc = Vfc::new();

    #[rustfmt::skip]
    let tileset: vfc::TileSet = [
        0b00011000,
        0b01111110,
        0b01111110,
        0b10111101,
        0b10011001,
        0b01000010,
        0b01100110,
        0b00011000,

        0b00000000,
        0b00011000,
        0b00111100,
        0b01111110,
        0b01111110,
        0b00111100,
        0b00011000,
        0b00000000,

        0b00000000,
        0b00000000,
        0b00111100,
        0b00011100,
        0b00001100,
        0b00001100,
        0b00001000,
        0b00000000,
    ]
    .into();

    vfc.render_frame();
}
