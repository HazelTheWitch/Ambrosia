use rltk::RGB;

use crate::constants;

pub struct Theme {
    pub ui_color: RGB,
    pub background_color: RGB,
    pub terrain_color_visible: RGB,
    pub terrain_color_discovered: RGB
}

impl Theme {
    pub fn new() -> Self {
        Theme {
            ui_color: constants::UI_COLOR.into(),
            background_color: constants::BACKGROUND_COLOR.into(),
            terrain_color_visible: constants::TERRAIN_COLOR_VISIBLE.into(),
            terrain_color_discovered: constants::TERRAIN_COLOR_DISCOVERED.into(),
        }
    }
}