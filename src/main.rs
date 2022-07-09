#![allow(dead_code)]
#![feature(downcast_unchecked)]

use std::process::exit;
use ecs::World;
use include_dir::{include_dir, Dir};
use rltk::{Rltk, GameState};
use ui::{UiAction, UiPanel, UiMaster};
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

pub struct KeyEntities {
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

impl Default for KeyEntities {
    fn default() -> Self {
        Self::new()
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

    pub fn open_by_ids(&mut self, ids: &[String]) {
        for i in ids.iter() {
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
        let action = self.tick_ui(ctx);

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

        if self.ui_panels.is_empty() {
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
    let _ = gs.world.insert_resource(systems::TickInfo::new()).unwrap();

    add_system!(gs.world, systems::TickSystem::new(), 1000);
    add_system!(gs.world, systems::ViewSystem::new(), -900);
    add_system!(gs.world, systems::DebugSystem::new(components::DebugLevel::None), -1000);

    let player = entities::player(gs.world.spawn(), "Hazel".into(), (constants::MAP_SIZE.0 / 2) as i32, (constants::MAP_SIZE.1 / 2) as i32).unwrap();
    key_entities.set_player(player);

    let _ = gs.world.insert_resource(key_entities).unwrap();

    rltk::main_loop(context, gs)
}
