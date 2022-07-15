use std::collections::HashMap;

use rltk::Rltk;
use serde::Deserialize;

use crate::{vectors::{Vector, ZERO_VECTOR}, theme::Theme, ecs::{world::World, entity::Entity}, map::Map, query_one, components::{Position, Camera, Renderer, Player, Viewshed}, transform::Transform, query};

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum UiAction {
    Open { ids: Vec<String> },
    Set { ids: Vec<String> },
    Exit,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UiComponent {
    position: Vector, 
    size: Vector,
    atomics: Vec<UiAtomic>
}

impl UiComponent {
    pub fn render(&self, world: &World, ctx: &mut Rltk, theme: &Theme) {
        for atomic in self.atomics.iter() {
            atomic.render(world, ctx, theme, self.position, self.size); 
        }
    }

    pub fn tick(&mut self, world: &World, ctx: &mut Rltk) -> Option<String> {
        for atomic in self.atomics.iter_mut() {
            let (new_atomic, response) = atomic.tick(world, ctx);
            *atomic = new_atomic;

            if let Some(response) = response {
                return Some(response);
            }
        }

        None
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct UiOption {
    id: String,
    display: String
}

#[derive(Deserialize, Clone, Debug)]
pub enum UiAtomic {
    Box { hollow: bool, double: bool },
    FullscreenOptions { options: Vec<UiOption>, selected: usize },
    Text { text: String },
    WorldView { escape: String },
}

impl UiAtomic {
    pub fn render(&self, world: &World, ctx: &mut Rltk, theme: &Theme, position: Vector, size: Vector) {
        match self {
            UiAtomic::Box { hollow, double } => {
                if *hollow {
                    if *double {
                        ctx.draw_hollow_box_double(position.x, position.y, size.x, size.y, theme.ui_color, theme.background_color);
                    } else {
                        ctx.draw_hollow_box(position.x, position.y, size.x, size.y, theme.ui_color, theme.background_color);
                    }
                } else {
                    if *double {
                        ctx.draw_box_double(position.x, position.y, size.x, size.y, theme.ui_color, theme.background_color);
                    } else {
                        ctx.draw_box(position.x, position.y, size.x, size.y, theme.ui_color, theme.background_color);
                    }
                }
             },
            UiAtomic::FullscreenOptions { options, selected } => {
                let center = Vector::center(position, position + size);
                for (i, option) in options.iter().enumerate() {
                    if &i == selected {
                        ctx.print_color_centered_at(center.x, center.y + i as i32, theme.background_color, theme.ui_color, option.display.to_owned());
                    } else {
                        ctx.print_color_centered_at(center.x, center.y + i as i32, theme.ui_color, theme.background_color, option.display.to_owned());
                    }
                }
            },
            UiAtomic::Text { text } => {
                ctx.print_centered_at(position.x, position.y, text);
            },
            UiAtomic::WorldView { escape: _ } => {
                let offset = match query_one!(world, Position, Camera) {
                    Some(entity) => match entity.get_component::<Position>() {
                        Some(component) => component.coords() - Vector::center(position, position + size),
                        None => ZERO_VECTOR
                    },
                    None => ZERO_VECTOR
                };
 
                let camera_transform = Transform::new(offset);

                if let Some(map) = world.get_resource_mut::<Map>() {
                    map.render(world, ctx, theme, camera_transform, position, size);
                }

                // Entity Rendering
                // First initialize all entity lists to empty vecs
                let mut entity_map: HashMap<Vector, (Position, &Entity)> = HashMap::new();

                let query = query!(Position, Renderer);

                // Fill the lists according to priotity
                for entity in world.query_entities(&query) {
                    if let Some(position) = entity.get_component::<Position>() {
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
                    if let Some(renderer) = entity.get_component::<Renderer>() {
                        let screen_pos = camera_transform.inverse_apply(position.coords());

                        // TODO: Fix bounds problem, thats a later hazel problem
                        ctx.set(screen_pos.x, screen_pos.y, renderer.fg().unwrap_or(theme.background_color), renderer.bg().unwrap_or(theme.background_color), renderer.glyph());
                    }
                }
            }
        }
    }

    pub fn tick(&self, world: &World, ctx: &mut Rltk) -> (UiAtomic, Option<String>) {
        match self {
            UiAtomic::Box { hollow: _, double: _ } => (self.clone(), None),
            UiAtomic::FullscreenOptions { options, mut selected } => {
                if let Some(key) = ctx.key {
                    match key {
                        rltk::VirtualKeyCode::W | rltk::VirtualKeyCode::Up => { 
                            if selected == 0 { selected = options.len() - 1 } else { selected -= 1 } 
                            (UiAtomic::FullscreenOptions { options: options.clone(), selected }, None)
                        },
                        rltk::VirtualKeyCode::S | rltk::VirtualKeyCode::Down => { 
                            selected = (selected + 1) % options.len(); 
                            (UiAtomic::FullscreenOptions { options: options.clone(), selected }, None)
                        },
                        rltk::VirtualKeyCode::Return => {
                            match options.get(selected) {
                                Some(option) => (self.clone(), Some(option.id.to_owned())),
                                None => (self.clone(), None)
                            }
                        }
                        _ => (self.clone(), None)
                    }
                } else {
                    (self.clone(), None)
                }
            },
            UiAtomic::Text { text: _text } => (self.clone(), None),
            UiAtomic::WorldView { escape } => {
                if let (Some(map), Some(player)) = (world.get_resource::<Map>(), query_one!(world, Player)) {
                    if let Some(mut position) = player.get_component_mut::<Position>() {
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
        
                        if let Some(mut viewshed) = player.get_component_mut::<Viewshed>() {
                            if moved {
                                (*viewshed).mark_dirty();
                            }
                        }
                    }
                }

                world.tick();
                (self.clone(), match ctx.key {
                    Some(key) => match key {
                        rltk::VirtualKeyCode::Escape => Some(escape.to_owned()),
                        _ => None
                    },
                    None => None
                })
            }
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct UiMaster {
    open: Vec<String>,
    panels: HashMap<String, UiPanel>
}

impl UiMaster {
    pub fn verify(&self) {
        for id in self.open.iter() {
            if !self.panels.contains_key(id) {
                panic!("panels did not contain id: {}", id);
            }
        }

        for panel in self.panels.values() {
            for id in panel.referenced_panels() {
                if !self.panels.contains_key(id) {
                    panic!("panels did not contain id: {}", id);
                }
            }
        }
    }

    pub fn get_panel(&self, id: String) -> Option<&UiPanel> {
        self.panels.get(&id)
    }

    pub fn open_panels(&self) -> Vec<&UiPanel> {
        self.open.iter().filter_map(|id| { self.get_panel(id.to_owned()) }).collect()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct UiPanel {
    results: HashMap<String, UiAction>,
    components: Vec<UiComponent>,
    clears: bool,
}

impl UiPanel {
    pub fn referenced_panels(&self) -> Vec<&String> {
        let mut references = Vec::new();

        for component in self.results.values() {
            match component {
                UiAction::Open { ids } => {
                    for id in ids {
                        references.push(id);
                    }
                },
                _ => ()
            }
        }

        references
    }

    pub fn render(&self, world: &World, ctx: &mut Rltk) {
        if let Some(theme) = world.get_resource::<Theme>() {
            for component in self.components.iter() {
                component.render(world, ctx, &theme);
            }
        }
    }

    pub fn tick(&mut self, world: &World, ctx: &mut Rltk) -> Option<&UiAction> {
        for component in self.components.iter_mut() {
            if let Some(action) = component.tick(world, ctx) {
                return Some(self.results.get(&action)?);
            }
        }

        None
    }

    pub fn clears(&self) -> bool {
        self.clears
    }
}