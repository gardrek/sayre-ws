use vfc::Subpalette;
use vfc::TileIndex;

const FIELD_CLEAR_COLOR: Subpalette = Subpalette::new(0);

pub const FIELD_X: usize = 7;
//~ pub const FIELD_Y: usize = 20;
pub const FIELD_Y: usize = 21;

pub const FIELD_WIDTH: usize = 10;
pub const FIELD_HEIGHT: usize = 31;
pub const CEILING_HEIGHT: usize = FIELD_HEIGHT / 2;
pub const TOP_VISIBLE_ROW: usize = CEILING_HEIGHT - 3;
//~ pub const SCORE_Y: usize = FIELD_Y + FIELD_HEIGHT;
pub const SCORE_Y: usize = FIELD_Y + TOP_VISIBLE_ROW - 1;

pub const TILE_EMPTY: TileIndex = TileIndex(0x00);
//~ pub const TILE_EMPTY: TileIndex = TileIndex(0x66); // capital F for testing rotation
//~ pub const TILE_EMPTY: TileIndex = TileIndex(0x10);
pub const TILE_CEILING: TileIndex = TileIndex(0x01);
pub const TILE_WALL: TileIndex = TileIndex(0x7f);
pub const TILE_BLOCK: TileIndex = TileIndex(0x80);
pub const TILE_SHADOW_OFFSET: TileIndex = TileIndex(0x84); // (0x80 + 0x84) % 0x100 == 4
pub const TILE_PIECE_ICON: TileIndex = TileIndex(0x08);
pub const TILE_ROW_CLEAR: TileIndex = TileIndex(0x18);
pub const TILE_GAME_OVER_BLOCK: TileIndex = TileIndex(0x20);

pub fn poke_bg(bg: usize, fc: &mut vfc::Vfc, x: usize, y: usize, tile_index: TileIndex) {
    fc.bg_layers[bg].tiles[(y % vfc::BG_HEIGHT) * vfc::BG_WIDTH + x % vfc::BG_WIDTH] = tile_index
}

pub fn poke_game_layer(fc: &mut vfc::Vfc, x: usize, y: usize, tile_index: TileIndex) {
    poke_bg(0, fc, x, y, tile_index)
}

pub fn peek_game_layer(fc: &vfc::Vfc, x: usize, y: usize) -> TileIndex {
    fc.bg_layers[0].tiles[(y % vfc::BG_HEIGHT) * vfc::BG_WIDTH + x % vfc::BG_WIDTH]
}

pub fn poke_menu_layer(fc: &mut vfc::Vfc, x: usize, y: usize, tile_index: TileIndex) {
    poke_bg(1, fc, x, y, tile_index)
}

/*
pub fn poke_bg_attribute(
    bg: usize,
    fc: &mut vfc::Vfc,
    x: usize,
    y: usize,
    attr: vfc::TileAttributes,
) {
    fc.bg_layers[bg].attributes[(y % vfc::BG_HEIGHT) * vfc::BG_WIDTH + x % vfc::BG_WIDTH] = attr
}
*/

pub fn poke_game_layer_palette(fc: &mut vfc::Vfc, x: usize, y: usize, palette_index: Subpalette) {
    poke_bg_palette(0, fc, x, y, palette_index)
}

pub fn peek_game_layer_palette(fc: &mut vfc::Vfc, x: usize, y: usize) -> Subpalette {
    peek_bg_palette(0, fc, x, y)
}

pub fn poke_bg_rotation(bg: usize, fc: &mut vfc::Vfc, x: usize, y: usize, rotation: u8) {
    let i = (y % vfc::BG_HEIGHT) * vfc::BG_WIDTH + x % vfc::BG_WIDTH; //get_index_from_coords(x, y);

    fc.bg_layers[bg].attributes[i].set_rotation(rotation);
}

pub fn poke_bg_palette(
    bg: usize,
    fc: &mut vfc::Vfc,
    x: usize,
    y: usize,
    palette_index: Subpalette,
) {
    let mut attr =
        //~ &mut fc.bg_layers[bg].attributes[(y % vfc::BG_HEIGHT) * vfc::BG_WIDTH + x % vfc::BG_WIDTH];
        fc.bg_layers[bg].attributes[(y % vfc::BG_HEIGHT) * vfc::BG_WIDTH + x % vfc::BG_WIDTH].clone();
    attr.set_palette(palette_index);
    fc.bg_layers[bg].attributes[(y % vfc::BG_HEIGHT) * vfc::BG_WIDTH + x % vfc::BG_WIDTH] = attr;
}

pub fn peek_bg_palette(bg: usize, fc: &mut vfc::Vfc, x: usize, y: usize) -> Subpalette {
    fc.bg_layers[bg].attributes[(y % vfc::BG_HEIGHT) * vfc::BG_WIDTH + x % vfc::BG_WIDTH]
        .get_palette()
}

pub fn draw_text(bg: usize, fc: &mut vfc::Vfc, x: usize, y: usize, string: &str) {
    for (xi, byte_ch) in string.bytes().enumerate() {
        if byte_ch >= 32 && byte_ch < 128 {
            poke_bg(bg, fc, x + xi, y, TileIndex(byte_ch + 32));
        } else {
            panic!();
        }
    }
}

pub fn init_playfield(fc: &mut vfc::Vfc) {
    //~ poke_game_layer(fc, FIELD_X + 10, FIELD_Y, TileIndex(0x40 + 1));

    for yi in 0..=FIELD_HEIGHT {
        // draw columns
        let y = FIELD_Y + yi;
        let left_x = FIELD_X - 1;
        let right_x = FIELD_X + FIELD_WIDTH;

        poke_game_layer(fc, left_x, y, TILE_WALL);
        poke_game_layer(fc, right_x, y, TILE_WALL);

        poke_bg_palette(0, fc, left_x, y, FIELD_CLEAR_COLOR);
        poke_bg_palette(0, fc, right_x, y, FIELD_CLEAR_COLOR);
    }

    clear_playfield(fc);

    // draw floor and cieling
    for xi in 0..FIELD_WIDTH {
        poke_game_layer(fc, FIELD_X + xi, FIELD_Y + FIELD_HEIGHT, TILE_WALL);
    }
}

pub fn clear_playfield(fc: &mut vfc::Vfc) {
    for yi in 0..FIELD_HEIGHT {
        // draw empty field
        for xi in 0..FIELD_WIDTH {
            //~ poke_game_layer(fc, FIELD_X + xi, FIELD_Y + yi, TileIndex((TILE_WALL.0 as usize + (xi + yi) % 2) as u8));
            poke_bg(0, fc, FIELD_X + xi, FIELD_Y + yi, TILE_EMPTY);
            poke_game_layer_palette(fc, FIELD_X + xi, FIELD_Y + yi, FIELD_CLEAR_COLOR);
            poke_bg_rotation(0, fc, FIELD_X + xi, FIELD_Y + yi, ((xi + yi) % 8) as u8);
        }
    }

    for xi in 0..FIELD_WIDTH {
        poke_bg(0, fc, FIELD_X + xi, FIELD_Y + CEILING_HEIGHT, TILE_CEILING);
        poke_game_layer_palette(
            fc,
            FIELD_X + xi,
            FIELD_Y + CEILING_HEIGHT,
            FIELD_CLEAR_COLOR,
        );

        poke_bg(1, fc, FIELD_X + xi, FIELD_Y + TOP_VISIBLE_ROW, TILE_EMPTY);
    }
}

pub fn clear_text_layer(fc: &mut vfc::Vfc) {
    for yi in -1..(FIELD_HEIGHT as isize + 1) {
        for xi in -1..(FIELD_WIDTH as isize + 1) {
            poke_bg(
                1,
                fc,
                ((FIELD_X as isize + xi) % 32) as usize,
                ((FIELD_Y as isize + yi) % 32) as usize,
                TILE_EMPTY,
            );
        }
    }
}
