use vfc::*;

pub fn poke_bg(bg: usize, fc: &mut vfc::Vfc, x: usize, y: usize, tile_index: TileIndex) {
    fc.bg_layers[bg].tiles[(y % vfc::BG_HEIGHT) * vfc::BG_WIDTH + x % vfc::BG_WIDTH] = tile_index
}

/*
pub fn poke_game_layer(fc: &mut vfc::Vfc, x: usize, y: usize, tile_index: TileIndex) {
    poke_bg(0, fc, x, y, tile_index)
}

pub fn peek_game_layer(fc: &vfc::Vfc, x: usize, y: usize) -> TileIndex {
    fc.bg_layers[0].tiles[(y % vfc::BG_HEIGHT) * vfc::BG_WIDTH + x % vfc::BG_WIDTH]
}

pub fn poke_menu_layer(fc: &mut vfc::Vfc, x: usize, y: usize, tile_index: TileIndex) {
    poke_bg(1, fc, x, y, tile_index)
}
*/

pub fn draw_text(bg: usize, fc: &mut vfc::Vfc, x: usize, y: usize, string: &str) {
    for (xi, byte_ch) in string.bytes().enumerate() {
        if byte_ch >= 32 && byte_ch < 128 {
            poke_bg(bg, fc, x + xi, y, TileIndex(byte_ch + 32));
        } else {
            panic!();
        }
    }
}

pub fn clear_bg(bg: usize, fc: &mut vfc::Vfc) {
    for yi in 0..BG_HEIGHT {
        for xi in 0..BG_WIDTH {
            poke_bg(bg, fc, xi, yi, TileIndex(0));
        }
    }
}

pub fn clear_line(bg: usize, fc: &mut vfc::Vfc, line: usize) {
    for xi in 0..BG_WIDTH {
        poke_bg(bg, fc, xi, line, TileIndex(0));
    }
}
