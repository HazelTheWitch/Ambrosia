#![allow(dead_code)]
#![feature(downcast_unchecked)]

use std::{collections::{HashMap, VecDeque}, process::exit};

use components::{Position, Viewshed};
use ecs::World;
use include_dir::{include_dir, Dir};
use rltk::{Rltk, GameState, RGB};
use ui::{UiAction, UiPanel, UiMaster};
use vectors::Vector;
use theme::Theme;

mod map;
mod components;
mod ecs;
mod macros;
mod systems;
mod entities;
mod constants;
mod vectors;
mod transform;
mod theme;
mod input;
mod ui;

static RAWS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/raws");

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

    pub fn player<'a>(&'a self, world: &'a World) -> Option<&ecs::Entity> {
        world.get(self.player?)
    }

    pub fn player_unchecked<'a>(&'a self, world: &'a World) -> &ecs::Entity {
        world.get(self.player.unwrap()).unwrap()
    }
}

pub struct State {
    world: ecs::World,
    ui_panels: Vec<UiPanel>,
    ui_master: UiMaster,
}

impl State {
    fn new(ui: UiMaster) -> Self {
        let mut state = State { world: ecs::World::new(), ui_panels: Vec::new(), ui_master: ui.clone() };

        for panel in ui.open_panels() {
            state.open(panel);
        }

        state
    }

    pub fn open(&mut self, panel: &UiPanel) {
        self.ui_panels.push(panel.clone());
    }

    pub fn open_by_id(&mut self, id: String) {
        let panel = self.ui_master.get_panel(id).unwrap().clone();
        self.open(&panel);
    }

    pub fn open_by_ids(&mut self, ids: &Vec<String>) {
        for i in ids.to_owned().into_iter() {
            let id = i.to_owned();
            self.open_by_id(id);
        }
    }

    pub fn close(&mut self) {
        self.ui_panels.pop().unwrap();
    }

    pub fn close_all(&mut self) {
        self.ui_panels.clear();
    }

    fn tick_ui(&mut self, ctx: &mut Rltk) -> Option<&UiAction> {
        let mut starting_index = None;

        for (index, panel) in self.ui_panels.iter().enumerate() {
            if panel.clears() {
                starting_index = Some(index);
            }
        }

        if let Some(starting_index) = starting_index {
            if let Some(theme) = self.world.get_resource::<Theme>() {
                ctx.cls_bg(theme.background_color);
            } else {
                ctx.cls();
            }

            for i in starting_index..self.ui_panels.len() {
                if let Some(panel) = self.ui_panels.get(i) {
                    panel.render(&self.world, ctx)
                }
            }
        } else {
            for panel in self.ui_panels.iter() {
                panel.render(&self.world, ctx);
            }
        }

        self.ui_panels.last_mut().unwrap().tick(&self.world, ctx)
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // Get Key Entities and Information
        // let key_entities = self.world.get_resource::<KeyEntities>().unwrap();
        // let colors = self.world.get_resource::<Theme>().unwrap();

        // let player = key_entities.player(&self.world);
        // let offset = match query_one!(self.world, components::Position, components::Camera) {
        //     Some(entity) => {
        //         match entity.get_component::<components::Position>() {
        //             Some(position) => position.coords() - constants::CORNER_POINT_2,
        //             None => vectors::ZERO_VECTOR
        //         }
        //     },
        //     None => vectors::ZERO_VECTOR
        // };

        // let camera_transform = transform::Transform::new(offset);

        // let corner = constants::CORNER_POINT;
        // let screen = constants::SCREEN_SIZE;

        // // Before ticking systems, move player
        // if let (Some(map), Some(player)) = (self.world.get_resource::<map::Map>(), player) {
        //     if let Some(mut position) = player.get_component_mut::<components::Position>() {
        //         let moved = match ctx.key {
        //             None => false,
        //             Some(key) => match key {
        //                 rltk::VirtualKeyCode::W | rltk::VirtualKeyCode::Up => (*position).try_move(&*map, Vector::new(0, -1)),
        //                 rltk::VirtualKeyCode::A | rltk::VirtualKeyCode::Left => (*position).try_move(&*map, Vector::new(-1, 0)),
        //                 rltk::VirtualKeyCode::S | rltk::VirtualKeyCode::Down => (*position).try_move(&*map, Vector::new(0, 1)),
        //                 rltk::VirtualKeyCode::D | rltk::VirtualKeyCode::Right => (*position).try_move(&*map, Vector::new(1, 0)),
        //                 _ => false
        //             }
        //         };

        //         if let Some(mut viewshed) = player.get_component_mut::<components::Viewshed>() {
        //             if moved {
        //                 (*viewshed).mark_dirty();
        //             }
        //         }
        //     }
        // }

        // // Tick world systems
        // self.world.tick();

        // // Draw Base UI Panes
        // ctx.draw_box_double(0, 0, corner.x, corner.y, colors.ui_color, colors.background_color);
        // ctx.draw_box_double(corner.x, 0, screen.x - corner.x - 1, corner.y, colors.ui_color, colors.background_color);
        // ctx.draw_box_double(0, corner.y, screen.x - 1, screen.y - corner.y - 1, colors.ui_color, colors.background_color);

        // // Render world
        // // Draw Map
        // if let (Some(map), Some(player)) = (self.world.get_resource::<map::Map>(), player) {
        //     for x in 1..corner.x {
        //         for y in 1..corner.y {
        //             let final_position = camera_transform.apply(vectors::Vector::new(x, y));

        //             if map.in_bounds(&final_position) {
        //                 if let Some(tile) = map.get(&final_position) {
        //                     let glyph: rltk::FontCharType = {
        //                         if tile.walkable() {
        //                             rltk::to_cp437('.')
        //                         } else {
        //                             rltk::to_cp437('#')
        //                         }
        //                     };

        //                     if tile.discovered() {
        //                         let terrain_color = match player.get_component::<Viewshed>() {
        //                             Some(viewshed) => {
        //                                 if viewshed.contains(&final_position) {
        //                                     colors.terrain_color_visible
        //                                 } else {
        //                                     colors.terrain_color_discovered
        //                                 }
        //                             },
        //                             None => colors.terrain_color_discovered
        //                         };
                                
        //                         ctx.set(x, y, terrain_color, colors.background_color, glyph)
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }

        // // Entity Rendering
        // // First initialize all entity lists to empty vecs
        // let mut entity_map: HashMap<vectors::Vector, (Position, &ecs::Entity)> = HashMap::new();

        // let query = query!(components::Position, components::SingleGlyphRenderer);

        // // Fill the lists according to priotity
        // for entity in self.world.query_entities(&query) {
        //     if let Some(position) = entity.get_component::<components::Position>() {
        //         let pos = position.coords();
        //         if let Some((other_position, _)) = entity_map.get(&pos) {
        //             if position.priority() > other_position.priority() {
        //                 entity_map.insert(pos, (*position, entity));
        //             }
        //         } else {
        //             entity_map.insert(pos, (*position, entity));
        //         }
        //     }
        // }

        // // Render all top prioritized entities
        // for (position, entity) in entity_map.values() {
        //     if let Some(renderer) = entity.get_component::<components::SingleGlyphRenderer>() {
        //         let screen_pos = camera_transform.inverse_apply(position.coords());

        //         ctx.set(screen_pos.x, screen_pos.y, renderer.fg(), renderer.bg(), renderer.glyph());
        //     }
        // }

        // ctx.print_color(1, 0, RGB::named(rltk::GREEN), colors.background_color, format!("{} fps", ctx.fps));

        let action = self.tick_ui(ctx).clone();

        match action {
            Some(action) => match action {
                UiAction::Open { ids } => {
                    let ids = ids.clone();
                    self.open_by_ids(&ids);
                },
                UiAction::Set { ids } => {
                    let ids = ids.clone();

                    self.close_all();

                    self.open_by_ids(&ids);
                }
                UiAction::Exit => { self.close(); },
            },
            None => ()
        }

        if self.ui_panels.len() == 0 {
            exit(0); 
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple(constants::SCREEN_SIZE.x, constants::SCREEN_SIZE.y)?
        .with_title("Ambrosia")
        .with_vsync(false)
        .build()?;

    let ui_master: ui::UiMaster = serde_json::from_str(RAWS.get_file("ui.json").unwrap().contents_utf8().unwrap()).unwrap();

    ui_master.verify();

    let mut gs = State::new(ui_master);

    let mut key_entities = KeyEntities::new();

    let _ = gs.world.insert_resource(Theme::new()).unwrap();

    let _ = gs.world.insert_resource(map::Map::new(constants::MAP_SIZE.0, constants::MAP_SIZE.1)).unwrap();

    add_system!(gs.world, systems::ViewSystem::new(), -900);
    add_system!(gs.world, systems::DebugSystem::new(components::DebugLevel::None), -1000);

    let player = entities::player(gs.world.spawn(), "Hazel".into(), (constants::MAP_SIZE.0 / 2) as i32, (constants::MAP_SIZE.1 / 2) as i32).unwrap();
    key_entities.set_player(player);

    let _ = gs.world.insert_resource(key_entities).unwrap();

    rltk::main_loop(context, gs)
}
