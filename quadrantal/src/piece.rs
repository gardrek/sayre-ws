use vfc::Subpalette;
use vfc::TileIndex;

use super::col;
use super::tet;

const NUM_BASIC_PIECES_PLUS_ONE: usize = 8;
const TET_H: usize = 2;
const TET_W: usize = 4;
pub const TILE_ROW_LENGTH: usize = 16;

// (0) O I T J L S Z
#[rustfmt::skip]
const TETROMINOS_DATA: [[[u8; TET_W]; TET_H]; NUM_BASIC_PIECES_PLUS_ONE] = [
    [
        [ 1, 0, 1, 0, ],
        [ 0, 1, 0, 1, ],
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
        [ 0, 1, 0, 0, ],
        [ 1, 1, 1, 0, ],
    ],
    [
        [ 1, 0, 0, 0, ],
        [ 1, 1, 1, 0, ],
    ],
    [
        [ 0, 0, 1, 0, ],
        [ 1, 1, 1, 0, ],
    ],
    [
        [ 0, 1, 1, 0, ],
        [ 1, 1, 0, 0, ],
    ],
    [
        [ 1, 1, 0, 0, ],
        [ 0, 1, 1, 0, ],
    ],
];

const fn make_tetrominos_from_data(
    data: [[[u8; TET_W]; TET_H]; NUM_BASIC_PIECES_PLUS_ONE],
) -> [[[TileIndex; TET_W]; TET_H]; NUM_BASIC_PIECES_PLUS_ONE] {
    let mut out = [[[TileIndex(0); TET_W]; TET_H]; NUM_BASIC_PIECES_PLUS_ONE];

    let mut piece = 0;
    while piece < NUM_BASIC_PIECES_PLUS_ONE {
        let mut yi = 0;
        while yi < TET_H {
            let mut xi = 0;
            while xi < TET_W {
                let mino = data[piece][yi][xi];
                out[piece][yi][xi] = if mino > 0 {
                    let west = if xi == 0 { 0 } else { data[piece][yi][xi - 1] };
                    let east = if xi == TET_W - 1 { 0 } else { data[piece][yi][xi + 1] };
                    let north = if yi == 0 { 0 } else { data[piece][yi - 1][xi] };
                    let south = if yi == TET_H - 1 { 0 } else { data[piece][yi + 1][xi] };

                    let west_east = if west > 0 && east > 0 { 2 } else if west > 0 { 3 } else if east > 0 { 1 } else { 0 };
                    let north_south = if north > 0 && south > 0 { 2 } else if north > 0 { 3 } else if south > 0 { 1 } else { 0 };

                    let base = tet::TILE_BLOCKS[piece];

                    TileIndex(base + west_east + north_south * TILE_ROW_LENGTH as u8)
                } else {
                    TileIndex(0)
                };

                xi += 1;
            }
            yi += 1;
        }
        piece += 1;
    }

    return out;
}

const TETROMINOS: [[[TileIndex; TET_W]; TET_H]; NUM_BASIC_PIECES_PLUS_ONE] =
    make_tetrominos_from_data(TETROMINOS_DATA);

#[derive(Clone)]
pub struct Piece {
    width: usize,
    height: usize,
    data: Vec<TileIndex>,
    index: usize,
    subpalette: Subpalette,
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
    pub fn new_from_array<const W: usize, const H: usize>(
        array: &[[TileIndex; W]; H],
        index: usize,
        subpalette: Subpalette,
    ) -> Piece {
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
            index,
            subpalette,
        }
    }

    pub fn new_basic(shape_index: usize, subpalette: Subpalette) -> Piece {
        Piece::new_from_array(&TETROMINOS[shape_index], shape_index, subpalette)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn get_subpalette(&self, _x: usize, _y: usize) -> Subpalette {
        self.subpalette
    }

    fn index_from_coords(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileIndex {
        let i = self.index_from_coords(x, y);

        self.data[i]
    }

    /*
    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileIndex) {
        let i = self.index_from_coords(x, y);

        self.data[i] = tile
    }
    */

    /*
    pub fn debug_try_lock(&self, fc: &mut vfc::Vfc, x: usize, y: usize) -> bool {
        let success = !self.test_collision(fc, x, y);

        if success {
            self.force_lock(fc, x, y);
        }

        success
    }
    */

    pub fn force_lock(&self, fc: &mut vfc::Vfc, x: usize, y: usize) {
        for yi in 0..self.height() {
            for xi in 0..self.width() {
                let tet_tile = self.get_tile(xi, yi);

                if col::tile_is_solid(tet_tile) {
                    let (fx, fy) = (x + xi, y + yi);

                    tet::poke_game_layer(fc, fx, fy, tet_tile);

                    let subpalette = self.subpalette;

                    tet::poke_game_layer_palette(fc, fx, fy, subpalette)
                }
            }
        }
    }

    pub fn force_lock_xor(
        &self,
        fc: &mut vfc::Vfc,
        x: usize,
        y: usize,
        no_overlap_tile: TileIndex,
        overlap_tile: TileIndex,
    ) {
        for yi in 0..self.height() {
            for xi in 0..self.width() {
                let piece_tile = self.get_tile(xi, yi);

                let bg_tile = tet::peek_game_layer(fc, x + xi, y + yi);

                if col::tile_is_solid(piece_tile) {
                    let t = if col::tile_is_solid(bg_tile) {
                        overlap_tile
                    } else {
                        no_overlap_tile
                    };

                    tet::poke_game_layer_palette(fc, x + xi, y + yi, self.get_subpalette(xi, yi));
                    tet::poke_game_layer(fc, x + xi, y + yi, t);
                }
            }
        }
    }

    pub fn test_collision(&self, fc: &vfc::Vfc, x: usize, y: usize) -> bool {
        col::test_bg0_collision(fc, x, y, &self)
    }

    pub fn draw_as_sprites(
        &self,
        fc: &mut vfc::Vfc,
        x: usize,
        y: usize,
        mut sprite_index_offset: usize,
        tile_offset: Option<u8>,
        palette_override: Option<Subpalette>,
    ) -> usize {
        for yi in 0..self.height() {
            for xi in 0..self.width() {
                let tet_tile = self.get_tile(xi, yi);

                if col::tile_is_solid(tet_tile) {
                    let sprite = &mut fc.oam.0[sprite_index_offset];

                    sprite.x = (x + xi * 8) as u8;
                    sprite.y = (y + yi * 8) as u8;

                    sprite.tile_index = match tile_offset {
                        Some(tile_offset) => {
                            let (_base, west_east, north_south) = combined_tile_to_base_and_connections(tet_tile);
                            TileIndex(tile_offset.wrapping_add(west_east + north_south * TILE_ROW_LENGTH as u8))
                        }
                        None => tet_tile,
                    };

                    let palette = match palette_override {
                        Some(subpalette) => subpalette,
                        None => self.subpalette,
                    };

                    sprite.attributes = vfc::TileAttributes::default();
                    sprite.attributes.set_palette(palette);

                    //~ sprite.attributes.set_rotation(1);

                    sprite_index_offset += 1;

                    sprite_index_offset &= 0xff;
                }
            }
        }

        sprite_index_offset
    }

    /// Centers the piece in the smallest square that fits it.
    /// Does not currently support pieces with multiple disconnected parts.
    pub fn shrink_wrap_square(&self) -> Piece {
        let mut first_data = vec![];
        let mut width = self.width();
        let mut height = self.height();
        let index = self.index();
        let subpalette = self.subpalette;

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

        //~ let mut yi = 0;
        for row in rows.iter() {
            //~ let mut xi = 0;
            for column in columns.iter() {
                first_data.push(self.get_tile(*column, *row));

                //~ xi += 1;
            }
            //~ yi += 1;
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
            index,
            subpalette,
        }
    }

    /// Make each tile connect to those around it
    pub fn _make_connections(&mut self) {
        let x_bit_0 = ();
        let y_bit_1 = ();
        eprintln!("{:?} {:?}", x_bit_0, y_bit_1);
        todo!()
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

                data.push(flip_rotate_tile(self.data[tile_y * self.width + tile_x], flip_x, flip_y, flip_diagonal));
            }
        }

        Piece { data, ..*self }
    }
}

pub fn combined_tile_to_base_and_connections(tile: TileIndex) -> (u8, u8, u8) {
    let u8_tile = tile.0;

    if u8_tile == 0 { return (u8_tile, 0, 0) }

    let west_east = u8_tile % (TILE_ROW_LENGTH as u8 / 4);

    let north_south = u8_tile % (TILE_ROW_LENGTH as u8 * 4) / TILE_ROW_LENGTH as u8;

    let base_column = u8_tile % (TILE_ROW_LENGTH as u8);

    let base_row = u8_tile / (TILE_ROW_LENGTH as u8);

    let base = (base_row / 4 * 4) << 4 | (base_column / 4 * 4);

    (base, west_east, north_south)
}

fn flip_rotate_tile(tile: TileIndex, flip_x: bool, flip_y: bool, flip_diagonal: bool) -> TileIndex {
    if tile.0 == 0 { return tile }

    let (base, mut west_east, mut north_south) = combined_tile_to_base_and_connections(tile);

    if flip_x {
        west_east = match west_east {
            0 => 0,
            1 => 3,
            2 => 2,
            3 => 1,
            _ => unreachable!(),
        };
    }

    if flip_y {
        north_south = match north_south {
            0 => 0,
            1 => 3,
            2 => 2,
            3 => 1,
            _ => unreachable!(),
        };
    }

    if flip_diagonal {
        (west_east, north_south) = (north_south, west_east);
    }

    TileIndex(base + west_east + north_south * TILE_ROW_LENGTH as u8)
}

pub struct FloatingPiece {
    rotation: usize,

    rotated_pieces: [Piece; 4],

    // offset, in tiles, to top-left corner of piece's bounding box
    position: (usize, usize),
}

impl FloatingPiece {
    pub fn new(piece: Piece, position: (usize, usize)) -> FloatingPiece {
        let piece = piece.shrink_wrap_square();

        let rotated_pieces = (0..4)
            .map(|rotation| piece.rotate_by_quarter_angle(rotation).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| unreachable!());

        FloatingPiece {
            position,
            rotation: 0,
            rotated_pieces,
        }
    }

    pub fn rotated_piece(&self, rotation: usize) -> &Piece {
        &self.rotated_pieces[rotation]
    }

    pub fn get_unrotated_piece(&self) -> &Piece {
        &self.rotated_pieces[0]
    }

    pub fn get_piece(&self) -> &Piece {
        //~ &self.rotated_pieces[self.rotation]
        self.rotated_piece(self.rotation)
    }

    pub fn position(&self) -> &(usize, usize) {
        &self.position
    }

    pub fn shadow_move(
        &self,
        fc: &vfc::Vfc,
        dir: (isize, isize),
        repeats: usize,
    ) -> (usize, usize) {
        if dir == (0, 0) {
            return self.position;
        };

        let mut prev_position = self.position;
        let mut next_position = self.position;
        let mut hit = false;

        for _ in 0..repeats {
            prev_position = next_position;
            next_position = (
                (next_position.0 as isize + dir.0) as usize % 32,
                (next_position.1 as isize + dir.1) as usize % 32,
            );

            hit = self
                .get_piece()
                .test_collision(fc, next_position.0, next_position.1);

            if hit {
                break;
            }
        }

        if hit {
            prev_position
        } else {
            next_position
        }
    }

    /// tries to move a piece from its current location using a movement vector
    pub fn try_move(&mut self, fc: &vfc::Vfc, dir: (isize, isize)) -> bool {
        if dir == (0, 0) {
            return true;
        };

        let next_position = (
            (self.position.0 as isize + dir.0) as usize % 32,
            (self.position.1 as isize + dir.1) as usize % 32,
        );

        let hit = self
            .get_piece()
            .test_collision(fc, next_position.0, next_position.1);

        if !hit {
            self.position = next_position;
        }

        !hit
    }

    pub fn rotate_move(&mut self, quarter_turns: isize, next_position: (usize, usize)) {
        let next_rotation = (self.rotation as isize + quarter_turns) as usize % 4;

        self.rotation = next_rotation;

        self.position = next_position;
    }

    pub fn try_rotate_with_kicks(
        &self,
        fc: &vfc::Vfc,
        rotation_vector: isize,
    ) -> Option<(usize, usize)> {
        let clockwise = if rotation_vector == -1 {
            false
        } else if rotation_vector == 1 {
            true
        } else {
            return Some((self.position.0, self.position.1));
        };

        let clockwise_kicks = [
            (0, 0),
            (-1, 0),
            (1, 0),
            (0, 1),
            (-1, 1),
            (1, 1),
            (0, -1),
            (-1, -1),
            (1, -1),
            (-2, 0),
            (2, 0),
            (-1, -2),
            (1, -2),
        ];

        const LEN: usize = 13;

        let anticlockwise_kicks: [(isize, isize); LEN] = (0..LEN)
            .map(|i| {
                let k = clockwise_kicks[i];
                (-k.0, k.1)
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| unreachable!());

        let all_kicks = [anticlockwise_kicks, clockwise_kicks];

        let kicks = all_kicks[clockwise as usize];

        let rotation = if clockwise { 1 } else { -1isize };

        let rotated_piece =
            self.rotated_piece((self.rotation as isize + rotation as isize) as usize % 4);

        for k in kicks.iter() {
            let next_position = (
                self.position.0 as isize + k.0,
                self.position.1 as isize + k.1,
            );

            let hit = rotated_piece.test_collision(
                fc,
                next_position.0 as usize,
                next_position.1 as usize,
            );

            if !hit {
                return Some((next_position.0 as usize, next_position.1 as usize));
            }
        }

        None
    }

    pub fn shadow_drop(&self, fc: &vfc::Vfc) -> (usize, usize) {
        self.shadow_move(fc, (0, 1), 32)
    }

    pub fn sonic_drop(&mut self, fc: &vfc::Vfc) -> bool {
        if !self.try_move(fc, (0, 1)) {
            return false;
        }

        for _ in 0..tet::FIELD_HEIGHT {
            if !self.try_move(fc, (0, 1)) {
                break;
            }
        }

        true
    }

    pub fn soft_drop(&mut self, fc: &mut vfc::Vfc) -> bool {
        let hit = !self.try_move(fc, (0, 1));

        if hit {
            self.lock(fc)
        }

        hit
    }

    pub fn lock(&self, fc: &mut vfc::Vfc) {
        self.get_piece()
            .force_lock(fc, self.position.0, self.position.1);
    }

    pub fn reset_and_test_overlap(&mut self, fc: &mut vfc::Vfc, next_piece: Piece) -> bool {
        self.reset(next_piece);

        let collided = self
            .get_piece()
            .test_collision(fc, self.position().0, self.position().1);

        if collided {
            let pc = self.get_piece();
            let position = self.position();
            pc.force_lock_xor(
                fc,
                position.0,
                position.1,
                TileIndex(tet::TILE_GAME_OVER_BLOCK.0 + 1),
                tet::TILE_GAME_OVER_BLOCK,
            );
        }

        collided
    }

    pub fn reset(&mut self, piece: Piece) {
        let rotated_pieces = (0..4)
            .map(|rotation| {
                piece
                    .shrink_wrap_square()
                    .rotate_by_quarter_angle(rotation)
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| unreachable!());

        self.rotation = 0;
        self.rotated_pieces = rotated_pieces;

        // centering
        let position_x = tet::FIELD_X + (tet::FIELD_WIDTH - piece.width()) / 2;

        let position = 'outer: {
            for row in 0..tet::FIELD_HEIGHT {
                let piece = &self.rotated_pieces[0];

                for yi in 0..piece.height() {
                    for xi in 0..piece.width() {
                        let pc_tile = piece.get_tile(xi, yi);

                        if yi + row > tet::CEILING_HEIGHT && col::tile_is_solid(pc_tile) {
                            break 'outer (position_x, tet::FIELD_Y + row - 2);
                        }
                    }
                }
            }

            unreachable!(); //i hope
        };

        self.position = position;
    }
}
