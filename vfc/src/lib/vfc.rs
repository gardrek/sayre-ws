//

// screen constants
pub const SCREEN_WIDTH: usize = 192;
pub const SCREEN_HEIGHT: usize = 160; // 144?
pub const NUM_SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

// tile constants
pub const NUM_PLANES: usize = 3;
pub const BYTES_PER_TILE_PLANE: usize = TILE_WIDTH * TILE_HEIGHT / 8;
pub const TILE_INDEX_BITS: usize = 8;
pub const NUM_TILES: usize = 2_usize.pow(TILE_INDEX_BITS as u32);

// oam constants
pub const NUM_OAM_ENTRIES: usize = 256;
pub const TILE_SIZE: usize = 8;
pub const TILE_WIDTH: usize = TILE_SIZE;
pub const TILE_HEIGHT: usize = TILE_SIZE;
pub const OBJECTS_PER_LINE: usize = 16;

// background layer constants
pub const NUM_BG_LAYERS: usize = 3;
pub const BG_SIZE: usize = 32;
pub const BG_WIDTH: usize = BG_SIZE;
pub const BG_HEIGHT: usize = BG_SIZE;
pub const NUM_BG_TILES: usize = BG_WIDTH * BG_HEIGHT;
pub const NUM_BG_PRIORITY_LEVELS: usize = 2;

// misc constants
pub const NUM_PALETTE_ENTRIES: usize = 64;
pub const TILE_PALETTE_SIZE: usize = 2_usize.pow(NUM_PLANES as u32);
pub const NUM_OBJECT_PRIORITY_LEVELS: usize = 4;

pub struct Vfc {
    // stuff goes here
    pub framebuffer: [Rgb; NUM_SCREEN_PIXELS],
    //~ pub indexed_framebuffer: [PaletteIndex; NUM_SCREEN_PIXELS],
    pub oam: OamTable,
    //~ sorted_objects: [[Option<OamIndex>; OBJECTS_PER_LINE]; SCREEN_HEIGHT],
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
pub struct Palette([Rgb; NUM_PALETTE_ENTRIES]);

impl Palette {
    pub fn new(p: [Rgb; NUM_PALETTE_ENTRIES]) -> Palette {
        Palette(p)
    }
}

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

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rgb(u32);
//~ pub struct Rgb(u8, u8, u8);

impl Rgb {
    /*
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb(r, g, b)
    }
    */

    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb(u32::from_be_bytes([255, r, g, b]))
    }

    pub fn from_argb_u32(argb: &u32) -> Rgb {
        let bytes = argb.to_be_bytes();
        Rgb::new(bytes[0], bytes[1], bytes[2])
    }

    /*
    pub fn as_argb_u32(&self) -> u32 {
        u32::from_be_bytes([255, self.0, self.1, self.2])
    }
    */

    pub fn as_argb_u32(&self) -> u32 {
        self.0
    }
}

// FIXME: why is there rotation data here what the hell
// absolutely need to fix this, there's no reason for the tileset to have rotation data
// NOTE: why the hell is this a PaletteIndex? answer: it's 8 PaletteIndex bits packed into a byte
// so like it's probably fine idk. probably better without tho
pub struct Tileset {
    pub pixel_data: [[[PaletteIndex; BYTES_PER_TILE_PLANE]; NUM_TILES]; NUM_PLANES],
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TileIndex(pub u8);

#[derive(Debug, Clone)]
pub struct Tile<'a> {
    pub tile: [&'a [PaletteIndex; BYTES_PER_TILE_PLANE]; NUM_PLANES],
}

// TODO: implement per-tile rotation
#[derive(Debug)]
pub struct BgLayer {
    pub x: u8,
    pub y: u8,
    pub tiles: [TileIndex; NUM_BG_TILES],
    pub hidden: bool,
}

#[derive(Debug, Default, Clone)]
pub struct TileAttributes(u8);

impl TileAttributes {
    fn oam_default() -> Self {
        let mut s = Self(0);
        s.set_priority(1);
        s
    }

    pub fn palette(&self) -> PaletteIndex {
        PaletteIndex(self.0 & 0b111)
    }

    pub fn rotation(&self) -> u8 {
        (self.0 >> 3) & 0b111
    }

    pub fn flip_diagonal(&self) -> bool {
        (self.0 >> 3) & 1 != 0
    }

    pub fn flip_x(&self) -> bool {
        (self.0 >> 4) & 1 != 0
    }

    pub fn flip_y(&self) -> bool {
        (self.0 >> 5) & 1 != 0
    }

    pub fn priority(&self) -> u8 {
        (self.0 >> 6) & 0b11
    }

    pub fn set_color(&mut self, palette_index: PaletteIndex) {
        self.0 &= 0b11_111_000;
        self.0 |= palette_index.0 & 0b111;
    }

    pub fn set_rotation(&mut self, rotation: u8) {
        self.0 &= 0b11_000_111;
        self.0 |= rotation & 0b111 << 3;
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.0 &= 0b00_111_111;
        self.0 |= priority & 0b11 << 6;
    }
}

impl Tile<'_> {
    // get a pixel in local coords
    pub fn get_pixel(&self, pixel_x: u8, pixel_y: u8) -> PaletteIndex {
        let pixel_x = pixel_x % TILE_WIDTH as u8;
        let pixel_y = pixel_y % TILE_HEIGHT as u8;

        //~ let flip_x = rotation & 1 != 0;
        //~ let flip_y = (rotation >> 1) & 1 != 0;
        //~ let flip_diagonal = (rotation >> 2) & 1 != 0;

        let flip_x = false;
        let flip_y = false;
        let flip_diagonal = false;

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
            TILE_HEIGHT - 1 - pixel_y
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
    */

    pub fn get_pixel(&self, tileset: &Tileset, pixel_x: u8, pixel_y: u8) -> PaletteIndex {
        let tile = tileset.get_tile(self.tile_index);

        tile.get_pixel(pixel_x, pixel_y)
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
        Tileset::default()
    }

    fn get_tile<'a>(&'a self, tile_index: TileIndex) -> Tile<'a> {
        Tile {
            tile: self.get_tile_pixels(tile_index),
        }
    }

    fn get_tile_pixels(
        &self,
        tile_index: TileIndex,
    ) -> [&[PaletteIndex; BYTES_PER_TILE_PLANE]; NUM_PLANES] {
        [(); NUM_PLANES]
            .iter()
            .enumerate()
            .map(|(plane_index, _)| &self.pixel_data[plane_index][tile_index.0 as usize])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| unreachable!())
        /*        (0..NUM_PLANES)
        .map(|plane_index| &self.pixel_data[plane_index][tile_index.0 as usize])
        .collect::<Vec<_>>()
        .try_into()
        .unwrap_or_else(|_| unreachable!())*/
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
    fn get_tile_index(&self, tile_x: u8, tile_y: u8) -> TileIndex {
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
                    //~ Rgb::new(x ^ y, 0, 0)
                    Rgb::new(x * 16 as u8, y * 16 as u8, i as u8)
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| unreachable!()),
        )
    }

    pub fn render_frame(&mut self) {
        //~ let object_list = &mut Default::default();

        for scanline in 0..SCREEN_HEIGHT as u8 {
            //~ let object_list = &mut Default::default();

            let object_list = self.get_objects_on_scanline(scanline);

            //~ self.get_objects_on_scanline_buffered(object_list, scanline);

            self.render_scanline(&object_list[..], scanline);
        }
    }

    pub fn render_scanline(&mut self, object_list: &[OamIndex], yi: u8) {
        //~ let object_list = self.get_objects_on_scanline(yi);

        for xi in 0..SCREEN_WIDTH as u8 {
            let pixel_index = Vfc::get_fb_pixel_index(xi, yi);

            let LayerHit {
                hit: palette_index,
                layer: _layer,
                priority: _priority,
            } = self.get_top_pixel(&object_list[..], xi, yi);

            self.framebuffer[pixel_index] = self.palette[palette_index];
        }
    }

    /*pub fn render_scanline_buffered(
        &mut self,
        object_list: &mut [Option<OamIndex>; OBJECTS_PER_LINE],
        yi: u8,
    ) {
        //~ let object_list = self.get_objects_on_scanline(yi);

        for xi in 0..SCREEN_WIDTH as u8 {
            let pixel_index = Vfc::get_fb_pixel_index(xi, yi);

            let LayerHit {
                hit: palette_index,
                layer: _layer,
                priority: _priority,
            } = self.get_top_pixel(&object_list[..], xi, yi);

            self.framebuffer[pixel_index] = self.palette[palette_index];
        }
    }*/

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

            let tile_index = layer.get_tile_index(tile_x, tile_y);

            // TODO: there's no offset that allows backgrounds to use tile data past the first 256 tiles
            //~ let tile = self.tileset.get_tile(tile_index);

            //~ let pixel = tile.get_pixel(tile_pixel_x, tile_pixel_y);

            let pixel = self.get_tile_pixel(tile_index, tile_pixel_x, tile_pixel_y);

            if pixel != PaletteIndex(0) {
                if priority > hit_priority {
                    hit_layer_index = Some(priority as u8);
                    hit_priority = priority;
                    hit_pixel = Some(pixel);
                    //~ hit_pixel = Some(PaletteIndex(1));
                    break;
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

    fn _get_objects_on_scanline_buffered(
        &self,
        output_list: &mut [Option<OamIndex>; OBJECTS_PER_LINE],
        scanline: u8,
    ) {
        let mut list_index = 0;

        for object_index in 0..NUM_OAM_ENTRIES {
            let object = &self.oam[OamIndex(object_index as u8)];

            if scanline >= object.y && scanline < object.y.wrapping_add(TILE_HEIGHT as u8) {
                output_list[list_index] = Some(OamIndex(object_index as u8));

                list_index += 1;

                if list_index >= OBJECTS_PER_LINE {
                    break;
                }
            }
        }
    }

    fn get_objects_on_scanline(&self, scanline: u8) -> Vec<OamIndex> {
        let mut sorted_objects = Vec::with_capacity(OBJECTS_PER_LINE);

        for object_index in 0..NUM_OAM_ENTRIES {
            let object = &self.oam[OamIndex(object_index as u8)];
            if scanline >= object.y && scanline < object.y.wrapping_add(TILE_HEIGHT as u8) {
                sorted_objects.push(OamIndex(object_index as u8));
            }
        }

        sorted_objects
    }

    fn get_top_pixel(&self, object_list: &[OamIndex], pixel_x: u8, pixel_y: u8) -> LayerHit {
        let oam_hit = 'l: {
            for index in object_list.iter() {
                let oam_entry = &self.oam[*index];

                // TODO: this doesn't account for negative x values
                // but it is necessary due to how get_tile_pixel_global works
                if pixel_x < oam_entry.x || pixel_x >= oam_entry.x + TILE_WIDTH as u8 {
                    continue;
                }

                let pixel = self.get_tile_pixel_global(*index, pixel_x, pixel_y);

                if pixel != PaletteIndex(0) {
                    break 'l Some(LayerHit {
                        hit: pixel,
                        layer: LayerType::Oam(*index),
                        priority: oam_entry.attributes.priority(),
                    });
                }
            }
            None
        };

        let bg_hit = self.bg_layer_hit(pixel_x, pixel_y);

        let hit = 'l: {
            for priority in (0..(NUM_OBJECT_PRIORITY_LEVELS as u8)).rev() {
                let oam_priority = oam_hit.as_ref().map(|hit| hit.priority);
                let bg_priority = bg_hit.as_ref().map(|hit| hit.priority);
                match (oam_priority, bg_priority) {
                    (Some(p), _) => {
                        if priority == p {
                            break 'l oam_hit;
                        }
                    }
                    (None, Some(p)) => {
                        if priority == p {
                            break 'l bg_hit;
                        }
                    }
                    (None, None) => continue,
                }
            }
            None
        };

        hit.unwrap_or(LayerHit {
            hit: self.background_color,
            layer: LayerType::BgColor,
            priority: 0,
        })
    }

    // get a pixel in screen coords
    pub fn get_tile_pixel_global(
        &self,
        tile_index: OamIndex,
        screen_x: u8,
        screen_y: u8,
    ) -> PaletteIndex {
        let oam_entry = &self.oam[tile_index];

        let tile_index = oam_entry.tile_index;

        let local_x = screen_x.wrapping_sub(oam_entry.x);
        let local_y = screen_y.wrapping_sub(oam_entry.y);

        self.get_tile_pixel(tile_index, local_x, local_y)
    }

    // NOTE: does not work with tiles wider than 8 pixels
    fn get_tile_pixel(
        &self,
        tile_index: TileIndex,
        /* attributes: TileAttributesAttributes, */ pixel_x: u8,
        pixel_y: u8,
    ) -> PaletteIndex {
        let pixel_x = pixel_x % TILE_WIDTH as u8;
        let pixel_y = pixel_y % TILE_HEIGHT as u8;

        /*
        attributes;
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
        */

        /*
        let pixel = self.tileset.pixel_data.iter().fold(0, |acc, plane| {
            (acc << 1)
                | ((plane[tile_index.0 as usize][pixel_y as usize].0
                    as usize
                    >> pixel_x)
                    & 1)
        });
        //~ */

        //~ /*
        let pixel = (0..NUM_PLANES).fold(0, |acc, plane_index| {
            (acc << 1)
                | ((self.tileset.pixel_data[plane_index][tile_index.0 as usize][pixel_y as usize].0
                    as usize
                    >> pixel_x)
                    & 1)
        });
        //~ */
        /*
        let mut pixel = 0;
        for plane_index in 0..NUM_PLANES {
            pixel = (pixel << 1)
                | ((self.tileset.pixel_data[plane_index][tile_index.0 as usize][pixel_y as usize].0
                    as usize
                    >> pixel_x)
                    & 1);
        }
        //~ */

        PaletteIndex(pixel as u8)
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
            tile_index: TileIndex(0),
            attributes: TileAttributes::oam_default(),
        }
    }
}

impl Default for Tileset {
    fn default() -> Tileset {
        let pixel_data = [(); NUM_PLANES].map(|_| {
            [(); NUM_TILES].map(|_| [(); BYTES_PER_TILE_PLANE].map(|_| PaletteIndex::default()))
        });

        Tileset { pixel_data }
    }
}
