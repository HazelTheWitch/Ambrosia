use crate::vectors::Vector;

// Glyphs
pub const PLAYER_GLYPH: char = '@';

// Colors
pub const PLAYER_COLOR: (u8, u8, u8) = rltk::YELLOW;

pub const BACKGROUND_COLOR: (u8, u8, u8) = rltk::BLACK;

// UI
pub const SCREEN_SIZE: Vector = Vector { x: 120, y: 80 };
pub const SCREEN_SIZE_2: Vector = Vector { x: SCREEN_SIZE.x / 2, y: SCREEN_SIZE.y / 2 };
pub const CORNER_POINT: Vector = Vector { x: 80, y: 60 };