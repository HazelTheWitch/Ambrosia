#![allow(dead_code)]
#![feature(downcast_unchecked)]

use rltk::{Rltk, GameState, RGB};

mod rendering;
mod map;
mod components;
mod ecs;

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
        ctx.cls();
        ctx.draw_box_double(1, 1, 10, 10, RGB::from_u8(150, 150, 150), RGB::from_u8(10, 10, 10));
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple(120, 80)?
        .with_title("Ambrosia")
        .build()?;

    let mut gs = State::new();

    if let Some(e) = gs.world.spawn() {
        e.insert(1.2);
    }

    if let Some(e) = gs.world.spawn() {
        e.insert("Hello World!");
    }

    if let Some(e) = gs.world.spawn() {
        e.insert(3.4);
        e.insert("Hello ECS!");
    }

    gs.world.iter().for_each(|e| {
        println!("{}", e.id())
    });

    println!();

    for e in gs.world.query(&ecs::Query::new().include::<&str>()) {
        println!("{}", e.id());
    }

    println!();

    for e in gs.world.query(&ecs::Query::new().include::<f64>()) {
        if let Some(num) = e.get_component::<f64>() {
            println!("{}", num);
        }
    }

    println!();

    gs.world.query_mut(&ecs::Query::new().include::<f64>().include::<&str>()).for_each(|e| {
        if let Some(string) = e.get_component_mut::<&str>() {
            if let Some(num) = e.get_component::<f64>() {
                *string = &*format!("{}", num);
            }
        }
    });

    println!();

    for e in gs.world.query(&ecs::Query::new().include::<&str>()) {
        if let Some(string) = e.get_component::<&str>() {
            println!("{}", string);
        }
    }


    rltk::main_loop(context, gs)
}
