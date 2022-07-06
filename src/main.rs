#![allow(dead_code)]
#![feature(downcast_unchecked)]

use std::collections::HashMap;

use components::{Position, Viewshed};
use ecs::World;
use rltk::{Rltk, GameState, RGB};
use vectors::Vector;

mod map;
mod components;
mod ecs;
mod macros;
mod systems;
mod entities;
mod constants;
mod vectors;
mod transform;

struct KeyEntities {
    player: Option<usize>
}

impl KeyEntities {
    pub fn new() -> Self {
        KeyEntities { player: None }
    }

    pub fn set_player(&mut self, player: &ecs::Entity) {
        match self.player {
            Some(_) => panic!("Already had a player entity!"),
            None => { self.player = Some(player.id()); }
        }
    }

    pub fn player<'a>(&'a self, world: &'a World) -> &ecs::Entity {
        world.get(self.player.unwrap()).unwrap()
    }
}

struct Colors {
    ui_color: RGB,
    background_color: RGB,
    terrain_color_visible: RGB,
    terrain_color_discovered: RGB
}

impl Colors {
    pub fn new() -> Self {
        Colors {
            ui_color: RGB::named(constants::UI_COLOR),
            background_color: RGB::named(constants::BACKGROUND_COLOR),
            terrain_color_visible: RGB::named(constants::TERRAIN_COLOR_VISIBLE),
            terrain_color_discovered: RGB::named(constants::TERRAIN_COLOR_DISCOVERED),
        }
    }
}

struct State {
    world: ecs::World,
    colors: Colors
}

impl State {
    fn new() -> Self {
        State { world: ecs::World::new(), colors: Colors::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // Get Key Entities and Information
        let key_entities = self.world.get_resource::<KeyEntities>().unwrap();

        let player = key_entities.player(&self.world);
        // let offset = match self.world.query_one_entity(&query!(components::Position, components::Camera)) {
        let offset = match query_one!(self.world, components::Position, components::Camera) {
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

        // Before ticking systems, move player
        if let Some(map) = self.world.get_resource::<map::Map>() {
            if let Some(mut position) = player.get_component_mut::<components::Position>() {
                let moved = match ctx.key {
                    None => false,
                    Some(key) => match key {
                        rltk::VirtualKeyCode::W | rltk::VirtualKeyCode::Up => (*position).try_move(&*map, Vector::new(0, -1)),
                        rltk::VirtualKeyCode::A | rltk::VirtualKeyCode::Left => (*position).try_move(&*map, Vector::new(-1, 0)),
                        rltk::VirtualKeyCode::S | rltk::VirtualKeyCode::Down => (*position).try_move(&*map, Vector::new(0, 1)),
                        rltk::VirtualKeyCode::D | rltk::VirtualKeyCode::Right => (*position).try_move(&*map, Vector::new(1, 0)),
                        _ => false
                    }
                };

                if let Some(mut viewshed) = player.get_component_mut::<components::Viewshed>() {
                    if moved {
                        (*viewshed).mark_dirty();
                    }
                }
            }
        }

        // Tick world systems
        self.world.tick();

        // Draw Base UI Panes
        ctx.draw_box_double(0, 0, corner.x, corner.y, self.colors.ui_color, self.colors.background_color);
        ctx.draw_box_double(corner.x, 0, screen.x - corner.x - 1, corner.y, self.colors.ui_color, self.colors.background_color);
        ctx.draw_box_double(0, corner.y, screen.x - 1, screen.y - corner.y - 1, self.colors.ui_color, self.colors.background_color);

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

                            if tile.discovered() {
                                let terrain_color = match player.get_component::<Viewshed>() {
                                    Some(viewshed) => {
                                        if viewshed.contains(&final_position) {
                                            self.colors.terrain_color_visible
                                        } else {
                                            self.colors.terrain_color_discovered
                                        }
                                    },
                                    None => self.colors.terrain_color_discovered
                                };
                                
                                ctx.set(x, y, terrain_color, self.colors.background_color, glyph)
                            }
                        }
                    }
                }
            }
        }

        // Entity Rendering
        // First initialize all entity lists to empty vecs
        let mut entity_map: HashMap<vectors::Vector, (Position, &ecs::Entity)> = HashMap::new();

        let query = query!(components::Position, components::SingleGlyphRenderer);

        // Fill the lists according to priotity
        for entity in self.world.query_entities(&query) {
            if let Some(position) = entity.get_component::<components::Position>() {
                let pos = position.coords();
                if let Some((other_position, _)) = entity_map.get(&pos) {
                    if position.priority() > other_position.priority() {
                        entity_map.insert(pos, (*position, entity));
                    }
                } else {
                    entity_map.insert(pos, (*position, entity));
                }
            }
        }

        // Render all top prioritized entities
        for (position, entity) in entity_map.values() {
            if let Some(renderer) = entity.get_component::<components::SingleGlyphRenderer>() {
                let screen_pos = camera_transform.inverse_apply(position.coords());

                ctx.set(screen_pos.x, screen_pos.y, renderer.fg(), renderer.bg(), renderer.glyph());
            }
        }

        ctx.print_color(1, 0, RGB::named(rltk::GREEN), self.colors.background_color, format!("{} fps", ctx.fps));
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple(constants::SCREEN_SIZE.x, constants::SCREEN_SIZE.y)?
        .with_title("Ambrosia")
        .with_vsync(true)
        .build()?;

    let mut gs = State::new();

    let mut key_entities = KeyEntities::new();

    let _ = gs.world.insert_resource(map::Map::new(constants::MAP_SIZE.0, constants::MAP_SIZE.1)).unwrap();

    add_system!(gs.world, systems::ViewSystem::new(), -900);
    add_system!(gs.world, systems::DebugSystem::new(components::DebugLevel::None), -1000);

    let player = entities::player(gs.world.spawn(), "Hazel".into(), (constants::MAP_SIZE.0 / 2) as i32, (constants::MAP_SIZE.1 / 2) as i32).unwrap();
    key_entities.set_player(player);

    let _ = gs.world.insert_resource(key_entities).unwrap();

    rltk::main_loop(context, gs)
}
