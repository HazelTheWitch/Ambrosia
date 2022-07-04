#![allow(dead_code)]
#![feature(downcast_unchecked)]

use rltk::{Rltk, GameState, RGB};

mod map;
mod components;
mod ecs;
mod macros;
mod systems;
mod entities;
mod constants;
mod vectors;

struct State {
    world: ecs::World
}

impl State {
    fn new() -> Self {
        State { world: ecs::World::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        self.world.tick();

        ctx.cls();

        // Draw Base UI Panes
        let corner = constants::CORNER_POINT;
        let screen = constants::SCREEN_SIZE;

        ctx.draw_box_double(0, 0, corner.x, corner.y, RGB::from_u8(150, 150, 150), RGB::from_u8(10, 10, 10));
        ctx.draw_box_double(corner.x, 0, screen.x - corner.x - 1, corner.y, RGB::from_u8(150, 150, 150), RGB::from_u8(10, 10, 10));
        ctx.draw_box_double(0, corner.y, screen.x - 1, screen.y - corner.y - 1, RGB::from_u8(150, 150, 150), RGB::from_u8(10, 10, 10));

        // Draw Map
        if let Some(map) = self.world.get_resource::<map::Map>() {
            let offset = match self.world.query_one_entity(&ecs::Query::new().include::<components::Position>().include::<components::Camera>()) {
                Some(entity) => {
                    match entity.get_component::<components::Position>() {
                        Some(position) => position.coords() - constants::SCREEN_SIZE_2,
                        None => vectors::ZERO_VECTOR
                    }
                },
                None => vectors::ZERO_VECTOR
            };
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple(constants::SCREEN_SIZE.x, constants::SCREEN_SIZE.y)?
        .with_title("Ambrosia")
        .build()?;

    let mut gs = State::new();

    let _ = gs.world.insert_resource(map::Map::new(1000, 1000));

    add_system!(gs.world, systems::DebugSystem::new(components::DebugLevel::None));

    let _ = entities::player(gs.world.spawn(), "Hazel".to_string(), 60, 40);

    rltk::main_loop(context, gs)
}
