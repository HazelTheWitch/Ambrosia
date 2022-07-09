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
            ui_color: RGB::named(constants::UI_COLOR),
            background_color: RGB::named(constants::BACKGROUND_COLOR),
            terrain_color_visible: RGB::named(constants::TERRAIN_COLOR_VISIBLE),
            terrain_color_discovered: RGB::named(constants::TERRAIN_COLOR_DISCOVERED),
        }
    }
}