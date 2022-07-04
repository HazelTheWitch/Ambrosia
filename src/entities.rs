use rltk::RGB;

use crate::ecs::*;
use crate::components::*;
use crate::constants::*;

pub fn named<'w>(entity: Result<&'w mut Entity, ECSError>, name: String) -> Result<&'w mut Entity, ECSError> {
    entity?.insert_component(Named::new(name))
}

pub fn debugged<'w>(entity: Result<&'w mut Entity, ECSError>, name: String) -> Result<&'w mut Entity, ECSError> {
    named(entity, name)?
        .insert_component(Debug::new())
}

pub fn positioned<'w>(entity: Result<&'w mut Entity, ECSError>, x: i32, y: i32) -> Result<&'w mut Entity, ECSError> {
    entity?.insert_component(Position::new(x, y))
}

pub fn renderable<'w>(entity: Result<&'w mut Entity, ECSError>, x: i32, y: i32, character: char, fg: (u8, u8, u8), bg: (u8, u8, u8)) -> Result<&'w mut Entity, ECSError> {
    positioned(entity, x, y)?
        .insert_component(Renderer::new(rltk::to_cp437(character), RGB::named(fg), RGB::named(bg)))
}

pub fn player<'w>(entity: Result<&'w mut Entity, ECSError>, name: String, x: i32, y: i32) -> Result<&'w mut Entity, ECSError> {
    renderable(
        debugged(entity, name), 
        x, y, PLAYER_GLYPH, PLAYER_COLOR, BACKGROUND_COLOR)?
        .insert_component(Camera::new())
}