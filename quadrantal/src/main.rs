use std::error::Error;
use vfc::Tileset;

mod col;
mod tet;

use tet::Piece;

pub fn load_tileset_from_path(path: &str) -> Result<Tileset, Box<dyn Error>> {
    use image::io::Reader as ImageReader;
    use vfc::*;

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

                    //~ print!("{r} ");

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
                //~ println!();
            }
            //~ println!();
        }
        //~ println!();
    }

    //~ */
    Ok(tileset)
}

fn render_to_argb_u32(framebuffer: &[vfc::Rgb; vfc::NUM_SCREEN_PIXELS], target_buffer: &mut [u32]) {
    for (index, argb) in framebuffer.iter().map(|rgb| rgb.as_argb_u32()).enumerate() {
        target_buffer[index] = argb;
    }
}


fn clear_sprites(fc: &mut vfc::Vfc) {
    let range = 0..=63;
    for i in range {
        fc.oam.0[i].y = vfc::SCREEN_HEIGHT as u8;
    }
}

////////////////////////////////////////////////////////////////////////////////

fn main() {
    use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
    use vfc::*;

    let mut fc = Vfc::default();

    ////////////////

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

    fc.tileset = load_tileset_from_path("quad_tiles.png").unwrap();

    tet::init_playfield(&mut fc);

    ////////////////

    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Limit to max ~640 fps cause why not
    //~ window.limit_update_rate(Some(std::time::Duration::from_micros(1612)));

    let mut hidden_oam = OamTable::default();

    let mut start_time;

    let mut frametime_hist = std::collections::VecDeque::from(vec![]);

    for _ in 0..5 {
        frametime_hist.push_back(0);
    }

    //~ let mut frames = 0;

    //----\\ INITIAL GAME STATE //----\\

    let pieces = {
        let pieces_initial = (0..8)
            .map(|i| Piece::new_basic(i).shrink_wrap_square())
            .collect::<Vec<_>>();

        let pieces_with_rotations = (0..4)
            .map(|rotation| {
                pieces_initial
                    .iter()
                    .map(|piece| piece.rotate_by_quarter_angle(rotation).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        pieces_with_rotations
    };

    // current piece held by player (1..=7)
    let mut current_piece: usize = 0;

    // rotation of current piece (0..=3)
    let mut current_rotation: usize = 0;

    // offset, in tiles, to top-left corner of piece's bounding box
    let mut piece_x = 1;
    let mut piece_y = 1;

    tet::init_playfield(&mut fc);

    //----\\ MAIN LOOP //----\\

    while window.is_open() && !window.is_key_down(Key::Escape) {
        //----\\ SOME TIMING //----\\

        start_time = std::time::Instant::now();

        //----\\ INPUT //----\\

        if window.is_key_pressed(Key::F8, KeyRepeat::No) {
            std::mem::swap(&mut fc.oam, &mut hidden_oam);
        }

        if window.is_key_pressed(Key::F9, KeyRepeat::No) {
            fc.bg_layers[0].hidden = !fc.bg_layers[0].hidden;
        }

        if window.is_key_pressed(Key::F10, KeyRepeat::No) {
            fc.bg_layers[1].hidden = !fc.bg_layers[1].hidden;
        }

        if window.is_key_pressed(Key::F, KeyRepeat::No) {
            let piece = &pieces[current_rotation][current_piece];
            piece.lock(&mut fc, piece_x, piece_y);
        }

        if window.is_key_pressed(Key::K, KeyRepeat::No) {
            current_piece = current_piece.wrapping_add(1) % 8;
            let piece = &pieces[current_rotation][current_piece];
        }

        if window.is_key_pressed(Key::I, KeyRepeat::No) {
            current_piece = current_piece.wrapping_sub(1) % 8;
            let piece = &pieces[current_rotation][current_piece];
        }

        if window.is_key_pressed(Key::L, KeyRepeat::No) {
            current_rotation = current_rotation.wrapping_add(1) % 4;
            let piece = &pieces[current_rotation][current_piece];
        }

        if window.is_key_pressed(Key::J, KeyRepeat::No) {
            current_rotation = current_rotation.wrapping_sub(1) % 4;
        }

        //----\\ LOGIC //----\\

        //~ todo!()

        //----\\ RENDERING //----\\

        clear_sprites(&mut fc);

        let piece = &pieces[current_rotation][current_piece];
        piece.draw_as_sprites(&mut fc, piece_x * 8, piece_y * 8, 0);

        fc.render_frame();

        render_to_argb_u32(&mut fc.framebuffer, &mut buffer);

        // We unwrap here as we want this code to exit if it fails.
        // Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();

        let end_time = std::time::Instant::now();

        //----\\ MORE TIMING //----\\

        let instant_microsecs_per_frame = (end_time - start_time).as_micros();

        frametime_hist.pop_front().expect("fifo wrong length");

        frametime_hist.push_back(instant_microsecs_per_frame);

        let mut average_frametime = 0;
        for i in 0..5 {
            average_frametime += frametime_hist[i];
        }
        average_frametime /= 5;

        let average_fps = 1_000_000 / average_frametime;

        //~ eprintln!("{instant_microsecs_per_frame:?}\t{average_frametime:?}\t{average_fps}");
    }
}
