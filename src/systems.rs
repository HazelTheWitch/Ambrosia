use crate::components::*;
use crate::ecs::*;

pub struct DebugSystem {
    pub min_level: DebugLevel
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
                    None => format!("Entity({})", e.id())
                };

                if debug.max_level >= self.min_level {
                    if debug.count() > 0 {
                        println!("{}", name);

                        for message in debug.messages.values() {
                            println!("    {}", message);
                        }
                    }
                }
            }
        }
    }
}