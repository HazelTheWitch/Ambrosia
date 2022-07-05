#![allow(dead_code)]
#![feature(downcast_unchecked)]

use std::collections::HashMap;

use components::{Position, Viewshed};
use rltk::{GameState, Rltk};
use vectors::Vector;

mod components;
mod constants;
mod ecs;
mod entities;
mod macros;
mod map;
mod systems;
mod transform;
mod vectors;

struct State {
    world: ecs::World,
}

impl State {
    fn new() -> Self {
        State {
            world: ecs::World::new(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // Get Key Entities and Information
        let player_query = ecs::Query::new().include::<components::Player>();
        let player = self.world.query_one_entity(&player_query);
        let offset = match self.world.query_one_entity(
            &ecs::Query::new()
                .include::<components::Position>()
                .include::<components::Camera>(),
        ) {
            Some(entity) => match entity.get_component::<components::Position>() {
                Some(position) => position.coords() - constants::CORNER_POINT_2,
                None => vectors::ZERO_VECTOR,
            },
            None => vectors::ZERO_VECTOR,
        };

        let camera_transform = transform::Transform::new(offset);

        let corner = constants::CORNER_POINT;
        let screen = constants::SCREEN_SIZE;

        let ui_color = rltk::RGB::named(constants::UI_COLOR);
        let background_color = rltk::RGB::named(constants::BACKGROUND_COLOR);
        let terrain_color_visible = rltk::RGB::named(constants::TERRAIN_COLOR_VISIBLE);
        let terrain_color_discovered = rltk::RGB::named(constants::TERRAIN_COLOR_DISCOVERED);

        // Before ticking systems, move player
        if let Some(map) = self.world.get_resource::<map::Map>() {
            if let Some(player) = player {
                if let Some(position) = player.get_component_mut::<components::Position>() {
                    let moved = match ctx.key {
                        None => false,
                        Some(key) => match key {
                            rltk::VirtualKeyCode::W => position.try_move(map, Vector::new(0, -1)),
                            rltk::VirtualKeyCode::A => position.try_move(map, Vector::new(-1, 0)),
                            rltk::VirtualKeyCode::S => position.try_move(map, Vector::new(0, 1)),
                            rltk::VirtualKeyCode::D => position.try_move(map, Vector::new(1, 0)),
                            _ => false,
                        },
                    };

                    if let Some(viewshed) = player.get_component_mut::<components::Viewshed>() {
                        if moved {
                            viewshed.mark_dirty();
                        }
                    }
                }
            }
        }

        // Tick world systems
        self.world.tick();

        // Draw Base UI Panes
        ctx.draw_box_double(0, 0, corner.x, corner.y, ui_color, background_color);
        ctx.draw_box_double(
            corner.x,
            0,
            screen.x - corner.x - 1,
            corner.y,
            ui_color,
            background_color,
        );
        ctx.draw_box_double(
            0,
            corner.y,
            screen.x - 1,
            screen.y - corner.y - 1,
            ui_color,
            background_color,
        );

        // Render world
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

                            if let Some(player) = player {
                                if tile.discovered() {
                                    let terrain_color = match player.get_component::<Viewshed>() {
                                        Some(viewshed) => {
                                            if viewshed.contains(&final_position) {
                                                terrain_color_visible
                                            } else {
                                                terrain_color_discovered
                                            }
                                        }
                                        None => terrain_color_discovered,
                                    };

                                    ctx.set(x, y, terrain_color, background_color, glyph)
                                }
                            }
                        }
                    }
                }
            }
        }

        // Entity Rendering
        // First initialize all entity lists to empty vecs
        let mut entity_map: HashMap<vectors::Vector, (&Position, &ecs::Entity)> = HashMap::new();

        let query = ecs::Query::new()
            .include::<components::Position>()
            .include::<components::SingleGlyphRenderer>();

        // Fill the lists according to priotity
        for entity in self.world.query_entities(&query) {
            if let Some(position) = entity.get_component::<components::Position>() {
                let pos = position.coords();
                if let Some((other_position, _)) = entity_map.get(&pos) {
                    if position.priority() > other_position.priority() {
                        entity_map.insert(pos, (position, entity));
                    }
                } else {
                    entity_map.insert(pos, (position, entity));
                }
            }
        }

        // Render all top prioritized entities
        for (position, entity) in entity_map.values() {
            if let Some(renderer) = (*entity).get_component::<components::SingleGlyphRenderer>() {
                let screen_pos = camera_transform.inverse_apply(position.coords());

                ctx.set(
                    screen_pos.x,
                    screen_pos.y,
                    renderer.fg().clone(),
                    renderer.bg().clone(),
                    renderer.glyph().clone(),
                );
            }
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple(constants::SCREEN_SIZE.x, constants::SCREEN_SIZE.y)?
        .with_title("Ambrosia")
        // .with_fps_cap(30.0)
        .build()?;

    let mut gs = State::new();

    let _ = gs
        .world
        .insert_resource(map::Map::new(constants::MAP_SIZE.0, constants::MAP_SIZE.1));

    add_system!(gs.world, systems::ViewSystem::new(), -900);
    add_system!(
        gs.world,
        systems::DebugSystem::new(components::DebugLevel::None),
        -1000
    );

    let _ = entities::player(
        gs.world.spawn(),
        "Hazel".to_string(),
        (constants::MAP_SIZE.0 / 2) as i32,
        (constants::MAP_SIZE.1 / 2) as i32,
    );

    rltk::main_loop(context, gs)
}
