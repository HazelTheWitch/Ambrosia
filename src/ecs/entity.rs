use std::any::{Any, TypeId};

use super::{archetype::Archetype, world::World, dynamic_storage::{DynamicStore, DynamicRef, DynamicRefMut}, ECSError};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct EntityId {
    index: usize,
    archetype: Archetype
}

impl EntityId {
    pub fn new(archetype: Archetype, index: usize) -> Self {
        EntityId { index, archetype }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn archetype(&self) -> &Archetype {
        &self.archetype
    }
}

pub struct EntityBuilder<'w> {
    world: &'w mut World,
    components: DynamicStore,
    archetype: Archetype,
}

impl <'w> EntityBuilder<'w> {
    pub fn build(self) -> &'w mut Entity {
        let entity = Entity { id: None, components: self.components, archetype: self.archetype };

        self.world.insert(entity)
    }

    pub fn insert_component<T: Any>(mut self, component: T) -> Result<Self, ECSError> {
        self.components.insert(component)?;
        self.archetype.add::<T>();
        Ok(self)
    }
}

pub struct Entity {
    id: Option<EntityId>,
    components: DynamicStore,
    archetype: Archetype
}

impl Entity {
    pub fn new(world: &mut World) -> EntityBuilder {
        EntityBuilder {
            world,
            components: Default::default(),
            archetype: Default::default()
        }
    }

    pub fn insert_component<T: Any>(&mut self, component: T) -> Result<&mut Self, ECSError> {
        self.components.insert(component)?;
        Ok(self)
    }

    pub fn has_component<T: Any>(&self) -> bool {
        self.components.has::<T>()
    }

    pub fn has_component_type_id(&self, type_id: &TypeId) -> bool {
        self.components.has_type_id(type_id)
    }

    pub fn get_component<T: Any>(&self) -> Option<DynamicRef<'_, T>> {
        self.components.get::<T>()
    }

    pub fn get_component_mut<T: Any>(&self) -> Option<DynamicRefMut<'_, T>> {
        self.components.get_mut::<T>()
    }

    pub fn set_id(&mut self, id: EntityId) {
        match self.id {
            Some(_) => panic!("Already inserted this entity."),
            None => { self.id = Some(id); }
        }
    }

    pub fn id(&self) -> Option<&EntityId> {
        self.id.as_ref()
    }

    pub fn archetype(&self) -> &Archetype {
        &self.archetype
    }
}