use std::fmt::Display;

use rltk::Rltk;

use crate::{clamp, constants::MAP_SIZE, vectors::Vector, ecs::world::World, theme::Theme, transform::Transform, systems::TickInfo};

/// Represents a tile
/// Strength:
///     0:       The tile is walkable
///     1 - 255: The tile is not broken and therefore is not walkable
/// Opaqueness:
/// This value is subtracted from the light value to determine if a tile is visible
///     0:       The tile is copletely visible and lets through all lights
///     1 - 255: The tile blocks out some light but will be visible if hit with a ray
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Tile {
    strength: u8,
    opaqueness: u8,
    discovered: bool,
    last_seen: Option<usize>,
}

impl Tile {
    pub fn new(strength: u8, opaqueness: u8) -> Self {
        Tile {
            strength,
            opaqueness,
            discovered: false,
            last_seen: None,
        }
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

    pub fn visible(&self, current_tick: Option<usize>) -> bool {
        match self.last_seen {
            Some(tick) => match current_tick {
                Some(current) => tick == current,
                None => false
            },
            None => false
        }
    }

    pub fn see(&mut self, tick: usize) {
        self.last_seen = Some(tick);
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile(s={},o={})", self.strength, self.opaqueness)
    }
}

pub struct RaycastResult<'t> {
    hit_position: Option<Vector>,
    start: Vector,
    distance: Option<f32>,
    path: Vec<(Vector, &'t Tile)>
}

impl <'t> RaycastResult<'t> {
    fn new(start: Vector, end: Option<Vector>, path: Vec<(Vector, &'t Tile)>) -> Self {
        match end {
            Some(end) => RaycastResult { hit_position: Some(end), start, distance: Some(Vector::distance(&start, &end)), path },
            None => RaycastResult { hit_position: None, start, distance: None, path }
        }
    }

    pub fn hit(&self) -> bool {
        self.hit_position.is_some()
    }

    pub fn hit_position(&self) -> Option<Vector> {
        self.hit_position
    }

    pub fn start(&self) -> Vector {
        self.start
    }

    pub fn distance(&self) -> Option<f32> {
        self.distance
    }

    pub fn path(&self) -> &[(Vector, &Tile)] {
        self.path.as_ref()
    }
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    data: Vec<Tile>,
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

    pub fn render(&self, world: &World, ctx: &mut Rltk, theme: &Theme, transform: Transform, position: Vector, size: Vector) {
        let tick = world.get_resource::<TickInfo>().expect("a TickInfo object in world").last_view_update_tick();

        for x in position.x..(position.x + size.x) {
            for y in position.y..(position.y + size.y) {
                let final_position = transform.apply(Vector::new(x, y));

                if let Some(tile) = self.get(&final_position) {
                    if tile.discovered() {
                        let glyph: rltk::FontCharType = {
                            if tile.walkable() {
                                rltk::to_cp437('.')
                            } else {
                                rltk::to_cp437('#')
                            }
                        };

                        let terrain_color = if tile.visible(tick) {
                            theme.terrain_color_visible
                        } else {
                            theme.terrain_color_discovered
                        };

                        ctx.set(x, y, terrain_color, theme.background_color, glyph);
                    }
                }
            }
        }
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Map {
            width,
            height,
            data: vec![Tile::ground(); width * height],
        }
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

        let index = (y * self.width as i32 + x) as usize;

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
            self.coords_to_index_unchecked(&Vector::new(
                clamp!(x, 0, self.width as i32),
                clamp!(y, 0, self.height as i32),
            ))
        }
    }

    pub fn in_bounds(&self, position: &Vector) -> bool {
        let (x, y) = position.tuple();
        (0 <= x && x < self.width as i32) && (0 <= y && y < self.height as i32)
    }

    pub fn raycast(&self, start: Vector, end: Vector, mode: RaycastMode) -> RaycastResult {
        let mut light: u8 = 255;
        let mut last: Option<Vector> = None;

        let mut path: Vec<(Vector, &Tile)> = Vec::new();

        for point in Vector::line(&start, &end) {
            if let Some(tile) = self.get(&point) {
                path.push((point, tile));

                match mode {
                    RaycastMode::Visibility => {
                        if tile.opaqueness >= light {
                            return RaycastResult::new(start, Some(point), path);
                        }

                        light -= tile.opaqueness;
                    },
                    RaycastMode::Walkable => {
                        if !tile.walkable() {
                            return match last {
                                Some(last) => RaycastResult::new(start, Some(last), path),
                                None => RaycastResult::new(start, Some(point), path)
                            };
                        }

                        last = Some(point);
                    }
                }
            } else {
                return RaycastResult::new(start, Some(point), path);
            }
        }

        RaycastResult::new(start, None, path)
    }
}

pub enum RaycastMode {
    Walkable,
    Visibility,
}
