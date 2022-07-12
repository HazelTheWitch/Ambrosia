use rltk::RGB;

use crate::components::*;
use crate::constants::*;
use crate::ecs::*;

type BuilderResult<'w> = Result<EntityBuilder<'w>, ECSError>;

pub fn named(
    entity: BuilderResult,
    name: String,
) -> BuilderResult {
    entity?.insert_component(Named::new(name))
}

pub fn debugged(
    entity: BuilderResult,
    name: String,
) -> BuilderResult {
    named(entity, name)?.insert_component(Debug::new())
}

pub fn positioned(
    entity: BuilderResult,
    x: i32,
    y: i32,
    priority: u8,
) -> BuilderResult {
    entity?.insert_component(Position::new(x, y, priority))
}

pub fn renderable(
    entity: BuilderResult,
    x: i32,
    y: i32,
    priority: u8,
    character: char,
    fg: Option<(u8, u8, u8)>,
    bg: Option<(u8, u8, u8)>,
) -> BuilderResult {
    let fg = match fg {
        Some(color) => Some(RGB::named(color)),
        None => None
    };

    let bg = match bg {
        Some(color) => Some(RGB::named(color)),
        None => None
    };

    positioned(entity, x, y, priority)?.insert_component(Renderer::new(
        rltk::to_cp437(character),
        fg,
        bg,
    ))
}

pub fn player(
    entity: BuilderResult,
    name: String,
    x: i32,
    y: i32,
) -> BuilderResult {
    renderable(
        debugged(entity, name),
        x,
        y,
        255,
        PLAYER_GLYPH,
        Some(PLAYER_COLOR),
        Some(BACKGROUND_COLOR),
    )?
    .insert_component(Camera::new())?
    .insert_component(Player::new())?
    .insert_component(Viewshed::new(11.5))
}
