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
            if let Some(debug) = e.get_component::<Debug>() {
                let name: String = match e.get_component::<Named>() {
                    Some(named) => named.name.to_string(),
                    None => format!("Entity({})", e.id()),
                };

                if debug.max_level >= self.min_level && debug.count() > 0 {
                    println!("{}", name);

                    for message in debug.messages.values() {
                        println!("    {}", message);
                    }
                }
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

        if let Some(map) = world.get_resource_mut::<Map>() {
            for entity in world.query_entities(&query) {
                if let (Some(viewshed), Some(position)) = (
                    entity.get_component_mut::<Viewshed>(),
                    entity.get_component::<Position>(),
                ) {
                    viewshed.update(map, &position.coords(), entity.has_component::<Player>());
                }
            }
        }
    }
}
