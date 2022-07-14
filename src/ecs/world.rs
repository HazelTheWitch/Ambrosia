use std::{collections::HashMap, any::{TypeId, Any}};

use super::{archetype::Archetype, entity::{Entity, EntityBuilder, EntityId}, system::System, dynamic_storage::{DynamicStore, DynamicRef, DynamicRefMut}, query::Query, ECSError};

// TODO: implement entity id reuse
pub struct World {
    entities: HashMap<Archetype, Vec<Option<Entity>>>,
    systems: Vec<(Box<dyn System>, i32)>,
    resources: DynamicStore,
}

impl World {
    pub fn new() -> Self {
        World {
            entities: Default::default(),
            systems: Default::default(),
            resources: Default::default(),
        }
    }

    pub fn spawn(&mut self) -> EntityBuilder {
        Entity::new(self)
    }
    
    fn get_next_id(&mut self, archetype: Archetype) -> EntityId {
        let entry = self.entities.entry(archetype.clone());
        EntityId::new(archetype, entry.or_default().len())
    }

    pub fn insert(&mut self, mut entity: Entity) -> Result<&mut Entity, ECSError> {
        let id = self.get_next_id(entity.archetype().clone());

        let entry = self.entities.entry(entity.archetype().clone());

        entity.try_set_id(id.clone())?;

        let entities = entry.or_default();

        if id.index() < entities.len() { // Can insert entity
            entities.insert(id.index(), Some(entity));
        } else {
            return Err(ECSError::InvalidInsertionIndex(id.index()))
        }

        match entities.get_mut(id.index()) {
            Some(entity) => match entity.as_mut() {
                Some(entity) => Ok(entity),
                None => Err(ECSError::CouldNotRetrieve),
            },
            None => Err(ECSError::CouldNotRetrieve),
        }
    }

    pub fn get(&self, id: &EntityId) -> Option<&Entity> {
        self.entities.get(id.archetype())?.get(id.index())?.as_ref()
    }

    pub fn get_mut(&mut self, id: &EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(id.archetype())?.get_mut(id.index())?.as_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values().flat_map(|entity_vec| { entity_vec.iter() }).flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.values_mut().flat_map(|entity_vec| { entity_vec.iter_mut() }).flatten()
    }

    pub fn query_entities<'a>(&'a self, query: &'a Query) -> impl Iterator<Item = &'a Entity> {
        self.entities.iter().filter_map(|(archetype, entities)| {
            if query.matches(archetype) {
                Some(entities)
            } else {
                None
            }
         }).flat_map(|entities| { entities.iter() }).flatten()
    }

    pub fn query_one_entity<'a>(&'a self, query: &'a Query) -> Option<&Entity> {
        self.query_entities(query).next()
    }

    pub fn remove(&mut self, entity: Entity) -> Option<Entity> {
        let id = entity.id().expect("an inserted entity");

        let list = self.entities.get_mut(id.archetype()).expect("a valid archetype");

        list.remove(id.index())
    }

    pub fn remove_id(&mut self, id: EntityId) -> Option<Entity> {
        let list = self.entities.get_mut(id.archetype()).expect("a valid archetype");

        list.remove(id.index())
    }

    pub fn add_system(&mut self, system: Box<dyn System>, priority: i32) -> &mut Self {
        system.initialize(self);
        
        let mut index = 0;

        while let Some((_, other_priority)) = self.systems.get(index) {
            if other_priority <= &priority {
                break;
            }

            index += 1;
        }

        self.systems.insert(index, (system, priority));

        self
    }

    pub fn tick(&self) {
        for (system, _) in self.systems.iter() {
            system.execute(self)
        }
    }

    pub fn insert_resource<T: Any>(&mut self, resource: T) -> Result<&mut Self, ECSError> {
        self.resources.insert(resource)?;
        Ok(self)
    }

    pub fn has_resource<T: Any>(&self) -> bool {
        self.resources.has::<T>()
    }

    pub fn has_resource_type_id(&self, type_id: &TypeId) -> bool {
        self.resources.has_type_id(type_id)
    }

    pub fn get_resource<T: Any>(&self) -> Option<DynamicRef<'_, T>> {
        self.resources.get::<T>()
    }

    pub fn get_resource_mut<T: Any>(&self) -> Option<DynamicRefMut<'_, T>> {
        self.resources.get_mut::<T>()
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}