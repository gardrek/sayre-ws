use std::error::Error;

use vfc::Tileset;

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
                        bytes[pixel_y][plane_index];
                }
            }
        }
    }

    Ok(tileset)
}
