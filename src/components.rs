use rltk::RGB;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::{
    map::{Map, RaycastMode},
    vectors::Vector,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Position {
    position: Vector,
    priority: u8,
}

impl Position {
    pub fn new(x: i32, y: i32, priority: u8) -> Self {
        Position {
            position: Vector::new(x, y),
            priority,
        }
    }

    pub fn coords(&self) -> Vector {
        self.position
    }

    pub fn priority(&self) -> u8 {
        self.priority
    }

    pub fn try_move(&mut self, map: &Map, delta: Vector) -> bool {
        self.try_set(map, self.position + delta)
    }

    pub fn try_set(&mut self, map: &Map, new_position: Vector) -> bool {
        if let Some(tile) = map.get(&new_position) {
            if tile.walkable() {
                self.position = new_position;

                return true;
            }
        }

        false
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum DebugLevel {
    None = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    Critical = 4,
}

pub struct DebugMessage {
    pub level: DebugLevel,
    pub reason: String,
    pub message: String,
}

impl DebugMessage {
    pub fn new(level: DebugLevel, reason: String, message: String) -> Self {
        DebugMessage {
            level,
            reason,
            message,
        }
    }
}

impl Display for DebugMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}): {}", self.reason, self.level as u8, self.message)
    }
}

pub struct Debug {
    pub max_level: DebugLevel,
    pub messages: HashMap<String, DebugMessage>,
}


impl Debug {
    pub fn new() -> Self {
        Debug {
            max_level: DebugLevel::None,
            messages: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.max_level = DebugLevel::None;
        self.messages.clear();
    }

    pub fn add_message(&mut self, level: DebugLevel, reason: String, message: String) {
        if level >= self.max_level {
            self.max_level = level;
        }

        self.messages
            .insert(reason.to_owned(), DebugMessage::new(level, reason, message));
    }

    pub fn count(&self) -> usize {
        self.messages.keys().len()
    }
}

pub struct Named {
    pub name: String,
}

impl Named {
    pub fn new(name: String) -> Self {
        Named { name }
    }
}

pub struct Renderer {
    glyph: rltk::FontCharType,
    fg: Option<RGB>,
    bg: Option<RGB>,
}

impl Renderer {
    pub fn new(glyph: rltk::FontCharType, fg: Option<RGB>, bg: Option<RGB>) -> Self {
        Renderer { glyph, fg, bg }
    }

    pub fn glyph(&self) -> rltk::FontCharType {
        self.glyph
    }

    pub fn fg(&self) -> Option<RGB> {
        self.fg
    }

    pub fn bg(&self) -> Option<RGB> {
        self.bg
    }
}

pub struct Camera {}

impl Camera {
    pub fn new() -> Self {
        Camera {}
    }
}

pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Player {}
    }
}

pub struct Viewshed {
    pub view_distance: f32,
    dirty: bool,
    visible: HashSet<Vector>,
}

impl Viewshed {
    pub fn new(view_distance: f32) -> Self {
        Viewshed {
            view_distance,
            dirty: true,
            visible: HashSet::new(),
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    /// Update the viewshed, returns true iff the view was recalculated
    pub fn update(&mut self, map: &mut Map, center: Vector, mark_discovered: bool, tick: Option<usize>) -> bool {
        if !self.dirty {
            return false;
        }

        self.dirty = false;
        self.visible = HashSet::new();

        let (top, bottom, left, right) = (
            (center.y as f32 - self.view_distance).floor() as i32,
            (center.y as f32 + self.view_distance).ceil() as i32,
            (center.x as f32 - self.view_distance).floor() as i32,
            (center.x as f32 + self.view_distance).ceil() as i32,
        );

        for x in left..=right {
            for y in top..=bottom {
                let pos = Vector::new(x, y);

                let dist = Vector::distance(&center, &pos);

                if dist <= self.view_distance {
                    // We are in the visiblity circle
                    let result = map.raycast(center, pos, RaycastMode::Visibility);

                    if let Some(hit_pos) = result.hit_position() {
                        if pos == hit_pos {
                            if let Some(tile) = map.get_mut(&pos) {
                                self.visible.insert(pos);
    
                                if mark_discovered {
                                    tile.discover();
                                    tile.see(tick.expect("a valid tick to set visible"));
                                }
                            }
                        }
                    } else if let Some(tile) = map.get_mut(&pos) {
                        self.visible.insert(pos);

                        if mark_discovered {
                            tile.discover();
                            tile.see(tick.expect("a valid tick to set visible"));
                        }
                    }
                }
            }
        }

        true
    }

    pub fn contains(&self, pos: &Vector) -> bool {
        self.visible.contains(pos)
    }

    pub fn visible(&self) -> HashSet<Vector> {
        self.visible.clone()
    }
}
