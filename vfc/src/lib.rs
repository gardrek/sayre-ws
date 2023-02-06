use std::num::Wrapping;

// screen constants
pub const PIXEL_WIDTH: usize = 192;
pub const PIXEL_HEIGHT: usize = 160; // 144?
pub const NUM_SCREEN_PIXELS: usize = PIXEL_WIDTH * PIXEL_HEIGHT;

// layers constants
pub const NUM_LAYERS: usize = 4;
pub const BYTES_PER_TILE: usize = TILE_WIDTH * TILE_HEIGHT / 8 * 3;
pub const NUM_TILES: usize = 1;

// oam constants
pub const NUM_OAM_ENTRIES: usize = 256;
pub const TILE_WIDTH: usize = 8;
pub const TILE_HEIGHT: usize = 8;

pub struct Vfc {
    // stuff goes here
    pub rgb_framebuffer: [RgbValue; NUM_SCREEN_PIXELS],
    //~ pub indexed_framebuffer: [PaletteIndex; NUM_SCREEN_PIXELS],
    pub oam: OamTable,
    pub palette: Palette,
    //~ pub background_color: PaletteIndex,
    pub tileset: TileSet,
}

#[repr(transparent)]
pub struct OamTable([OamEntry; NUM_OAM_ENTRIES]);

#[derive(Default, Clone)]
pub struct OamEntry {
    x: Wrapping<u8>,
    y: Wrapping<u8>,
    rotation: Rotation,
    layer: u8,
    tile_index: TileIndex,
    palette_index: PaletteIndex,
}

impl OamEntry {
    pub fn bounding_box_contains_pixel(&self, x: Wrapping<u8>, y: Wrapping<u8>) -> bool {
        let left = self.x;
        let right = self.x + Wrapping(TILE_WIDTH as u8);
        let top = self.y;
        let bottom = self.y + Wrapping(TILE_HEIGHT as u8);
        x >= left && x < right && y >= top && y < bottom
    }
}

#[repr(transparent)]
#[derive(Default, Clone, Copy)]
pub struct OamIndex(u8);

impl std::ops::Index<OamIndex> for OamTable {
    type Output = OamEntry;

    fn index(&self, index: OamIndex) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

pub enum Layer {
    BgColor,
    BgLayer(u8),
    Oam(OamIndex),
}

pub struct TileSet {
    pub pixel_data: [PaletteIndex; BYTES_PER_TILE * NUM_TILES],
}

impl TileSet {
    pub fn f() {}
}

impl From<[u8; BYTES_PER_TILE * NUM_TILES]> for TileSet {
    fn from(raw_pixel_data: [u8; BYTES_PER_TILE * NUM_TILES]) -> TileSet {
        let pixel_data = raw_pixel_data.map(|byte| PaletteIndex(byte));
        TileSet { pixel_data }
    }
}

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct TileIndex(u8);

impl std::ops::Index<TileIndex> for TileSet {
    type Output = PaletteIndex;

    fn index(&self, index: TileIndex) -> &Self::Output {
        &self.pixel_data[index.0 as usize]
    }
}

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct Palette([RgbValue; 8]);

#[repr(transparent)]
#[derive(Default, Clone, Copy)]
pub struct PaletteIndex(u8);

impl std::ops::Index<PaletteIndex> for Palette {
    type Output = RgbValue;

    fn index(&self, index: PaletteIndex) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

#[repr(transparent)]
#[derive(Default, Clone, Copy)]
pub struct Rotation(u8);

#[derive(Default, Clone, Copy)]
pub struct RgbValue(u8, u8, u8);

impl Vfc {
    pub fn new() -> Vfc {
        Vfc { ..Vfc::default() }
    }

    pub fn render_frame(&mut self) {
        for yi in 0..PIXEL_HEIGHT {
            for xi in 0..PIXEL_WIDTH {
                let xi = Wrapping(xi as u8);
                let yi = Wrapping(yi as u8);
                let pixel_index = Vfc::get_fb_pixel_index(xi, yi);
                let (color_index, object_index) = self.get_top_pixel(xi, yi);
                self.rgb_framebuffer[pixel_index] = self.get_pixel_rgb(color_index, object_index);
            }
        }
    }

    pub fn get_fb_pixel_index(x: Wrapping<u8>, y: Wrapping<u8>) -> usize {
        (x + Wrapping(PIXEL_WIDTH as u8) * y).0.into()
    }

    pub fn get_top_pixel(&self, _pixel_x: Wrapping<u8>, _pixel_y: Wrapping<u8>) -> (PaletteIndex, Option<u8>) {
        /*
        let color_index = 0;
        //let object_index = PixelObject::BgColor;
        let object_index = {
            let mut object_index = 0;
            loop {
                let object = self.oam[object_index];
                if object.bounding_box_contains_pixel(pixel_x, pixel_y) {
                    break Some(object_index);
                } else if object_index < NUM_OAM_ENTRIES {
                    object_index += 1;
                    continue;
                } else {
                    break None;
                }
            }
        };

        (color_index, object_index)
        */
        todo!()
    }

    pub fn get_layer_at_pixel(&self, x: Wrapping<u8>, y: Wrapping<u8>) -> Layer {
        use Layer::*;

        let bg_hit = self.background_hit(x, y);

        let object_hit = self.object_hit(x, y);

        'l: {
            for layer in 0..NUM_LAYERS {
                if let Some(object_index) = object_hit {
                    let object = &self.oam[object_index];
                    if layer as u8 == object.layer {
                        break 'l Oam(object_index);
                    }
                } else if let Some(l) = bg_hit {
                    break 'l BgLayer(l);
                }
            }
            BgColor
        }
    }

    pub fn get_pixel_rgb(&self, _color_index: PaletteIndex, _object_index: Option<u8>) -> RgbValue {
        todo!()
    }

    pub fn background_hit(&self, _pixel_x: Wrapping<u8>, _pixel_y: Wrapping<u8>) -> Option<u8> {
        todo!()
    }

    pub fn object_hit_pix(&self, _pixel_x: Wrapping<u8>, _pixel_y: Wrapping<u8>) -> bool {
        todo!()
    }

    pub fn object_hit(&self, pixel_x: Wrapping<u8>, pixel_y: Wrapping<u8>) -> Option<OamIndex> {
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

impl Default for Vfc {
    fn default() -> Vfc {
        Vfc {
            oam: OamTable([(); NUM_OAM_ENTRIES].map(|_| OamEntry::default())),
            rgb_framebuffer: [(); NUM_SCREEN_PIXELS].map(|_| RgbValue::default()),
            palette: Default::default(),
            tileset: TileSet {
                pixel_data: [PaletteIndex(0); BYTES_PER_TILE * NUM_TILES],
            },
        }
    }
}
