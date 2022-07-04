use std::{collections::HashMap, fmt::Display};
use rltk::RGB;

use crate::vectors::Vector;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    position: Vector
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { position: Vector::new(x, y) }
    }

    pub fn coords(&self) -> Vector {
        self.position
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum DebugLevel {
    None = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    Critical = 4
}

pub struct DebugMessage {
    pub level: DebugLevel,
    pub reason: String,
    pub message: String
}

impl DebugMessage {
    pub fn new(level: DebugLevel, reason: String, message: String) -> Self {
        DebugMessage { level, reason, message }
    }
}

impl Display for DebugMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}): {}", self.reason, self.level as u8, self.message)
    }
}

pub struct Debug {
    pub max_level: DebugLevel,
    pub messages: HashMap<String, DebugMessage>
}

impl Debug {
    pub fn new() -> Self {
        Debug { max_level: DebugLevel::None, messages: HashMap::new() }
    }

    pub fn clear(&mut self) {
        self.max_level = DebugLevel::None;
        self.messages = HashMap::new();
    }

    pub fn add_message(&mut self, level: DebugLevel, reason: String, message: String) {
        if level >= self.max_level {
            self.max_level = level;
        }

        self.messages.insert(reason.to_owned(), DebugMessage::new(level, reason, message));
    }

    pub fn count(&self) -> usize {
        self.messages.keys().len()
    }
}

pub struct Named {
    pub name: String
}

impl Named {
    pub fn new(name: String) -> Self {
        Named { name }
    }
}

pub struct Renderer {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB
}

impl Renderer {
    pub fn new(glyph: rltk::FontCharType, fg: RGB, bg: RGB) -> Self {
        Renderer { glyph, fg, bg }
    }
}

pub struct Camera { }

impl Camera {
    pub fn new() -> Self {
        Camera { }
    }
}