use crate::vectors::Vector;

// Map
pub const MAP_SIZE: (usize, usize) = (100, 100);

// Glyphs
pub const PLAYER_GLYPH: char = '@';

// Colors
pub const PLAYER_COLOR: (u8, u8, u8) = rltk::YELLOW;
pub const UI_COLOR: (u8, u8, u8) = rltk::GRAY90;
pub const TERRAIN_COLOR_VISIBLE: (u8, u8, u8) = rltk::GRAY70;
pub const TERRAIN_COLOR_DISCOVERED: (u8, u8, u8) = rltk::GRAY30;

pub const BACKGROUND_COLOR: (u8, u8, u8) = rltk::GRAY4;

// UI
pub const SCREEN_SIZE: Vector = Vector { x: 120, y: 80 };
pub const SCREEN_SIZE_2: Vector = Vector {
    x: SCREEN_SIZE.x / 2,
    y: SCREEN_SIZE.y / 2,
};
pub const CORNER_POINT: Vector = Vector { x: 80, y: 60 };
pub const CORNER_POINT_2: Vector = Vector {
    x: CORNER_POINT.x / 2,
    y: CORNER_POINT.y / 2,
};
