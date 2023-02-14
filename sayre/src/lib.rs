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

pub mod gfx;
pub mod mob;
pub mod snd;
pub mod vector;

pub use vfc;

use vfc::*;

pub fn main() -> Vfc {
    let mut vfc = Vfc::new();

    /*
    #[rustfmt::skip]
    let tileset: vfc::Tileset = [
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
    */

    /*
    #[rustfmt::skip]
    let tileset = vfc::Tileset{
        pixel_data: [
            [
                [
                    0b00011000,
                    0b01111110,
                    0b01111110,
                    0b10111101,
                    0b10011001,
                    0b01000010,
                    0b01100110,
                    0b00011000,
                ].map(|byte| vfc::PaletteIndex(byte)),
            ],
            [
                [
                    0b00000000,
                    0b00011000,
                    0b00111100,
                    0b01111110,
                    0b01111110,
                    0b00111100,
                    0b00011000,
                    0b00000000,
                ].map(|byte| vfc::PaletteIndex(byte)),
            ],
            [
                [
                    0b00000000,
                    0b00000000,
                    0b00111100,
                    0b00011100,
                    0b00001100,
                    0b00001100,
                    0b00001000,
                    0b00000000,
                ].map(|byte| vfc::PaletteIndex(byte)),
            ],
        ]
    };
    */

    /*
    #[rustfmt::skip]
    let palette = Palette::default().iter().append(// vec.splice?
        [
            //~ Rgb(0x33, 0x77, 0xdd), // background color
            Rgb(0x11, 0xbb, 0xdd), // background color
            Rgb(0x22, 0x33, 0x44),
            Rgb(0x44, 0x77, 0x11),
            Rgb(0x66, 0xaa, 0x55),
            Rgb(0x99, 0xdd, 0x66),
            Rgb(0xaa, 0x33, 0x22),
            Rgb(0xdd, 0xbb, 0x66),
            Rgb(0xff, 0xff, 0xff),
            Rgb(0x00, 0x00, 0x00),
            Rgb(0x00, 0x00, 0x00),
            Rgb(0x00, 0x00, 0x00),
            Rgb(0x00, 0x00, 0x00),
            Rgb(0x00, 0x00, 0x00),
            Rgb(0x00, 0x00, 0x00),
            Rgb(0x00, 0x00, 0x00),
            Rgb(0x00, 0x00, 0x00),
        ].iter()
    );
    */

    #[rustfmt::skip]
    let tileset = vfc::Tileset{
        pixel_data: [
            [
                [
                    0b01111110,
                    0b01000010,
                    0b01111110,
                    0b01000100,
                    0b01111100,
                    0b01010000,
                    0b01110000,
                    0b01110000,
                ].map(|byte| vfc::PaletteIndex(byte)),
            ],
            [
                [
                    0b00000000,
                    0b00100000,
                    0b00100000,
                    0b00000000,
                    0b00000000,
                    0b00100000,
                    0b00100000,
                    0b00000000,
                ].map(|byte| vfc::PaletteIndex(byte)),
            ],
            [
                [
                    0b10000001,
                    0b00000000,
                    0b00000000,
                    0b00100000,
                    0b00100000,
                    0b00100000,
                    0b00100000,
                    0b11111111,
                ].map(|byte| vfc::PaletteIndex(byte)),
            ],
        ]
    };

    vfc.tileset = tileset;

    for i in 0..=255u8 {
        let spacing = 10;
        let columns = 16;

        let entry = vfc::OamEntry {
            x: (i % columns).wrapping_mul(spacing).wrapping_add(4),
            y: (i / columns).wrapping_mul(spacing).wrapping_add(1),
            rotation: Rotation(0),
            priority: 0,
            tile_index: TileIndex(0),

            // offset into the palette to find the colors for this object
            palette_offset: PaletteIndex(0),
        };

        vfc.oam.0[i as usize] = entry;
    }

    for i in 0..8 {
        /*
        let entry = vfc::OamEntry {
            x: (i % 4) * 16 + 16,
            y: (i / 4) * 16 + 16,
            rotation: Rotation(i),
            layer: 0,
            tile_index: TileIndex(0),

            // offset into the palette to find the colors for this object
            palette_offset: PaletteIndex(0),
        };

        vfc.oam.0[(i + 8) as usize] = entry;
        */
        vfc.oam.0[(i + 8) as usize].rotation = Rotation(i);
    }

    /*
    let p = [(); 256]
        .iter()
        .enumerate()
        .map(|(i, _)| vfc::RgbValue(i as u8, i as u8, i as u8))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    vfc.palette = vfc::Palette(p);
    */

    vfc.palette = Vfc::test_palette();

    let bg_color = vfc::PaletteIndex(15);

    vfc.palette.0[bg_color.0 as usize] = vfc::Rgb(0x33, 0x77, 0xdd);

    vfc.background_color = bg_color;

    vfc
}

pub fn next_frame(vfc: &mut Vfc) -> Vec<u32> {
    vfc.render_frame();

    as_argb_u32(&vfc.framebuffer)
}

fn as_argb_u32(framebuffer: &[vfc::Rgb; vfc::NUM_SCREEN_PIXELS]) -> Vec<u32> {
    framebuffer
        .map(|rgb| rgb.as_argb_u32())
        .into_iter()
        .collect()
}
