use std::{any::{Any, TypeId}, ops::Add};

use super::{entity::Entity, archetype::Archetype};

#[derive(Clone)]
pub struct Query {
    includes: Vec<TypeId>,
    excludes: Vec<TypeId>,
}

impl Query {
    pub fn new() -> Self {
        Query {
            includes: Vec::new(),
            excludes: Vec::new(),
        }
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

    pub fn matches(&self, archetype: &Archetype) -> bool {
        self.includes.iter().all(|ty| { archetype.has_type_id(ty) }) && !self.excludes.iter().any(|ty| { archetype.has_type_id(ty) })
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

impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for Query {
    type Output = Query;

    fn add(self, rhs: Self) -> Self::Output {
        let mut clone = self;
        clone.join(rhs);
        clone
    }
}