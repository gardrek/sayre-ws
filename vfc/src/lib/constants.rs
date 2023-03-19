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
pub const NUM_BG_LAYERS: usize = 2;
pub const BG_SIZE: usize = 32;
pub const BG_WIDTH: usize = BG_SIZE;
pub const BG_HEIGHT: usize = BG_SIZE;
pub const NUM_BG_TILES: usize = BG_WIDTH * BG_HEIGHT;
pub const NUM_BG_PRIORITY_LEVELS: usize = 2;

// misc constants
pub const NUM_PALETTE_ENTRIES: usize = 64;
pub const TILE_PALETTE_SIZE: usize = 2_usize.pow(NUM_PLANES as u32);
pub const NUM_OBJECT_PRIORITY_LEVELS: usize = 4;
pub const SUBPALETTE_SIZE: usize = 8;
