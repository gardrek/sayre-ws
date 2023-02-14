// screen constants
pub const PIXEL_WIDTH: usize = 192;
pub const PIXEL_HEIGHT: usize = 160; // 144?
pub const NUM_SCREEN_PIXELS: usize = PIXEL_WIDTH * PIXEL_HEIGHT;

// tile constants
pub const NUM_PLANES: usize = 3;
pub const BYTES_PER_TILE_PLANE: usize = TILE_WIDTH * TILE_HEIGHT / 8;
pub const NUM_TILES: usize = 1;

// oam constants
pub const NUM_OAM_ENTRIES: usize = 256;
pub const TILE_WIDTH: usize = 8;
pub const TILE_HEIGHT: usize = 8;

// background layer constants
pub const NUM_BG_LAYERS: usize = 3;
pub const BG_WIDTH: usize = 64;
pub const BG_HEIGHT: usize = 64;
pub const NUM_BG_TILES: usize = BG_WIDTH * BG_HEIGHT;
pub const NUM_BG_PRIORITY_LEVELS: usize = 2;

// misc constants
pub const NUM_PALETTE_ENTRIES: usize = 256;
pub const NUM_OBJECT_PRIORITY_LEVELS: usize = 4;

pub struct Vfc {
    // stuff goes here
    pub framebuffer: [Rgb; NUM_SCREEN_PIXELS],
    //~ pub indexed_framebuffer: [PaletteIndex; NUM_SCREEN_PIXELS],
    pub oam: OamTable,
    pub palette: Palette,
    pub background_color: PaletteIndex,
    pub tileset: Tileset,
    pub bg_layers: [BgLayer; NUM_BG_LAYERS],
}

#[repr(transparent)]
pub struct OamTable(pub [OamEntry; NUM_OAM_ENTRIES]);

#[derive(Debug, Clone)]
pub struct OamEntry {
    pub x: u8,
    pub y: u8,
    pub rotation: Rotation,
    pub priority: u8,
    pub tile_index: TileIndex,

    // offset into the palette to find the colors for this object
    pub palette_offset: PaletteIndex,
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

#[derive(Debug, Default)]
pub enum LayerType {
    #[default]
    BgColor,
    BgLayer(u8),
    Oam(OamIndex),
}

#[derive(Debug, Default)]
pub struct LayerHit {
    layer: LayerType,
    hit: PaletteIndex,
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TileIndex(pub u8);

/*
impl std::ops::Index<TileIndex> for Tileset {
    type Output = [&[PaletteIndex; BYTES_PER_TILE_PLANE]; NUM_PLANES];

    fn index(&self, index: TileIndex) -> &Self::Output {
        &self.get_tile(index)
    }
}
*/

#[repr(transparent)]
#[derive(Clone)]
pub struct Palette(pub [Rgb; NUM_PALETTE_ENTRIES]);

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PaletteIndex(pub u8);

impl std::ops::Index<PaletteIndex> for Palette {
    type Output = Rgb;

    fn index(&self, index: PaletteIndex) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rotation(pub u8);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn as_argb_u32(&self) -> u32 {
        u32::from_be_bytes([255, self.0, self.1, self.2])
    }
}

pub struct Tileset {
    pub pixel_data: [[[PaletteIndex; BYTES_PER_TILE_PLANE]; NUM_TILES]; NUM_PLANES],
}

#[derive(Debug)]
pub struct BgLayer {
    priority: u8,
    tiles: [TileIndex; NUM_BG_TILES],
}

impl OamEntry {
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
    */

    // get a pixel in local coords
    pub fn get_pixel(&self, tileset: &Tileset, pixel_x: u8, pixel_y: u8) -> PaletteIndex {
        let pixel_x = pixel_x % TILE_WIDTH as u8;
        let pixel_y = pixel_y % TILE_HEIGHT as u8;

        let tile = tileset.get_tile(self.tile_index);

        let flip_x = self.rotation.0 & 1 != 0;
        let flip_y = (self.rotation.0 >> 1) & 1 != 0;
        let flip_diagonal = (self.rotation.0 >> 2) & 1 != 0;

        let (pixel_x, pixel_y) = if flip_diagonal {
            (pixel_y as usize, pixel_x as usize)
        } else {
            (pixel_x as usize, pixel_y as usize)
        };

        let pixel_x = if flip_x {
            pixel_x
        } else {
            TILE_WIDTH - 1 - pixel_x
        };

        let pixel_y = if flip_y {
            TILE_WIDTH - 1 - pixel_y
        } else {
            pixel_y
        };

        // NOTE: does not work with tiles wider than 8 pixels
        let mut pixel = 0;
        for plane_index in 0..NUM_PLANES {
            pixel = (pixel << 1) | ((tile[plane_index][pixel_y].0 as usize >> pixel_x) & 1);
        }

        PaletteIndex(pixel as u8)
    }

    // get a pixel in screen coords
    pub fn get_pixel_global(&self, tileset: &Tileset, screen_x: u8, screen_y: u8) -> PaletteIndex {
        let local_x = screen_x.wrapping_sub(self.x);
        let local_y = screen_y.wrapping_sub(self.y);

        self.get_pixel(tileset, local_x, local_y)
    }
}

impl Tileset {
    pub fn new() -> Tileset {
        //~ let pixel_data = [(); BYTES_PER_TILE * NUM_TILES].map(|_| PaletteIndex::default()),

        let pixel_data =
            [[[(); BYTES_PER_TILE_PLANE].map(|_| PaletteIndex::default()); NUM_TILES]; NUM_PLANES];

        Tileset { pixel_data }
    }

    pub fn get_tile(
        &self,
        tile_index: TileIndex,
    ) -> [&[PaletteIndex; BYTES_PER_TILE_PLANE]; NUM_PLANES] {
        (0..NUM_PLANES)
            .map(|plane_index| &self.pixel_data[plane_index][tile_index.0 as usize])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| unreachable!())
    }

    pub fn write_tile(
        &mut self,
        tile_index: TileIndex,
        tile: [[PaletteIndex; BYTES_PER_TILE_PLANE]; NUM_PLANES],
    ) {
        for (plane_index, plane) in tile.iter().enumerate() {
            self.pixel_data[plane_index][tile_index.0 as usize] = *plane;
        }
    }
}

impl Vfc {
    pub fn new() -> Vfc {
        Vfc { ..Vfc::default() }
    }

    pub fn test_palette() -> Palette {
        Palette(
            [(); 256]
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    let x = i as u8 % 16;
                    let y = i as u8 / 16;
                    //~ Rgb(x ^ y, 0, 0)
                    Rgb(x * 16 as u8, y * 16 as u8, i as u8)
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| unreachable!()),
        )
    }

    pub fn render_frame(&mut self) {
        for yi in 0..PIXEL_HEIGHT {
            for xi in 0..PIXEL_WIDTH {
                let xi = xi as u8;
                let yi = yi as u8;

                let pixel_index = Vfc::get_fb_pixel_index(xi, yi);

                let LayerHit {
                    hit: index,
                    layer: _layer,
                } = self.get_top_pixel(xi, yi);

                self.framebuffer[pixel_index] = self.palette[index];
            }
        }
    }

    pub fn get_fb_pixel_index(x: u8, y: u8) -> usize {
        let x = x as usize;
        let y = y as usize;
        x + PIXEL_WIDTH * y
    }

    pub fn bg_layer_hit(&self, layer_index: u8, _x: u8, _y: u8) -> Option<LayerHit> {
        todo!()
    }

    pub fn get_top_pixel(&self, pixel_x: u8, pixel_y: u8) -> LayerHit {
        //~ let object_list = self.get_objects_at_pixel(NUM_OAM_ENTRIES, pixel_x, pixel_y);
        let object_list = self.get_objects_at_pixel(16, pixel_x, pixel_y);

        let oam_hit = 'l: {
            for index in object_list.iter() {
                let oam_entry = &self.oam[*index];
                let pixel = oam_entry.get_pixel_global(&self.tileset, pixel_x, pixel_y);
                if pixel != PaletteIndex(0) {
                    break 'l Some(LayerHit {
                        hit: pixel,
                        layer: LayerType::Oam(*index),
                    });
                }
            }
            None
        };

        let bg_hit = self.get_bg_layer_hit_at_pixel(pixel_x, pixel_y);

        let hits = [oam_hit, bg_hit];

        for hit in hits {
            if let Some(r) = hit {
                return r;
            }
        }

        LayerHit {
            hit: self.background_color,
            layer: LayerType::BgColor,
        }
    }

    //*
    pub fn get_layer_at_pixel(&self, x: u8, y: u8) -> LayerType {
        use LayerType::*;

        let bg_hit = self.background_hit(x, y);

        let object_hit = self.object_hit(x, y);

        'l: {
            for priority in 0..NUM_OBJECT_PRIORITY_LEVELS {
                if let Some(object_index) = object_hit {
                    let object = &self.oam[object_index];
                    if priority as u8 == object.priority {
                        break 'l Oam(object_index);
                    }
                } else if let Some(l) = bg_hit {
                    break 'l BgLayer(l);
                }
            }
            BgColor
        }
    }
    // */
    // write to a slice/vec all objects whose bounding box includes this pixel,
    // in layer order, up to a maximum number
    pub fn get_objects_at_pixel(&self, max_len: usize, pixel_x: u8, pixel_y: u8) -> Vec<OamIndex> {
        //~ pub fn get_objects_at_pixel(&self, output: &mut [OamEntry], pixel_x: u8, pixel_y: u8) {
        let mut vec = vec![];
        //~ let max_len = output.len();
        if max_len == 0 {
            return vec;
        }
        //~ let mut index = 0;
        'l: {
            for priority in (0..NUM_OBJECT_PRIORITY_LEVELS).rev() {
                for (index, oam_entry) in self.oam.0.iter().enumerate() {
                    if priority as u8 == oam_entry.priority
                        && oam_entry.bounding_box_contains_pixel(pixel_x, pixel_y)
                    {
                        vec.push(OamIndex(index as u8));
                        //~ output[index] = oam_entry.clone();
                        //~ index += 1;
                        if index >= max_len - 1 {
                            break 'l;
                        }
                    }
                }
            }
        }
        vec
    }

    pub fn get_bg_layer_hit_at_pixel(&self, pixel_x: u8, pixel_y: u8) -> Option<LayerHit> {
        // priorities reversed because priority 0 is the farthest back
        for priority in (0..NUM_BG_PRIORITY_LEVELS).rev() {
            for (index, layer) in self.bg_layers.0.iter().enumerate() {
                if priority as u8 == layer.priority {
                    if let Some(hit) = self.bg_layer_hit(index, pixel_x, pixel_y) {
                        return Some(hit)
                    }
                }
            }
        }

        None
    }

    pub fn get_pixel_rgb(&self, color_index: PaletteIndex, object_index: LayerType) -> Rgb {
        todo!("get_pixel_rgb(&self, {color_index:?}: PaletteIndex, {object_index:?}: LayerType) -> Rgb")
    }

    pub fn background_hit(&self, pixel_x: u8, pixel_y: u8) -> Option<u8> {
        todo!("background_hit(&self, {pixel_x:?}, {pixel_y:?}) -> Option<u8>")
    }

    pub fn object_hit_pix(&self, pixel_x: u8, pixel_y: u8) -> bool {
        todo!("object_hit_pix(&self, {pixel_x:?}, {pixel_y:?}) -> bool")
    }

    pub fn object_hit(&self, pixel_x: u8, pixel_y: u8) -> Option<OamIndex> {
        let mut object_index = 0;
        loop {
            let object = &self.oam[OamIndex(object_index)];

            if object.bounding_box_contains_pixel(pixel_x, pixel_y)
                && self.object_hit_pix(pixel_x, pixel_y)
            {
                break Some(OamIndex(object_index as u8));
            } else if object_index < NUM_OAM_ENTRIES as u8 {
                object_index += 1;
                continue;
            } else {
                break None;
            }
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self([(); NUM_PALETTE_ENTRIES].map(|_| Rgb::default()))
    }
}

impl Default for OamTable {
    fn default() -> Self {
        Self([(); NUM_OAM_ENTRIES].map(|_| OamEntry::default()))
    }
}

impl Default for BgLayer {
    fn default() -> Self {
        Self { tiles: [(); NUM_BG_TILES].map(|_| TileIndex::default()), priority: 0 }
    }
}

impl Default for Vfc {
    fn default() -> Self {
        Self {
            oam: OamTable::default(),
            framebuffer: [(); NUM_SCREEN_PIXELS].map(|_| Rgb::default()),
            palette: Palette::default(),
            background_color: PaletteIndex::default(),
            tileset: Tileset::new(),
            bg_layers: Default::default(),
        }
    }
}

impl Default for OamEntry {
    fn default() -> Self {
        Self {
            x: 0,
            y: PIXEL_HEIGHT as u8,
            rotation: Rotation(0),
            priority: 0,
            tile_index: TileIndex(0),
            palette_offset: PaletteIndex(0),
        }
    }
}
