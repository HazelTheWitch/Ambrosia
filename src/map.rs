use crate::clamp;

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
    opaqueness: u8
}

impl Tile {
    pub fn new(strength: u8, opaqueness: u8) -> Self {
        Tile { strength, opaqueness }
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
}

pub struct Map {
    width: usize,
    height: usize,
    data: Vec<Tile>
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut map = Map::empty(width, height);

        for x in 0..(width as i32) {
            if let Some(tile) = map.get_mut(x, 0) {
                tile.set_both(255);
            }
            
            if let Some(tile) = map.get_mut(x, height as i32 - 1) {
                tile.set_both(255);
            }
        }

        for y in 0..(height as i32) {
            if let Some(tile) = map.get_mut(0, y) {
                tile.set_both(255);
            }
            
            if let Some(tile) = map.get_mut(width as i32 - 1, y) {
                tile.set_both(255);
            }
        }

        map
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Map { width, height, data: vec![Tile::ground(); width * height]}
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&Tile> {
        let index = self.coords_to_index(x, y)?;
        self.data.get(index)
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        let index = self.coords_to_index(x, y)?;
        self.data.get_mut(index)
    }

    pub unsafe fn get_unchecked(&self, x: i32, y: i32) -> &Tile {
        let index = self.coords_to_index_unchecked(x, y);
        self.data.get_unchecked(index)
    }

    pub unsafe fn get_unchecked_mut(&mut self, x: i32, y: i32) -> &Tile {
        let index = self.coords_to_index_unchecked(x, y);
        self.data.get_unchecked_mut(index)
    }

    fn coords_to_index(&self, x: i32, y: i32) -> Option<usize> {
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

    unsafe fn coords_to_index_unchecked(&self, x: i32, y: i32) -> usize {
        (y * self.width as i32 + x) as usize
    }

    unsafe fn index_to_coords_unchecked(&self, index: usize) -> (i32, i32) {
        ((index % self.width) as i32, (index / self.width) as i32)
    }

    fn coords_to_nearest_index(&self, x: i32, y: i32) -> usize {
        unsafe {
            self.coords_to_index_unchecked(clamp!(x, 0, self.width as i32), clamp!(y, 0, self.height as i32))
        }
    }
}
