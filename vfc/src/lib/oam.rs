use crate::constants::*;
use crate::{TileAttributes, TileIndex};

#[repr(transparent)]
pub struct OamTable(pub [OamEntry; NUM_OAM_ENTRIES]);

#[derive(Debug, Clone)]
pub struct OamEntry {
    pub x: u8,
    pub y: u8,
    pub tile_index: TileIndex,

    // offset into the palette to find the colors for this object
    // TODO: it should be multiplied by 8 to get the actual offset
    //~ pub palette_offset: PaletteIndex,
    //~ pub rotation: Rotation,
    //~ pub priority: u8,
    pub attributes: TileAttributes,
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct OamIndex(pub u8);

impl std::ops::Index<OamIndex> for OamTable {
    type Output = OamEntry;

    fn index(&self, index: OamIndex) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl std::ops::IndexMut<OamIndex> for OamTable {
    fn index_mut(&mut self, index: OamIndex) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

impl Default for OamTable {
    fn default() -> Self {
        Self([(); NUM_OAM_ENTRIES].map(|_| OamEntry::default()))
    }
}

impl Default for OamEntry {
    fn default() -> Self {
        Self {
            x: 0,
            y: SCREEN_HEIGHT as u8,
            tile_index: TileIndex(0),
            attributes: TileAttributes::oam_default(),
        }
    }
}

impl OamEntry {
    pub fn new(x: u8, y: u8, tile_index: TileIndex, attributes: TileAttributes) -> Self {
        Self {
            x,
            y,
            tile_index,
            attributes,
        }
    }

    pub fn bounding_box_contains_pixel(&self, x: u8, y: u8) -> bool {
        let left = self.x;
        let right = self.x.wrapping_add(TILE_WIDTH as u8);
        let top = self.y;
        let bottom = self.y.wrapping_add(TILE_HEIGHT as u8);

        let horizontal = if left <= right {
            x >= left && x < right
        } else {
            x >= left || x < right
        };

        let vertical = if top <= bottom {
            y >= top && y < bottom
        } else {
            y >= top || y < bottom
        };

        horizontal && vertical
    }

    /*
    pub fn poke_pixel()
        // FIXME: support non-8x8 tiles

        for plane in tile {
            for byte in plane {
                for i in 0..8 {
                    let bit = byte >> i) & 0b0000_0001;

                }
            }
        }
    }

    fn get_pixel(&self, tileset: &Tileset, pixel_x: u8, pixel_y: u8) -> PaletteIndex {
        let tile = tileset.get_tile(self.tile_index);

        tile.get_pixel(pixel_x, pixel_y)
    }

    // get a pixel in screen coords
    fn get_pixel_global(&self, tileset: &Tileset, screen_x: u8, screen_y: u8) -> PaletteIndex {
        let local_x = screen_x.wrapping_sub(self.x);
        let local_y = screen_y.wrapping_sub(self.y);

        self.get_pixel(tileset, local_x, local_y)
    }
    */

    pub fn hide(&mut self) {
        self.y = SCREEN_HEIGHT as u8;
    }
}
