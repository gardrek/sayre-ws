use vfc::TileIndex;

use super::col;

const NUM_TET: usize = 8;
const TET_H: usize = 2;
const TET_W: usize = 4;

// (0) O I T J L S Z
#[rustfmt::skip]
const TETROMINOS_DATA: [[[u8; TET_W]; TET_H]; NUM_TET] = [
    [
        [ 1, 0, 0, 1, ],
        [ 1, 1, 1, 1, ],
    ],
    [
        [ 0, 1, 1, 0, ],
        [ 0, 1, 1, 0, ],
    ],
    [
        [ 0, 0, 0, 0, ],
        [ 1, 1, 1, 1, ],
    ],
    [
        [ 0, 0, 1, 0, ],
        [ 0, 1, 1, 1, ],
    ],
    [
        [ 0, 1, 0, 0, ],
        [ 0, 1, 1, 1, ],
    ],
    [
        [ 0, 0, 0, 1, ],
        [ 0, 1, 1, 1, ],
    ],
    [
        [ 0, 0, 1, 1, ],
        [ 0, 1, 1, 0, ],
    ],
    [
        [ 0, 1, 1, 0, ],
        [ 0, 0, 1, 1, ],
    ],
];

const fn make_tetrominos_from_data(
    data: [[[u8; TET_W]; TET_H]; NUM_TET],
) -> [[[TileIndex; TET_W]; TET_H]; NUM_TET] {
    let mut out = [[[TileIndex(0); TET_W]; TET_H]; NUM_TET];

    let mut piece = 0;
    while piece < NUM_TET {
        let mut yi = 0;
        while yi < TET_H {
            let mut xi = 0;
            while xi < TET_W {
                out[piece][yi][xi] = TileIndex(data[piece][yi][xi] * TILE_BLOCK.0);
                xi += 1;
            }
            yi += 1;
        }
        piece += 1;
    }

    return out;
}

const TETROMINOS: [[[TileIndex; TET_W]; TET_H]; NUM_TET] =
    make_tetrominos_from_data(TETROMINOS_DATA);

pub struct Piece {
    width: usize,
    height: usize,
    data: Vec<TileIndex>,
}

/*
enum TetrominoShape {
    O = 0,
    I,
    T,
    J,
    L,
    S,
    Z,
}
*/

impl Piece {
    pub fn new_from_array<const W: usize, const H: usize>(array: &[[TileIndex; W]; H]) -> Piece {
        let mut data = Vec::with_capacity(W * H);

        for yi in 0..H {
            for xi in 0..W {
                data.push(array[yi][xi]);
            }
        }

        Piece {
            width: W,
            height: H,
            data,
        }
    }

    pub fn new_empty(width: usize, height: usize) -> Piece {
        Piece {
            width,
            height,
            data: [TileIndex(0)].repeat(width * height),
        }
    }

    pub fn new_basic(shape_index: usize) -> Piece {
        Piece::new_from_array(&TETROMINOS[shape_index])
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn index_from_coords(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileIndex {
        let i = self.index_from_coords(x, y);

        self.data[i]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileIndex) {
        let i = self.index_from_coords(x, y);

        self.data[i] = tile
    }

    pub fn lock(&self, fc: &mut vfc::Vfc, x: usize, y: usize) {
        for yi in 0..self.height() {
            for xi in 0..self.width() {
                let tet_tile = self.get_tile(xi, yi);

                if col::tile_is_solid(tet_tile) {
                    poke_bg0(fc, x + xi, y + yi, tet_tile);
                }
            }
        }
    }

    pub fn draw_as_sprites(
        &self,
        fc: &mut vfc::Vfc,
        x: usize,
        y: usize,
        mut sprite_index_offset: usize,
    ) {
        for yi in 0..self.height() {
            for xi in 0..self.width() {
                let tet_tile = self.get_tile(xi, yi);

                if col::tile_is_solid(tet_tile) {
                    let sprite = &mut fc.oam.0[sprite_index_offset];

                    sprite.x = (x + xi * 8) as u8;
                    sprite.y = (y + yi * 8) as u8;

                    sprite.tile_index = tet_tile;

                    //~ sprite.attributes = todo!();

                    sprite_index_offset += 1;
                }
            }
        }
    }

    /// Centers the piece in the smallest square that fits it.
    /// Does not currently support spieces with multiple disconnected parts.
    pub fn shrink_wrap_square(self) -> Piece {
        let mut first_data = vec![];
        let mut width = self.width();
        let mut height = self.height();

        let mut rows = vec![];
        let mut columns = vec![];

        for yi in 0..self.height() {
            let mut accumulator = 0;
            for xi in 0..self.width() {
                accumulator |= self.get_tile(xi, yi).0;
            }
            if accumulator != 0 {
                rows.push(yi);
            } else {
                height -= 1;
            }
        }

        for xi in 0..self.width() {
            let mut accumulator = 0;
            for yi in 0..self.height() {
                accumulator |= self.get_tile(xi, yi).0;
            }
            if accumulator != 0 {
                columns.push(xi);
            } else {
                width -= 1;
            }
        }

        let mut yi = 0;
        for row in rows.iter() {
            let mut xi = 0;
            for column in columns.iter() {
                first_data.push(self.get_tile(*column, *row));

                xi += 1;
            }
            yi += 1;
        }

        /*
        let start_y = for yi in 0..self.height() {
            let mut accumulator = 0;
            for xi in 0..self.width() {
                accumulator |= self.get_tile(xi, yi).0;
            }
            if accumulator != 0 {
                break yi;
            }
        };

        let end_y = for yi in (0..self.height()).rev() {
            let mut accumulator = 0;
            for xi in 0..self.width() {
                accumulator |= self.get_tile(xi, yi).0;
            }
            if accumulator != 0 {
                break yi;
            }
        };
        */

        /*
            1 2 3 4 5
        1   0 x x x x
        2   0 0 x x x
        3   1 0 0 x x
        4   1 1 0 0 x
        5   2 1 1 0 0
        */

        let new_size = width.max(height);

        let x_start = (new_size - width) / 2;
        let y_start = (new_size - height) / 2;
        let x_end = x_start + width - 1;
        let y_end = y_start + height - 1;

        let mut data = vec![];

        for yi in 0..new_size {
            for xi in 0..new_size {
                data.push(
                    if xi >= x_start && xi <= x_end && yi >= y_start && yi <= y_end {
                        let xx = xi - x_start;
                        let yy = yi - y_start;

                        let i = yy * width + xx;

                        first_data[i]
                    } else {
                        TileIndex(0)
                    },
                );
            }
        }

        Piece {
            width: new_size,
            height: new_size,
            data,
        }
    }

    pub fn rotate_by_quarter_angle(&self, angle: usize) -> Option<Piece> {
        if self.width() != self.height() {
            return None;
        }

        Some(match angle {
            0 => self.flip_rotate(false, false, false),
            1 => self.flip_rotate(false, true, true),
            2 => self.flip_rotate(true, true, false),
            3 => self.flip_rotate(true, false, true),
            _ => return None,
        })
    }

    fn flip_rotate(&self, flip_x: bool, flip_y: bool, flip_diagonal: bool) -> Piece {
        let mut data = Vec::with_capacity(self.width * self.height);

        for tile_y in 0..self.height {
            for tile_x in 0..self.width {
                let (tile_x, tile_y) = if flip_diagonal {
                    (tile_y as usize, tile_x as usize)
                } else {
                    (tile_x as usize, tile_y as usize)
                };

                let tile_x = if flip_x {
                    self.width - 1 - tile_x
                } else {
                    tile_x
                };

                let tile_y = if flip_y {
                    self.height - 1 - tile_y
                } else {
                    tile_y
                };

                data.push(self.data[tile_y * self.width + tile_x]);
            }
        }

        Piece { data, ..*self }
    }
}

const FIELD_X: usize = 1;
const FIELD_Y: usize = 18;

const FIELD_WIDTH: usize = 10;
const FIELD_HEIGHT: usize = 32;

const TILE_EMPTY: TileIndex = TileIndex(0x00);
const TILE_CIELING: TileIndex = TileIndex(0x01);
const TILE_WALL: TileIndex = TileIndex(0x7f);
const TILE_BLOCK: TileIndex = TileIndex(0x80);

pub fn poke_bg0(fc: &mut vfc::Vfc, x: usize, y: usize, tile_index: TileIndex) {
    fc.bg_layers[0].tiles[(y % vfc::BG_HEIGHT) * vfc::BG_WIDTH + x % vfc::BG_WIDTH] = tile_index
}

pub fn peek_bg0(fc: &vfc::Vfc, x: usize, y: usize) -> TileIndex {
    fc.bg_layers[0].tiles[(y % vfc::BG_HEIGHT) * vfc::BG_WIDTH + x % vfc::BG_WIDTH]
}

pub fn init_playfield(fc: &mut vfc::Vfc) {
    //~ poke_bg0(fc, FIELD_X + 10, FIELD_Y, TileIndex(0x40 + 1));

    for yi in 0..FIELD_HEIGHT {
        // draw columns
        poke_bg0(fc, FIELD_X + FIELD_WIDTH, FIELD_Y + yi, TILE_WALL);
        poke_bg0(fc, FIELD_X - 1, FIELD_Y + yi, TILE_WALL);

        // draw empty field
        for xi in 0..FIELD_WIDTH {
            //~ poke_bg0(fc, FIELD_X + xi, FIELD_Y + yi, TileIndex((TILE_WALL.0 as usize + (xi + yi) % 2) as u8));
            poke_bg0(fc, FIELD_X + xi, FIELD_Y + yi, TILE_EMPTY);
        }
    }

    // draw floor and cieling
    for xi in 0..FIELD_WIDTH {
        poke_bg0(fc, FIELD_X + xi, FIELD_Y + FIELD_HEIGHT + 1, TILE_WALL);
        poke_bg0(
            fc,
            FIELD_X + xi,
            FIELD_Y + FIELD_HEIGHT / 2 + 1,
            TILE_CIELING,
        );
    }
}
