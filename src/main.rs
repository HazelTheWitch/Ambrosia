#![allow(dead_code)]
#![feature(downcast_unchecked)]

use rltk::{Rltk, GameState, RGB};

mod rendering;
mod map;
mod components;
mod ecs;
mod macros;
mod systems;
mod entities;
mod constants;

struct State {
    world: ecs::World,
    map: map::Map
}

impl State {
    fn new() -> Self {
        State { world: ecs::World::new(), map: map::Map::new(1000, 1000) }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        self.world.tick();

        ctx.cls();
        ctx.draw_box_double(1, 1, 10, 10, RGB::from_u8(150, 150, 150), RGB::from_u8(10, 10, 10));
    }
}

fn main() {// -> rltk::BError
    // use rltk::RltkBuilder;

    // let context = RltkBuilder::simple(120, 80)?
    //     .with_title("Ambrosia")
    //     .build()?;

    let mut gs = State::new();

    add_system!(gs.world, systems::DebugSystem::new(components::DebugLevel::None));

    match entities::player(gs.world.spawn(), "Hazel".to_string(), 60, 40) {
        Ok(_) => println!("{}", "spawned player"),
        Err(_) => ()
    };
    
    gs.world.tick();

    // rltk::main_loop(context, gs)
}
