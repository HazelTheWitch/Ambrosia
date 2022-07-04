use std::any::{Any, TypeId};
use std::cell:: UnsafeCell;
use std::collections::HashMap;
use std::ops::Add;

pub trait System {
    fn execute(&self, world: &World);
}

pub trait Component {
    fn allowed(&self, _entity: &Entity) -> bool {
        true
    }
}

pub trait AnyComponent: Component + Any {}

pub struct Entity {
    id: usize,
    components: HashMap<TypeId, UnsafeCell<Box<dyn AnyComponent>>>
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Entity { id, components: HashMap::new() }
    }

    pub fn insert<T: AnyComponent>(&mut self, component: T) -> Result<&mut Self, ECSError> {
        let id = component.type_id();

        if !self.components.contains_key(&id) {
            self.components.insert(id, UnsafeCell::new(Box::new(component)));
            
            Ok(self)
        } else {
            Err(ECSError::ComponentAlreadyExists)
        }
    }

    fn has_component_type_id(&self, type_id: &TypeId) -> bool {
        self.components.contains_key(type_id)
    }

    pub fn has_component<T: AnyComponent>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }

    fn get_component_cell<T: AnyComponent>(&self) -> Option<&UnsafeCell<Box<dyn AnyComponent>>> {
        self.components.get(&TypeId::of::<T>())
    }

    pub fn get_component<T: AnyComponent>(&self) -> Option<&T> {
        let cell = self.get_component_cell::<T>()?;

        unsafe {
            let the_box = &*cell.get();

            let the_any = *the_box;

            Some(&*the_any as &T)
            // let any = &mut *cell.get() as &mut dyn Any;

            // Some(&**any.downcast_ref::<Box<T>>()?)
        }
    }

    pub fn get_component_mut<T: AnyComponent>(&self) -> Option<&mut T> {
        let cell = self.get_component_cell::<T>()?;

        unsafe {
            // let any = &mut *cell.get() as &mut dyn Any;

            // Some(&mut **any.downcast_mut::<Box<T>>()?)
        }
    }

    pub unsafe fn get_component_unchecked<T: AnyComponent>(&self) -> &T {
        let cell = self.get_component_cell::<T>().unwrap();

        // let any = &mut *cell.get() as &mut dyn Any;

        // &**any.downcast_ref_unchecked::<Box<T>>()
    }

    pub unsafe fn get_component_mut_unchecked<T: AnyComponent>(&self) -> &mut T {
        let cell = self.get_component_cell::<T>().unwrap();

        // let any = &mut *cell.get() as &mut dyn Any;

        // &mut **any.downcast_mut_unchecked::<Box<T>>()
    }

    pub fn id(&self) -> usize {
        self.id
    }
}


#[derive(Clone)]
pub struct Query {
    includes: Vec<TypeId>,
    excludes: Vec<TypeId>
}

impl Query {
    pub fn new() -> Self {
        Query { includes: Vec::new(), excludes: Vec::new() }
    }

    pub fn include<T: AnyComponent>(mut self) -> Self {
        self.includes.push(TypeId::of::<T>());
        self
    }

    pub fn exclude<T: AnyComponent>(mut self) -> Self {
        self.excludes.push(TypeId::of::<T>());
        self
    }

    pub fn contains(&self, entity: &Entity) -> bool {
        for type_id in &self.includes {
            if !entity.has_component_type_id(type_id) {
                return false;
            }
        }

        for type_id in &self.excludes {
            if entity.has_component_type_id(type_id) {
                return false;
            }
        }

        true
    }

    pub fn join(&mut self, other: Query) -> &mut Self {
        for include in other.includes {
            self.includes.push(include);
        }

        for include in other.excludes {
            self.excludes.push(include);
        }

        self
    }
}

impl Add for Query {
    type Output = Query;

    fn add(self, rhs: Self) -> Self::Output {
        let mut clone = self.clone();
        clone.join(rhs);
        clone
    }
}


pub struct World {
    entities: Vec<Option<Entity>>,
    systems: Vec<Box<dyn System>>
}

impl World {
    pub fn new() -> Self {
        World { entities: Vec::new(), systems: Vec::new() }
    }

    pub fn spawn(&mut self) -> Result<&mut Entity, ECSError> {
        let entity = Entity::new(self.entities.len());
        let id = entity.id;
        self.entities.push(Some(entity));
        match self.entities.get_mut(id) {
            Some(entity) => {
                match entity {
                    Some(entity) => Ok(entity),
                    None => Err(ECSError::CouldNotSpawn)
                }
            },
            None => Err(ECSError::CouldNotSpawn)
        }
    }

    pub fn insert(&mut self, entity: Entity) {
        self.entities.push(Some(entity));
    }

    pub fn get(&self, id: usize) -> Option<&Entity> {
        if id < self.entities.len() {
            self.entities[id].as_ref()
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&Entity> {
        self.entities.iter().flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut Entity> {
        self.entities.iter_mut().flatten()
    }

    pub fn query_entities<'a>(&'a self, query: &'a Query) -> impl Iterator<Item = &'a Entity> {
        self.iter().filter(|entity| { query.contains(entity) })
    }

    pub fn query_one_entity<'a>(&'a self, query: &'a Query) -> Option<&Entity> {
        for e in &self.entities {
            if let Some(e) = e {
                if query.contains(e) {
                    return Some(e);
                }
            }
        }

        None
    }

    pub fn remove(&mut self, entity: Entity) {
        self.entities[entity.id] = None;
    }

    pub fn remove_id(&mut self, id: usize) {
        self.entities[id] = None;
    }

    pub fn add_system(&mut self, system: Box<dyn System>) -> &mut Self {
        self.systems.push(system);
        self
    }

    pub fn tick(&self) {
        for system in self.systems.iter() {
            system.execute(self)
        }
    }
}


pub enum ECSError {
    ComponentAlreadyExists,
    CouldNotSpawn
}

// TODO: Query Helper Methods
// TODO: Query Macro / `for (T1, T2, T3) in query!(world, T1, T2, T3)` syntax

// TODO: System abstraction
