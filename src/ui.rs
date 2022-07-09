use std::collections::HashMap;

use rltk::Rltk;
use serde::Deserialize;

use crate::{vectors::Vector, theme::Theme, ecs::World};

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
    focusable: bool,
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
    Box,
    FullscreenOptions { options: Vec<UiOption>, selected: usize },
    Text { text: String }
}

impl UiAtomic {
    pub fn render(&self, world: &World, ctx: &mut Rltk, theme: &Theme, position: Vector, size: Vector) {
        match self {
            UiAtomic::Box => { ctx.draw_box(position.x, position.y, size.x, size.y, theme.ui_color, theme.background_color); },
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
            }
        }
    }

    pub fn tick(&self, world: &World, ctx: &mut Rltk) -> (UiAtomic, Option<String>) {
        match self {
            UiAtomic::Box => (self.clone(), None),
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
                        rltk::VirtualKeyCode::Return => (self.clone(), Some(options.get(selected).expect(&format!("a value in index {}", selected)).id.to_owned())),
                        _ => (self.clone(), None)
                    }
                } else {
                    (self.clone(), None)
                }
            },
            UiAtomic::Text { text: _text } => (self.clone(), None),
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

        for panel in self.panels.values().into_iter() {
            for id in panel.referenced_panels() {
                if !self.panels.contains_key(id) {
                    panic!("panels did not contain id: {}", id);
                }
            }
        }
    }

    pub fn get_panel(&self, id: String) -> Option<&UiPanel> {
        Some(self.panels.get(&id)?)
    }

    pub fn open_panels(&self) -> Vec<&UiPanel> {
        self.open.iter().filter_map(|id| { self.get_panel(id.to_owned()) }).collect()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct UiPanel {
    name: String,
    results: HashMap<String, UiAction>,
    components: Vec<UiComponent>,
    clears: bool,
    focused: i32
}

impl UiPanel {
    pub fn referenced_panels(&self) -> Vec<&String> {
        let mut references = Vec::new();

        for component in self.results.values().into_iter() {
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
                return Some(self.results.get(&action).unwrap());
            }
        }

        None
    }

    pub fn clears(&self) -> bool {
        self.clears
    }
}