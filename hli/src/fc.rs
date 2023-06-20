use vfc::*;

fn get_index_from_coords(x: usize, y: usize) -> usize {
    (y % BG_HEIGHT) * BG_WIDTH + x % BG_WIDTH
}

pub fn poke_bg(bg: usize, fc: &mut Vfc, x: usize, y: usize, tile_index: TileIndex) {
    let i = get_index_from_coords(x, y);

    fc.bg_layers[bg].tiles[i] = tile_index;
}

pub fn poke_bg_rotation(bg: usize, fc: &mut Vfc, x: usize, y: usize, rotation: u8) {
    let i = get_index_from_coords(x, y);

    fc.bg_layers[bg].attributes[i].set_rotation(rotation);
}

pub fn peek_bg(bg: usize, fc: &Vfc, x: usize, y: usize) -> TileIndex {
    let i = get_index_from_coords(x, y);

    fc.bg_layers[bg].tiles[i]
}

pub fn poke_bg_palette(bg: usize, fc: &mut Vfc, x: usize, y: usize, palette_index: Subpalette) {
    let i = get_index_from_coords(x, y);

    fc.bg_layers[bg].attributes[i].set_palette(palette_index);
}

pub fn poke_main_bg(fc: &mut vfc::Vfc, x: usize, y: usize, tile_index: TileIndex) {
    poke_bg(0, fc, x, y, tile_index)
}

pub fn poke_main_rotation(fc: &mut vfc::Vfc, x: usize, y: usize, rotation: u8) {
    poke_bg_rotation(0, fc, x, y, rotation)
}

pub fn peek_main_bg(fc: &vfc::Vfc, x: usize, y: usize) -> TileIndex {
    peek_bg(0, fc, x, y)
}

pub fn peek_game_layer(fc: &vfc::Vfc, x: usize, y: usize) -> TileIndex {
    fc.bg_layers[0].tiles[(y % vfc::BG_HEIGHT) * vfc::BG_WIDTH + x % vfc::BG_WIDTH]
}

pub fn poke_aux_bg(fc: &mut vfc::Vfc, x: usize, y: usize, tile_index: TileIndex) {
    poke_bg(1, fc, x, y, tile_index)
}

pub fn draw_text(bg: usize, fc: &mut Vfc, x: usize, y: usize, string: &str) {
    for (xi, byte_ch) in string.bytes().enumerate() {
        if byte_ch >= 32 && byte_ch < 128 {
            poke_bg(bg, fc, x + xi, y, TileIndex(byte_ch + 32));
            poke_bg_palette(bg, fc, x + xi, y, Subpalette::new(0));
        } else {
            panic!();
        }
    }
}

pub fn paint_rect_palette(
    bg: usize,
    fc: &mut Vfc,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    palette: Subpalette,
) {
    for yi in y..(y.wrapping_add(h)) {
        for xi in x..(x.wrapping_add(w)) {
            poke_bg_palette(bg, fc, xi, yi, palette);
        }
    }
}

pub fn clear_rect_tiles(bg: usize, fc: &mut Vfc, x: usize, y: usize, w: usize, h: usize) {
    for yi in y..h {
        for xi in x..w {
            poke_bg(bg, fc, xi, yi, TileIndex(0));
        }
    }
}

pub fn clear_rect_palette(bg: usize, fc: &mut Vfc, x: usize, y: usize, w: usize, h: usize) {
    for yi in y..h {
        for xi in x..w {
            poke_bg_palette(bg, fc, xi, yi, Subpalette::new(0));
        }
    }
}

pub fn clear_bg_tiles(bg: usize, fc: &mut Vfc) {
    clear_rect_tiles(bg, fc, 0, 0, BG_WIDTH, BG_HEIGHT);
}

pub fn clear_bg_palette(bg: usize, fc: &mut Vfc) {
    clear_rect_palette(bg, fc, 0, 0, BG_WIDTH, BG_HEIGHT);
}

pub fn clear_line(bg: usize, fc: &mut Vfc, line: usize) {
    for xi in 0..BG_WIDTH {
        poke_bg(bg, fc, xi, line, TileIndex(0x0));
        //~ poke_bg_palette(bg, fc, xi, line, Subpalette::new(1));
    }
}

pub fn clear_sprites(fc: &mut Vfc) {
    let range = 0..=255;
    for i in range {
        fc.oam.0[i].hide();
    }
}
