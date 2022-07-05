#![allow(dead_code)]
#![feature(downcast_unchecked)]

use rltk::{Rltk, GameState};

mod map;
mod components;
mod ecs;
mod macros;
mod systems;
mod entities;
mod constants;
mod vectors;
mod transform;

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

        // Get Key Entities and Information
        let _player = self.world.query_one_entity(&ecs::Query::new().include::<components::Player>());
        let offset = match self.world.query_one_entity(&ecs::Query::new().include::<components::Position>().include::<components::Camera>()) {
            Some(entity) => {
                match entity.get_component::<components::Position>() {
                    Some(position) => position.coords() - constants::CORNER_POINT_2,
                    None => vectors::ZERO_VECTOR
                }
            },
            None => vectors::ZERO_VECTOR
        };

        let camera_transform = transform::Transform::new(offset);

        let corner = constants::CORNER_POINT;
        let screen = constants::SCREEN_SIZE;

        let ui_color = rltk::RGB::named(constants::UI_COLOR);
        let background_color = rltk::RGB::named(constants::BACKGROUND_COLOR);
        let terrain_color = rltk::RGB::named(constants::TERRAIN_COLOR);

        // Draw Base UI Panes
        ctx.draw_box_double(0, 0, corner.x, corner.y, ui_color, background_color);
        ctx.draw_box_double(corner.x, 0, screen.x - corner.x - 1, corner.y, ui_color, background_color);
        ctx.draw_box_double(0, corner.y, screen.x - 1, screen.y - corner.y - 1, ui_color, background_color);

        // Draw Map
        if let Some(map) = self.world.get_resource::<map::Map>() {
            for x in 1..corner.x {
                for y in 1..corner.y {
                    let final_position = camera_transform.apply(vectors::Vector::new(x, y));

                    if map.in_bounds(&final_position) {
                        if let Some(tile) = map.get(&final_position) {
                            let glyph: rltk::FontCharType = {
                                if tile.walkable() {
                                    rltk::to_cp437('.')
                                } else {
                                    rltk::to_cp437('#')
                                }
                            };

                            ctx.set(x, y, terrain_color, background_color, glyph);
                        }
                    }
                }
            }
        }

        // Entity Rendering
        // First initialize all entity lists to empty vecs
        let mut entity_lists: Vec<Vec<&ecs::Entity>> = Vec::with_capacity(256);

        for _ in 0..256 {
            entity_lists.push(Vec::with_capacity(0))
        }

        let query = ecs::Query::new().include::<components::Position>().include::<components::SingleGlyphRenderer>();

        // Fill the lists according to priotity
        for entity in self.world.query_entities(&query) {
            if let Some(position) = entity.get_component::<components::Position>() {
                let list = &mut entity_lists[position.priority() as usize];
                list.push(entity);
            }
        }

        // Render from least prioritized to most
        for list in entity_lists {
            for entity in list {
                if let (Some(position), Some(renderer)) = (entity.get_component::<components::Position>(), entity.get_component::<components::SingleGlyphRenderer>()) {
                    let screen_pos = camera_transform.inverse_apply(position.coords());

                    ctx.set(screen_pos.x, screen_pos.y, renderer.fg().clone(), renderer.bg().clone(), renderer.glyph().clone());
                }
            }
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple(constants::SCREEN_SIZE.x, constants::SCREEN_SIZE.y)?
        .with_title("Ambrosia")
        .with_fps_cap(30.0)
        .build()?;

    let mut gs = State::new();

    let _ = gs.world.insert_resource(map::Map::new(constants::MAP_SIZE.0, constants::MAP_SIZE.1));

    add_system!(gs.world, systems::DebugSystem::new(components::DebugLevel::None));

    let _ = entities::player(gs.world.spawn(), "Hazel".to_string(), (constants::MAP_SIZE.0 / 2) as i32, (constants::MAP_SIZE.1 / 2) as i32);

    rltk::main_loop(context, gs)
}
