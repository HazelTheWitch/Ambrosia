use crate::components::*;
use crate::ecs::*;
use crate::map::Map;

pub struct DebugSystem {
    pub min_level: DebugLevel,
}

impl DebugSystem {
    pub fn new(level: DebugLevel) -> Self {
        DebugSystem { min_level: level }
    }

    pub fn set_level(&mut self, level: DebugLevel) {
        self.min_level = level;
    }
}

impl System for DebugSystem {
    fn execute(&self, world: &World) {
        for e in world.query_entities(&Query::new().include::<Debug>()) {
            if let Some(mut debug) = e.get_component_mut::<Debug>() {
                let name: String = match e.get_component::<Named>() {
                    Some(named) => named.name.to_string(),
                    None => format!("Entity({:?})", e.id()),
                };

                if debug.max_level >= self.min_level && debug.count() > 0 {
                    println!("{}", name);

                    for message in debug.messages.values() {
                        println!("    {}", message);
                    }
                }

                debug.clear();
            }
        }
    }
}

pub struct ViewSystem {}

impl ViewSystem {
    pub fn new() -> Self {
        ViewSystem {}
    }
}

impl System for ViewSystem {
    fn execute(&self, world: &World) {
        let query = Query::new().include::<Viewshed>().include::<Position>();

        if let Some(mut map) = world.get_resource_mut::<Map>() {
            for entity in world.query_entities(&query) {
                if let (Some(mut viewshed), Some(position)) = (
                    entity.get_component_mut::<Viewshed>(),
                    entity.get_component::<Position>(),
                ) {
                    let is_player = entity.has_component::<Player>();
                    if let Some(mut tick_info) = world.get_resource_mut::<TickInfo>() {
                        if (*viewshed).dirty() && is_player {
                            tick_info.update_last_view_update();   
                        }
                     
                        (*viewshed).update(&mut *map, position.coords(), is_player, tick_info.last_view_update_tick());
                    }
                }
            }
        }
    }
}

pub struct TickInfo {
    current_tick: Option<usize>,
    behaviour_tick: bool,
    last_view_update_tick: Option<usize>
}

impl TickInfo {
    pub fn new() -> Self {
        TickInfo {
            current_tick: None,
            behaviour_tick: false,
            last_view_update_tick: None
        }
    }

    pub fn current_tick(&self) -> Option<usize> {
        self.current_tick
    }

    pub fn behaviour_tick(&self) -> bool {
        todo!()
    }

    pub fn offset_tick(&self, offset: i32) -> Option<usize> {
        if offset >= 0 {
            Some(self.current_tick? + offset as usize)
        } else {
            if -offset as usize > self.current_tick? {
                None
            } else {
                Some(self.current_tick? + offset as usize)
            }
        }
    }

    fn increment_tick(&mut self) {
        self.current_tick = if let Some(current_tick) = self.current_tick {
            Some(current_tick + 1)
        } else {
            Some(0)
        };
    }

    pub fn last_view_update_tick(&self) -> Option<usize> {
        self.last_view_update_tick
    }

    pub fn update_last_view_update(&mut self) {
        self.last_view_update_tick = self.current_tick;
    }
}

pub struct TickSystem {}

impl TickSystem {
    pub fn new() -> Self {
        TickSystem {}
    }
}

impl System for TickSystem {
    fn execute(&self, world: &World) {
        if let Some(mut tick_info) = world.get_resource_mut::<TickInfo>() {
            tick_info.increment_tick();
        }
    } 
}
