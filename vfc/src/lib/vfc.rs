//
mod constants;
mod oam;

pub use oam::*;

pub use constants::*;

pub struct Vfc {
    // stuff goes here
    pub framebuffer: [Rgb; NUM_SCREEN_PIXELS],
    //~ pub indexed_framebuffer: [PaletteIndex; NUM_SCREEN_PIXELS],
    pub oam: OamTable,
    pub oam_hidden: bool,
    //~ sorted_objects: [[Option<OamIndex>; OBJECTS_PER_LINE]; SCREEN_HEIGHT],
    pub palette: Palette,
    pub background_color: PaletteIndex,
    pub tileset: Tileset,
    pub bg_layers: [BgLayer; NUM_BG_LAYERS],
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
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct RawPixel(pub u8);

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

impl PaletteIndex {
    pub const fn new(s: u8) -> PaletteIndex {
        PaletteIndex(s)
    }

    pub fn get(&self) -> &u8 {
        &self.0
    }
}

impl std::ops::Index<PaletteIndex> for Palette {
    type Output = Rgb;

    fn index(&self, index: PaletteIndex) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Subpalette(u8);

impl Subpalette {
    pub const fn new(s: u8) -> Subpalette {
        Subpalette(s)
    }

    pub fn get(&self) -> &u8 {
        &self.0
    }

    fn colorize_pixel(&self, pixel: RawPixel) -> PaletteIndex {
        PaletteIndex(
            self.0
                .wrapping_mul(SUBPALETTE_SIZE as u8)
                .wrapping_add(pixel.0),
        )
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
    pub pixel_data: [[[u8; BYTES_PER_TILE_PLANE]; NUM_TILES]; NUM_PLANES],
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TileIndex(pub u8);

#[derive(Debug, Clone)]
pub struct Tile<'a> {
    pub tile: [&'a [u8; BYTES_PER_TILE_PLANE]; NUM_PLANES],
}

// TODO: implement per-tile rotation
#[derive(Debug)]
pub struct BgLayer {
    pub x: u8,
    pub y: u8,
    pub tiles: [TileIndex; NUM_BG_TILES],
    pub attributes: [TileAttributes; NUM_BG_TILES],
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

    pub fn get_palette(&self) -> Subpalette {
        Subpalette::new(self.0 & 0b111)
    }

    pub fn get_rotation(&self) -> u8 {
        (self.0 >> 3) & 0b111
    }

    pub fn get_flip_x(&self) -> bool {
        (self.0 >> 3) & 1 != 0
    }

    pub fn get_flip_y(&self) -> bool {
        (self.0 >> 4) & 1 != 0
    }

    pub fn get_flip_diagonal(&self) -> bool {
        (self.0 >> 5) & 1 != 0
    }

    pub fn get_priority(&self) -> u8 {
        (self.0 >> 6) & 0b11
    }

    pub fn set_palette(&mut self, subpalette: Subpalette) {
        self.0 &= 0b11_111_000;
        self.0 |= subpalette.get() & 0b111;
    }

    pub fn set_rotation(&mut self, rotation: u8) {
        self.0 &= 0b11_000_111;
        self.0 |= (rotation & 0b111) << 3;
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.0 &= 0b00_111_111;
        self.0 |= (priority & 0b11) << 6;
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
            pixel = (pixel << 1) | ((self.tile[plane_index][pixel_y] as usize >> pixel_x) & 1);
        }

        PaletteIndex(pixel as u8)
    }
}

impl Tileset {
    pub fn new() -> Tileset {
        Tileset::default()
    }

    /*
    fn get_tile<'a>(&'a self, tile_index: TileIndex) -> Tile<'a> {
        Tile {
            tile: self.get_tile_pixels(tile_index),
        }
    }

    fn get_tile_pixels(&self, tile_index: TileIndex) -> [&[u8; BYTES_PER_TILE_PLANE]; NUM_PLANES] {
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
    */

    pub fn write_tile(
        &mut self,
        tile_index: TileIndex,
        tile: [[u8; BYTES_PER_TILE_PLANE]; NUM_PLANES],
    ) {
        for (plane_index, plane) in tile.iter().enumerate() {
            self.pixel_data[plane_index][tile_index.0 as usize] = *plane;
        }
    }
}

impl BgLayer {
    fn get_tile_index(&self, tile_x: u8, tile_y: u8) -> TileIndex {
        let tile_x = tile_x as usize;
        let tile_y = tile_y as usize;

        // this one was a fun bug to track down
        //~ self.tiles[tile_x.wrapping_add(tile_y.wrapping_mul(TILE_SIZE as u8)) as usize]

        self.tiles[tile_x.wrapping_add(tile_y.wrapping_mul(BG_WIDTH)) as usize]

        //~ self.tiles[tile_x as usize + tile_y as usize * BG_WIDTH as usize]
    }

    fn get_tile_attribute(&self, tile_x: u8, tile_y: u8) -> &TileAttributes {
        let tile_x = tile_x as usize;
        let tile_y = tile_y as usize;

        &self.attributes[tile_x.wrapping_add(tile_y.wrapping_mul(BG_WIDTH)) as usize]
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
            //~ let priority = priority + 1;

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

            // TODO: there's no offset that allows backgrounds
            // to use tile data past the first 256 tiles.
            // do we want this?
            //~ let tile = self.tileset.get_tile(tile_index);

            //~ let pixel = tile.get_pixel(tile_pixel_x, tile_pixel_y);

            let pixel = self.get_tile_pixel(tile_index, tile_pixel_x, tile_pixel_y);

            let subpalette = layer.get_tile_attribute(tile_x, tile_y).get_palette();

            let colorized_pixel = subpalette.colorize_pixel(pixel);

            if pixel != RawPixel(0) {
                if priority >= hit_priority {
                    hit_layer_index = Some(priority as u8);
                    hit_priority = priority;
                    hit_pixel = Some(colorized_pixel);
                    //~ hit_pixel = Some(pixel);
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

            let bottom = object.y.wrapping_add(TILE_HEIGHT as u8);

            if scanline >= object.y && scanline < bottom {
                //~ if (scanline >= object.y && scanline < bottom)
                //~ || (object.y > bottom && (scanline > bottom || scanline <= object.y))
                //~ {
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
                let right = oam_entry.x.wrapping_add(TILE_WIDTH as u8);

                if (pixel_x < oam_entry.x) || (pixel_x >= right) {
                    //~ if !((pixel_x >= oam_entry.x && pixel_x < right)
                    //~ || (oam_entry.x > right && (pixel_x > right || pixel_x <= oam_entry.x)))
                    //~ {
                    continue;
                }

                let pixel = self.get_tile_pixel_global(*index, pixel_x, pixel_y);

                /*
                let subpalette = oam_entry.attributes.get_palette();

                let colorized_pixel = PaletteIndex(
                    pixel
                        .0
                        .wrapping_add(subpalette.0.wrapping_mul(SUBPALETTE_SIZE as u8)),
                );
                */

                let colorized_pixel = oam_entry.attributes.get_palette().colorize_pixel(pixel);

                if pixel != RawPixel(0) {
                    break 'l Some(LayerHit {
                        hit: colorized_pixel,
                        layer: LayerType::Oam(*index),
                        priority: oam_entry.attributes.get_priority(),
                    });
                }
            }
            None
        };

        let bg_hit = self.bg_layer_hit(pixel_x, pixel_y);

        let hit = 'l: {
            for priority in (0..(NUM_OBJECT_PRIORITY_LEVELS as u8)).rev() {
                if self.oam_hidden {
                    break 'l None;
                }

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
    ) -> RawPixel {
        let oam_entry = &self.oam[tile_index];

        let tile_index = oam_entry.tile_index;

        let local_x = screen_x.wrapping_sub(oam_entry.x);
        let local_y = screen_y.wrapping_sub(oam_entry.y);

        self.get_tile_pixel_rotated(tile_index, &oam_entry.attributes, local_x, local_y)
    }

    // NOTE: does not work with tiles wider than 8 pixels
    fn get_tile_pixel(&self, tile_index: TileIndex, pixel_x: u8, pixel_y: u8) -> RawPixel {
        //~ let pixel_x = pixel_x % TILE_WIDTH as u8;
        //~ let pixel_y = pixel_y % TILE_HEIGHT as u8;

        let pixel = (0..NUM_PLANES).fold(0, |acc, plane_index| {
            (acc << 1)
                | ((self.tileset.pixel_data[plane_index][tile_index.0 as usize][pixel_y as usize]
                    as usize
                    >> pixel_x)
                    & 1)
        });

        RawPixel(pixel as u8)
    }

    // NOTE: does not work with tiles wider than 8 pixels
    fn get_tile_pixel_rotated(
        &self,
        tile_index: TileIndex,
        attributes: &TileAttributes,
        pixel_x: u8,
        pixel_y: u8,
    ) -> RawPixel {
        let pixel_x = pixel_x % TILE_WIDTH as u8;
        let pixel_y = pixel_y % TILE_HEIGHT as u8;

        let rotation = attributes.get_rotation();

        let flip_x = rotation & 1 != 0;
        let flip_y = (rotation >> 1) & 1 != 0;
        let flip_diagonal = (rotation >> 2) & 1 != 0;
        //~ let flip_x = true;
        //~ let flip_y = false;
        //~ let flip_diagonal = true;

        let (pixel_x, pixel_y) = (pixel_x as usize, pixel_y as usize);

        let pixel_x = if flip_x {
            TILE_WIDTH - 1 - pixel_x
        } else {
            pixel_x
        };

        let pixel_y = if flip_y {
            TILE_WIDTH - 1 - pixel_y
        } else {
            pixel_y
        };

        let (pixel_x, pixel_y) = if flip_diagonal {
            (pixel_y, pixel_x)
        } else {
            (pixel_x, pixel_y)
        };

        let pixel = (0..NUM_PLANES).fold(0, |acc, plane_index| {
            (acc << 1)
                | ((self.tileset.pixel_data[plane_index][tile_index.0 as usize][pixel_y as usize]
                    as usize
                    >> pixel_x)
                    & 1)
        });

        RawPixel(pixel as u8)
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self([(); NUM_PALETTE_ENTRIES].map(|_| Rgb::default()))
    }
}

impl Default for BgLayer {
    fn default() -> Self {
        Self {
            tiles: [(); NUM_BG_TILES].map(|_| TileIndex::default()),
            attributes: [(); NUM_BG_TILES].map(|_| TileAttributes::default()),
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
            oam_hidden: false,
            framebuffer: [(); NUM_SCREEN_PIXELS].map(|_| Rgb::default()),
            palette: Palette::default(),
            background_color: PaletteIndex::default(),
            tileset: Tileset::new(),
            bg_layers: Default::default(),
        }
    }
}

impl Default for Tileset {
    fn default() -> Tileset {
        let pixel_data = [(); NUM_PLANES].map(|_| {
            [(); NUM_TILES].map(|_| [(); BYTES_PER_TILE_PLANE].map(|_| Default::default()))
        });

        Tileset { pixel_data }
    }
}
