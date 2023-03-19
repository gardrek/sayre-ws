use super::piece::Piece;
use super::tet::peek_game_layer;
use vfc::TileIndex;

const COLLIDE_TEST_MASK: u8 = 0b1100_0000;

const COLLIDE_EMPTY: u8 = 0b0000_0000;
const COLLIDE_WALL: u8 = 0b0100_0000;
const COLLIDE_BLOCK_A: u8 = 0b1000_0000;
const COLLIDE_BLOCK_B: u8 = 0b1100_0000;

fn collision_test(a: u8, mask: u8) -> bool {
    ((a & COLLIDE_TEST_MASK) ^ mask) == 0
}

fn tile_test(a: TileIndex, mask: u8) -> bool {
    collision_test(a.0, mask)
}

pub fn tile_is_empty(a: TileIndex) -> bool {
    tile_test(a, COLLIDE_EMPTY)
}

pub fn tile_is_wall(a: TileIndex) -> bool {
    tile_test(a, COLLIDE_WALL)
}

pub fn tile_is_block(a: TileIndex) -> bool {
    tile_test(a, COLLIDE_BLOCK_A) || tile_test(a, COLLIDE_BLOCK_B)
}

pub fn tile_is_solid(a: TileIndex) -> bool {
    !tile_is_empty(a)
}

pub fn test_bg0_collision(fc: &vfc::Vfc, x: usize, y: usize, piece: &Piece) -> bool {
    'outer: {
        for yi in 0..piece.height() {
            for xi in 0..piece.width() {
                let bg_tile = peek_game_layer(fc, x + xi, y + yi);

                if tile_is_solid(bg_tile) {
                    let tet_tile = piece.get_tile(xi, yi);

                    if tile_is_solid(tet_tile) {
                        break 'outer true;
                    }
                }
            }
        }

        false
    }
}
