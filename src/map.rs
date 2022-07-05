use std::fmt::Display;

use crate::{clamp, vectors::Vector, constants::MAP_SIZE};

/// Represents a tile
/// Strength:
///     0:       The tile is walkable
///     1 - 255: The tile is not broken and therefore is not walkable
/// Opaqueness:
/// This value is subtracted from the light value to determine if a tile is visible
///     0:       The tile is copletely visible and lets through all lights
///     1 - 255: The tile blocks out some light but will be visible if hit with a ray
#[derive(PartialEq, Copy, Clone)]
pub struct Tile {
    strength: u8,
    opaqueness: u8,
    discovered: bool
}

impl Tile {
    pub fn new(strength: u8, opaqueness: u8) -> Self {
        Tile { strength, opaqueness, discovered: false }
    }

    pub fn ground() -> Self {
        Tile::new(0, 0)
    }

    pub fn wall() -> Self {
        Tile::new(255, 255)
    }

    pub fn window() -> Self {
        Tile::new(255, 0)
    }

    pub fn set_both(&mut self, value: u8) {
        self.strength = value;
        self.opaqueness = value;
    }

    pub fn walkable(&self) -> bool {
        self.strength == 0
    }

    pub fn discover(&mut self) {
        self.discovered = true;
    }

    pub fn discovered(&self) -> bool {
        self.discovered
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile(s={},o={})", self.strength, self.opaqueness)
    }
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    data: Vec<Tile>
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut map = Map::empty(width, height);

        for x in 0..(width as i32) {
            if let Some(tile) = map.get_mut(&Vector::from((x, 0))) {
                tile.set_both(255);
            }
            
            if let Some(tile) = map.get_mut(&Vector::from((x, height as i32 - 1))) {
                tile.set_both(255);
            }
        }

        for y in 0..(height as i32) {
            if let Some(tile) = map.get_mut(&Vector::from((0, y))) {
                tile.set_both(255);
            }
            
            if let Some(tile) = map.get_mut(&Vector::from((width as i32 - 1, y))) {
                tile.set_both(255);
            }
        }

        let mut rng = rltk::RandomNumberGenerator::new();

        let middle = Vector::new((MAP_SIZE.0 / 2) as i32, (MAP_SIZE.1 / 2) as i32);

        for _i in 0..(MAP_SIZE.0 * MAP_SIZE.1 / 5) {
            let x = rng.roll_dice(1, MAP_SIZE.0 as i32 - 1);
            let y = rng.roll_dice(1, MAP_SIZE.1 as i32 - 1);

            let pos = Vector::new(x, y);

            if pos != middle {
                if let Some(tile) = map.get_mut(&pos) {
                    tile.set_both(255);
                }
            }
        }

        map
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Map { width, height, data: vec![Tile::ground(); width * height]}
    }

    pub fn get(&self, position: &Vector) -> Option<&Tile> {
        if !self.in_bounds(position) {
            return None;
        }

        unsafe {
            let index = self.coords_to_index_unchecked(position);
        
            self.data.get(index)
        }
    }

    pub fn get_mut(&mut self, position: &Vector) -> Option<&mut Tile> {
        if !self.in_bounds(position) {
            return None;
        }

        unsafe {
            let index = self.coords_to_index_unchecked(position);
        
            self.data.get_mut(index)
        }
    }

    pub unsafe fn get_unchecked(&self, position: &Vector) -> &Tile {
        let index = self.coords_to_index_unchecked(position);
        self.data.get_unchecked(index)
    }

    pub unsafe fn get_unchecked_mut(&mut self, position: &Vector) -> &Tile {
        let index = self.coords_to_index_unchecked(position);
        self.data.get_unchecked_mut(index)
    }

    fn coords_to_index(&self, position: &Vector) -> Option<usize> {
        let (x, y) = position.tuple();
        
        let index =  (y * self.width as i32 + x) as usize;

        if index < self.data.len() {
            Some(index)
        } else {
            None
        }
    }

    fn index_to_coords(&self, index: usize) -> Option<(i32, i32)> {
        if index < self.data.len() {
            Some(((index % self.width) as i32, (index / self.width) as i32))
        } else {
            None
        }
    }

    unsafe fn coords_to_index_unchecked(&self, position: &Vector) -> usize {
        let (x, y) = position.tuple();
        
        (y * self.width as i32 + x) as usize
    }

    unsafe fn index_to_coords_unchecked(&self, index: usize) -> (i32, i32) {
        ((index % self.width) as i32, (index / self.width) as i32)
    }

    fn coords_to_nearest_index(&self, position: &Vector) -> usize {
        let (x, y) = position.tuple();

        unsafe {
            self.coords_to_index_unchecked(&Vector::new(clamp!(x, 0, self.width as i32), clamp!(y, 0, self.height as i32)))
        }
    }

    pub fn in_bounds(&self, position: &Vector) -> bool {
        let (x, y) = position.tuple();
        (0 <= x && x < self.width as i32) && (0 <= y && y < self.height as i32)
    }

    pub fn raycast(&self, start: &Vector, end: &Vector, mode: RaycastMode) -> (bool, f32) {
        let mut light: u8 = 255;

        for point in Vector::line(start, end) {
            if let Some(tile) = self.get(&point) {
                match mode {
                    RaycastMode::Visibility => {
                        if tile.opaqueness > light {
                            return (true, start.distance(&point));
                        }

                        light -= tile.opaqueness;
                    },
                    RaycastMode::Walkable => {
                        if !tile.walkable() {
                            return (true, start.distance(&point));
                        }
                    }
                }
            } else {
                return (true, start.distance(&point));
            }
        }

        (false, start.distance(end))
    }
}

pub enum RaycastMode {
    Walkable,
    Visibility
}
