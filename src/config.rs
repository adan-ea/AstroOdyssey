// Sprite sheet constants
pub const SPRITE_PADDING: f32 = 2.0;
pub const SPRITE_SCALE_FACTOR: usize = 5;
pub const SPRITE_SHEET_HEIGHT: usize = 9;
pub const SPRITE_SHEET_OFFSET: f32 = 2.0;
pub const SPRITE_SHEET_PATH: &str = "sprite-sheet.png";
pub const SPRITE_SHEET_WIDTH: usize = 8;
pub const TILE_HEIGHT: usize = 8;
pub const TILE_WIDTH: usize = 6;

// Window constants
pub const GRID_COLS: i32 = 200;
pub const GRID_ROWS: i32 = 100;
pub const GRID_W: usize = GRID_COLS as usize * TILE_WIDTH;
pub const GRID_H: usize = GRID_ROWS as usize * TILE_HEIGHT;
pub const BG_COLOR: (u8, u8, u8) = (181, 212, 220);

// If seed is set to 0, the seed will be random
pub const SEED: u32 = 0;