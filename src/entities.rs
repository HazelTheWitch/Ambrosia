use rltk::RGB;

use crate::components::*;
use crate::constants::*;
use crate::ecs::{ECSError, entity::EntityBuilder};

type BuilderResult<'w> = Result<EntityBuilder, ECSError>;

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
    fg: Option<RGB>,
    bg: Option<RGB>,
) -> BuilderResult {
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
        Some(RGB::named(PLAYER_COLOR)),
        Some(RGB::named(BACKGROUND_COLOR)),
    )?
    .insert_component(Camera::new())?
    .insert_component(Player::new())?
    .insert_component(Viewshed::new(11.5))
}
