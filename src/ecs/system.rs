use super::world::World;

pub trait System {
    fn initialize(&self, _world: &World) {
        
    }
    
    fn execute(&self, world: &World);
}