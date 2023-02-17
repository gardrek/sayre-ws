// screen constants
pub const SCREEN_WIDTH: usize = 192;
pub const SCREEN_HEIGHT: usize = 160; // 144?
pub const NUM_SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

// tile constants
pub const NUM_PLANES: usize = 3;
pub const BYTES_PER_TILE_PLANE: usize = TILE_WIDTH * TILE_HEIGHT / 8;
pub const NUM_TILES: usize = 4;

// oam constants
pub const NUM_OAM_ENTRIES: usize = 256;
pub const TILE_SIZE: usize = 8;
pub const TILE_WIDTH: usize = TILE_SIZE;
pub const TILE_HEIGHT: usize = TILE_SIZE;

// background layer constants
pub const NUM_BG_LAYERS: usize = 3;
pub const BG_SIZE: usize = 32;
pub const BG_WIDTH: usize = BG_SIZE;
pub const BG_HEIGHT: usize = BG_SIZE;
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

// TODO: eventually we'll pack palette, rotation, and priority into one byte
// priority
//  |  Rotation
//  |    |  palette
//  |    |     |
// [+] [ + ] [ + ]
// 7 6 5 4 3 2 1 0
#[derive(Debug, Clone)]
pub struct OamEntry {
    pub x: u8,
    pub y: u8,
    pub priority: u8,
    pub tile_index: TileIndex,

    pub rotation: Rotation,
    // offset into the palette to find the colors for this object
    // it's multiplied by 8 to get the actual offset
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

impl std::ops::IndexMut<OamIndex> for OamTable {
    fn index_mut(&mut self, index: OamIndex) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

#[derive(Debug, Default, Clone)]
pub enum LayerType {
    #[default]
    BgColor,
    BgLayer(u8),
    Oam(OamIndex),
}

#[derive(Debug, Default, Clone)]
pub struct LayerHit {
    layer: LayerType,
    hit: PaletteIndex,
    priority: u8,
}

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

// TODO: why the hell is this a PaletteIndex? it's 8 pixel bits packed into a byte
pub struct Tileset {
    pub pixel_data: [[[PaletteIndex; BYTES_PER_TILE_PLANE]; NUM_TILES]; NUM_PLANES],
    pub rotation_data: [Rotation; NUM_TILES],
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TileIndex(pub u8);

#[derive(Debug, Clone)]
pub struct Tile<'a> {
    pub tile: [&'a [PaletteIndex; BYTES_PER_TILE_PLANE]; NUM_PLANES],
    pub rotation: Rotation,
}

#[derive(Debug)]
pub struct BgLayer {
    pub x: u8,
    pub y: u8,
    tiles: [TileIndex; NUM_BG_TILES],
    pub hidden: bool,
}

impl Tile<'_> {
    // [&[PaletteIndex; BYTES_PER_TILE_PLANE]; NUM_PLANES]
    // get a pixel in local coords
    pub fn get_pixel(&self, pixel_x: u8, pixel_y: u8) -> PaletteIndex {
        let pixel_x = pixel_x % TILE_WIDTH as u8;
        let pixel_y = pixel_y % TILE_HEIGHT as u8;

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
            pixel = (pixel << 1) | ((self.tile[plane_index][pixel_y].0 as usize >> pixel_x) & 1);
        }

        PaletteIndex(pixel as u8)
    }
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

    pub fn get_pixel(&self, tileset: &Tileset, pixel_x: u8, pixel_y: u8) -> PaletteIndex {
        let tile = tileset.get_tile(self.tile_index);

        tile.get_pixel(pixel_x, pixel_y)
    }

    /*
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
    */

    // get a pixel in screen coords
    pub fn get_pixel_global(&self, tileset: &Tileset, screen_x: u8, screen_y: u8) -> PaletteIndex {
        let local_x = screen_x.wrapping_sub(self.x);
        let local_y = screen_y.wrapping_sub(self.y);

        self.get_pixel(tileset, local_x, local_y)
    }
}

impl Tileset {
    pub fn new() -> Tileset {
        Tileset::default()
    }

    pub fn get_tile<'a>(&'a self, tile_index: TileIndex) -> Tile<'a> {
        Tile {
            tile: self.get_tile_pixels(tile_index),
            rotation: self.get_tile_rotation(tile_index),
        }
    }

    // TODO: implement per-tile rotation
    pub fn get_tile_rotation(&self, tile_index: TileIndex) -> Rotation {
        self.rotation_data[tile_index.0 as usize]
    }

    pub fn get_tile_pixels(
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

impl BgLayer {
    fn get_tile(&self, tile_x: u8, tile_y: u8) -> TileIndex {
        self.tiles[tile_x.wrapping_add(tile_y.wrapping_mul(TILE_SIZE as u8)) as usize]
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
        for yi in 0..SCREEN_HEIGHT {
            for xi in 0..SCREEN_WIDTH {
                let xi = xi as u8;
                let yi = yi as u8;

                let pixel_index = Vfc::get_fb_pixel_index(xi, yi);

                let LayerHit {
                    hit: palette_index,
                    layer: _layer,
                    priority: _priority,
                } = self.get_top_pixel(xi, yi);

                /*
                let palette_offset = PaletteIndex(match layer {
                    LayerType::
                    LayerType::
                    _ => 0,
                } * 8);
                */

                self.framebuffer[pixel_index] = self.palette[palette_index];
            }
        }
    }

    pub fn get_fb_pixel_index(x: u8, y: u8) -> usize {
        let x = x as usize;
        let y = y as usize;
        x + SCREEN_WIDTH * y
    }

    fn bg_layer_hit(&self, screen_pixel_x: u8, screen_pixel_y: u8) -> Option<LayerHit> {
        let screen_pixel_x = screen_pixel_x as u8;
        let screen_pixel_y = screen_pixel_y as u8;

        let mut hit_layer_index = None;
        let mut hit_priority = 0;
        let mut hit_pixel = None;

        for (priority, layer) in self.bg_layers.iter().enumerate() {
            if layer.hidden {
                continue;
            }

            let relative_x = screen_pixel_x.wrapping_sub(layer.x);
            let relative_y = screen_pixel_y.wrapping_sub(layer.y);

            let tile_x = relative_x / TILE_SIZE as u8;
            let tile_y = relative_y / TILE_SIZE as u8;

            let tile_pixel_x = relative_x % TILE_SIZE as u8;
            let tile_pixel_y = relative_y % TILE_SIZE as u8;

            let tile_index = layer.get_tile(tile_x, tile_y);

            // TODO: there's no offset that allows backgrounds to use tile data past the first 256 tiles
            let tile = self.tileset.get_tile(tile_index);

            let pixel = tile.get_pixel(tile_pixel_x, tile_pixel_y);

            if pixel != PaletteIndex(0) {
                if priority > hit_priority {
                    hit_layer_index = Some(priority as u8);
                    hit_priority = priority;
                    hit_pixel = Some(pixel);
                    //~ hit_pixel = Some(PaletteIndex(1));
                }
            }
        }

        let layer = if let Some(l) = hit_layer_index {
            LayerType::BgLayer(l)
        } else {
            return None;
        };

        let hit = if let Some(p) = hit_pixel {
            p
        } else {
            return None;
        };

        let priority = hit_priority as u8;

        Some(LayerHit {
            layer,
            hit,
            priority,
        })
    }

    // write to a slice/vec all objects whose bounding box includes this pixel,
    // in layer order, up to a maximum number
    fn get_objects_at_pixel(&self, max_len: usize, pixel_x: u8, pixel_y: u8) -> Vec<OamIndex> {
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

    fn get_top_pixel(&self, pixel_x: u8, pixel_y: u8) -> LayerHit {
        //~ let object_list = self.get_objects_at_pixel(NUM_OAM_ENTRIES, pixel_x, pixel_y);
        let object_list = self.get_objects_at_pixel(16, pixel_x, pixel_y);

        // TODO: does this properly account for priority? investigate
        let oam_hit = 'l: {
            for index in object_list.iter() {
                let oam_entry = &self.oam[*index];
                let pixel = oam_entry.get_pixel_global(&self.tileset, pixel_x, pixel_y);
                if pixel != PaletteIndex(0) {
                    break 'l Some(LayerHit {
                        hit: pixel,
                        layer: LayerType::Oam(*index),
                        priority: oam_entry.priority,
                    });
                }
            }
            None
        };

        let bg_hit = self.bg_layer_hit(pixel_x, pixel_y);
        //~ let bg_hit: Option<LayerHit> = None;

        let hit = 'l: {
            for priority in (0..(NUM_OBJECT_PRIORITY_LEVELS as u8)).rev() {
                let oam_priority = oam_hit.as_ref().map(|hit| hit.priority);
                let bg_priority = bg_hit.as_ref().map(|hit| hit.priority);
                match (bg_priority, oam_priority) {
                    (Some(p), _) => {
                        if priority == p {
                            break 'l bg_hit;
                        }
                    }
                    (None, Some(p)) => {
                        if priority == p {
                            break 'l oam_hit;
                        }
                    }
                    (None, None) => continue,
                }
            }
            None
        };

        /*
        let hits = (0..(NUM_OBJECT_PRIORITY_LEVELS as u8)).map(|priority| {
            let oam_priority = oam_hit.as_ref().map(|h| {
                let LayerHit {
                    hit: _hit,
                    layer: _layer,
                    priority: oam_priority,
                } = h;
                *oam_priority
            }).unwrap_or(0);

            let bg_priority = bg_hit.as_ref().map(|h| {
                let LayerHit {
                    hit: _hit,
                    layer: _layer,
                    priority: bg_priority,
                } = h;
                *bg_priority
            }).unwrap_or(0);

            if priority == bg_priority {
                bg_hit.clone()
            } else if priority == oam_priority {
                oam_hit.clone()
            } else {
                None
            }
        }).collect::<Vec<_>>();

        for hit in hits.into_iter().rev() {
            if let Some(r) = hit {
                return r;
            }
        }
        */

        hit.unwrap_or(LayerHit {
            hit: self.background_color,
            layer: LayerType::BgColor,
            priority: 0,
        })
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
        Self {
            tiles: [(); NUM_BG_TILES].map(|_| TileIndex::default()),
            x: 0,
            y: 0,
            hidden: false,
        }
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
            y: SCREEN_HEIGHT as u8,
            rotation: Rotation(0),
            priority: 1,
            tile_index: TileIndex(0),
            palette_offset: PaletteIndex(0),
        }
    }
}

impl Default for Tileset {
    fn default() -> Tileset {
        let pixel_data = [(); NUM_PLANES].map(|_| {
            [(); NUM_TILES].map(|_| [(); BYTES_PER_TILE_PLANE].map(|_| PaletteIndex::default()))
        });

        let rotation_data = [(); NUM_TILES].map(|_| Rotation::default());

        Tileset {
            pixel_data,
            rotation_data,
        }
    }
}