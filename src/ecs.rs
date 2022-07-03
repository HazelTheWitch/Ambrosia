use std::any::{Any, TypeId};
use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::ops::Add;

pub struct Entity {
    id: usize,
    components: HashMap<TypeId, Box<dyn Any>>
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Entity { id, components: HashMap::new() }
    }

    pub fn insert<T: Any>(&mut self, component: T) -> &mut Self {
        let id = component.type_id();

        if !self.components.contains_key(&id) {
            self.components.insert(id, Box::new(component));
        }

        self
    }

    fn has_component_type_id(&self, type_id: &TypeId) -> bool {
        self.components.contains_key(type_id)
    }

    pub fn has_component<T: Any>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }

    pub fn get_component<T: Any>(&self) -> Option<&T> {
        self.components.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }

    pub fn get_component_mut<T: Any>(&self) -> Option<&mut T> {
        self.components.get_mut(&TypeId::of::<T>())?.downcast_mut::<T>()
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

    pub fn include<T: Any>(mut self) -> Self {
        self.includes.push(TypeId::of::<T>());
        self
    }

    pub fn exclude<T: Any>(mut self) -> Self {
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
    entities: Vec<Option<Entity>>
}

impl World {
    pub fn new() -> Self {
        World { entities: Vec::new() }
    }

    pub fn spawn(&mut self) -> &mut Option<Entity> {
        let entity = Entity::new(self.entities.len());
        let id = entity.id;
        self.entities.push(Some(entity));
        unsafe {
            self.entities.get_unchecked_mut(id)
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

    pub fn query<'a>(&'a self, query: &'a Query) -> impl Iterator<Item = &'a Entity> {
        self.iter().filter(|entity| { query.contains(entity) })
    }

    pub fn query_mut<'a>(&'a mut self, query: &'a Query) -> impl Iterator<Item = &'a mut Entity> {
        self.iter_mut().filter(|entity| { query.contains(entity) })
    }

    pub fn remove(&mut self, entity: Entity) {
        self.entities[entity.id] = None;
    }

    pub fn remove_id(&mut self, id: usize) {
        self.entities[id] = None;
    }
}

// TODO: Query Iterators
// TODO: Query Helper Methods
// TODO: Query Macro / `for (T1, T2, T3) in query!(world, T1, T2, T3)` syntax
