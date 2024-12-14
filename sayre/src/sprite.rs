// sayre/sprite.rs

use vfc::OamEntry;

#[derive(Debug, Default)]
pub struct SpriteList {
    list: Vec<OamEntry>,
}

pub fn test_list(x: u8, y: u8) -> SpriteList {
    let mut list = SpriteList::default();

    let id = vfc::TileAttributes::default();
    //~ let flipped = id.with_rotation(1);
    let p0 = id.with_priority(0);
    let p1 = id.with_priority(1);
    let p2 = id.with_priority(2);
    let p3 = id.with_priority(3);

    list.add_sprite_centered(
        x as i32,
        y as i32,
        2,
        2,
        &[
            vfc::TileIndex(0x10),
            vfc::TileIndex(0x11),
            vfc::TileIndex(0x12),
            vfc::TileIndex(0x13),
        ],
        &[
            p0,
            p1,
            p2,
            p3,
        ],
    );

    /*
    list.add_sprite_centered(
        30,
        30,
        2,
        2,
        &[
            vfc::TileIndex(0xc1),
            vfc::TileIndex(0xc2),
            vfc::TileIndex(0xc3),
            vfc::TileIndex(0xc4),
        ],
        &[
            p1,
            flipped,
            p3,
            flipped,
        ],
    );
    */

    list
}

impl SpriteList {
    pub fn add_sprite_centered(
        &mut self,
        center_x: i32,
        center_y: i32,
        w: i32,
        h: i32,
        tiles: &[vfc::TileIndex],
        attributes: &[vfc::TileAttributes],
    ) {
        let pixel_w = w * vfc::TILE_WIDTH as i32;
        let pixel_h = h * vfc::TILE_HEIGHT as i32;

        for yi in 0..w {
            for xi in 0..h {
                let x = ((center_x - pixel_w / 2 + xi * vfc::TILE_WIDTH as i32) & 0xff) as u8;
                let y = ((center_y - pixel_h / 2 + yi * vfc::TILE_HEIGHT as i32) & 0xff) as u8;

                let i: usize = (yi * w + xi).try_into().unwrap();

                let tile_index = tiles[i];

                let attributes = attributes[i].clone();

                self.list.push(OamEntry {
                    x,
                    y,
                    tile_index,
                    attributes,
                });
            }
        }
    }

    pub fn render(&self, offset: u8, table: &mut vfc::OamTable) {
        for (i, obj) in self.list.iter().enumerate() {
            table[vfc::OamIndex(offset.wrapping_add((i & 0xff) as u8))] = obj.clone();
        }
    }

    pub fn render_partial(&self, offset: u8, start: u8, length: u8, table: &mut vfc::OamTable) {
        for i in 0..length {
            let i = (i & 0xff) as u8;
            let len = self.list.len();
            let obj = &self.list[(start.wrapping_add(i) as usize) % len];
            let oam_index = offset.wrapping_add(i);
            table[vfc::OamIndex(oam_index)] = obj.clone();
        }
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}
