use rltk::RGB;

use crate::components::*;
use crate::constants::*;
use crate::ecs::*;

pub fn named(
    entity: Result<&mut Entity, ECSError>,
    name: String,
) -> Result<&mut Entity, ECSError> {
    entity?.insert_component(Named::new(name))
}

pub fn debugged(
    entity: Result<&mut Entity, ECSError>,
    name: String,
) -> Result<&mut Entity, ECSError> {
    named(entity, name)?.insert_component(Debug::new())
}

pub fn positioned(
    entity: Result<&mut Entity, ECSError>,
    x: i32,
    y: i32,
    priority: u8,
) -> Result<&mut Entity, ECSError> {
    entity?.insert_component(Position::new(x, y, priority))
}

pub fn renderable(
    entity: Result<&mut Entity, ECSError>,
    x: i32,
    y: i32,
    priority: u8,
    character: char,
    fg: (u8, u8, u8),
    bg: (u8, u8, u8),
) -> Result<&mut Entity, ECSError> {
    positioned(entity, x, y, priority)?.insert_component(SingleGlyphRenderer::new(
        rltk::to_cp437(character),
        RGB::named(fg),
        RGB::named(bg),
    ))
}

pub fn player(
    entity: Result<&mut Entity, ECSError>,
    name: String,
    x: i32,
    y: i32,
) -> Result<&mut Entity, ECSError> {
    renderable(
        debugged(entity, name),
        x,
        y,
        255,
        PLAYER_GLYPH,
        PLAYER_COLOR,
        BACKGROUND_COLOR,
    )?
    .insert_component(Camera::new())?
    .insert_component(Player::new())?
    .insert_component(Viewshed::new(11.5))
}
