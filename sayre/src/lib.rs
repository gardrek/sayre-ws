//

pub mod constants;

pub mod prng;

pub mod fp;
pub mod vector;

pub mod item;
pub mod mob;
pub mod mob_drop;

pub mod gfx;
pub mod snd;

pub use vfc;

use vfc::*;

use image::io::Reader as ImageReader;

pub fn main() -> Vfc {
    let mut prng = prng::Prng::new([1, 0]);

    let mut fc = Vfc::new();

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

    let preview_palette_array = [
        //~ Rgb(0x33, 0x77, 0xdd), // background color
        Rgb::new(0x11, 0xbb, 0xdd), // background color
        Rgb::new(0x22, 0x33, 0x44),
        Rgb::new(0x44, 0x77, 0x11),
        Rgb::new(0x66, 0xaa, 0x55),
        Rgb::new(0x99, 0xdd, 0x66),
        Rgb::new(0xaa, 0x33, 0x22),
        Rgb::new(0xdd, 0xbb, 0x66),
        Rgb::new(0xff, 0xff, 0xff),
    ];

    fc.palette = Palette::new(
        (0..NUM_PALETTE_ENTRIES)
            .map(|index| {
                if index < 8 {
                    preview_palette_array[index]
                } else {
                    Rgb::default()
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| unreachable!()),
    );

    /*
    #[rustfmt::skip]
    let tile_x_raw = [
        [
            0b00000000,
            0b01000100,
            0b00101000,
            0b00010000,
            0b00101000,
            0b01000100,
            0b00000000,
            0b00000000,
        ],
        [
            0b00000001,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000001,
        ],
        [
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
        ],
    ];

    #[rustfmt::skip]
    let tile_f_raw = [
        [
            0b10000001,
            0b00000000,
            0b00000000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b10001111,
        ],
        [
            0b10000001,
            0b00111100,
            0b00100000,
            0b00011000,
            0b00000000,
            0b00100000,
            0b00100000,
            0b10001111,
        ],
        [
            0b01111110,
            0b01000010,
            0b01111110,
            0b01000100,
            0b01111100,
            0b01010000,
            0b01110000,
            0b01110000,
        ],
    ];

    #[rustfmt::skip]
    let tile_circle_raw = [
        [
            0b00011000,
            0b01100110,
            0b01000010,
            0b10000001,
            0b10000001,
            0b01000010,
            0b01100110,
            0b00011000,
        ],
        [
            0b00000000,
            0b00011000,
            0b00111100,
            0b01111110,
            0b01111110,
            0b00111100,
            0b00011000,
            0b00000000,
        ],
        [
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
        ],
    ];

    let tiles: [_; NUM_TILES] = [tile_f_raw, tile_x_raw, tile_circle_raw, tile_x_raw];

    let pixel_data = (0..NUM_PLANES)
        .map(|plane_index| {
            tiles
                .iter()
                .map(|tile| tile[plane_index].map(|byte| vfc::PaletteIndex(byte)))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| unreachable!())
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap_or_else(|_| unreachable!());

    fc.tileset = Tileset {
        pixel_data,
        ..Tileset::default()
    };
    */

    fc.tileset = load_tileset_from_path("test_tiles.png").unwrap();

    for i in 0..=255 {
        //~ fc.bg_layers[0].tiles[i] = vfc::TileIndex(109);
        fc.bg_layers[0].tiles[i] = vfc::TileIndex((prng.next().unwrap() & 0xff) as u8);
    }

    for i in 0..=((NUM_OAM_ENTRIES - 1) as u8) {
        //~ let spacing = 10;
        let spacing = 10;
        let columns = 16;

        let tile_x = i % columns;
        let tile_y = i / columns;

        let entry = vfc::OamEntry {
            x: tile_x.wrapping_mul(spacing).wrapping_add(4),
            y: tile_y.wrapping_mul(spacing).wrapping_add(1),
            tile_index: TileIndex((tile_x / 2 + tile_y / 2) % 2 + 2),
            //~ tile_index: ,
            attributes: TileAttributes::default(),
        };

        fc.oam.0[i as usize] = entry;
    }

    for i in 0..8 {
        /*
        let entry = vfc::OamEntry {
            x: (i % 4) * 16 + 16,
            y: (i / 4) * 16 + 16,
            rotation: Rotation(i),
            priority: 0,
            tile_index: TileIndex(0),

            // offset into the palette to find the colors for this object
            palette_offset: PaletteIndex(0),
        };

        fc.oam.0[(i + 8) as usize] = entry;
        // */
        fc.oam.0[(i + 8) as usize].attributes.set_rotation(i);
    }

    /*
    let p = [(); 256]
        .iter()
        .enumerate()
        .map(|(i, _)| vfc::RgbValue(i as u8, i as u8, i as u8))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    fc.palette = vfc::Palette(p);
    */

    //~ fc.palette = Vfc::test_palette();

    //~ let bg_color = vfc::PaletteIndex(15);

    //~ fc.palette.0[bg_color.0 as usize] = vfc::Rgb::new(0x33, 0x77, 0xdd);

    //~ fc.background_color = bg_color;

    fc
}

pub fn next_frame(fc: &mut Vfc) -> Vec<u32> {
    fc.render_frame();

    as_argb_u32(&fc.framebuffer)
}

pub fn as_argb_u32(framebuffer: &[vfc::Rgb; vfc::NUM_SCREEN_PIXELS]) -> Vec<u32> {
    framebuffer
        .map(|rgb| rgb.as_argb_u32())
        .into_iter()
        .collect()
}

pub fn render_to_argb_u32(
    framebuffer: &[vfc::Rgb; vfc::NUM_SCREEN_PIXELS],
    target_buffer: &mut [u32],
) {
    for (index, argb) in framebuffer.iter().map(|rgb| rgb.as_argb_u32()).enumerate() {
        target_buffer[index] = argb;
    }
}

pub fn load_tileset_from_path(path: &str) -> Result<Tileset, Box<dyn std::error::Error>> {
    let mut tileset = Tileset::default();

    let raw_img = ImageReader::open(path)?.decode()?;

    // first, we load the image
    let img = raw_img.into_rgba8();
    // check the dimensions
    let (image_width, image_height) = img.dimensions();

    let tile_columns = image_width as usize / TILE_WIDTH; // (rounds down)
    let tile_rows = image_height as usize / TILE_HEIGHT; // (rounds down)

    //~ /*
    // iterate each tile index

    for column in 0..tile_columns {
        for row in 0..tile_rows {
            let tile_index = column + row * tile_rows;

            let tile_x = column * TILE_WIDTH;
            let tile_y = row * TILE_HEIGHT;

            // NOTE: this only works for a TILE_WIDTH of 8
            for pixel_y in 0..TILE_HEIGHT {
                let mut bytes = [
                    [0, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0],
                ];

                for pixel_x in 0..TILE_WIDTH {
                    let pixel = img.get_pixel((pixel_x + tile_x) as u32, (pixel_y + tile_y) as u32);

                    let [r, _g, _b, a] = pixel.0;

                    print!("{r} ");

                    //~ let color_index = 0;
                    //~ /*
                    let color_index = if a < 128 {
                        0
                    } else {
                        r / (256 / TILE_PALETTE_SIZE) as u8
                    };
                    //~ */
                    for plane_index in 0..NUM_PLANES {
                        let bit = (color_index & (1 << plane_index)) >> plane_index;

                        bytes[pixel_y][NUM_PLANES - 1 - plane_index] |= bit << pixel_x;
                    }
                }

                for plane_index in 0..NUM_PLANES {
                    tileset.pixel_data[plane_index][tile_index][pixel_y] =
                        PaletteIndex(bytes[pixel_y][plane_index]);
                }
                println!();
            }
            println!();

            //~ insert_tile();
        }
        println!();
    }

    //~ */
    Ok(tileset)
}
