use std::any::{Any, TypeId};
use std::cell::{UnsafeCell, Cell};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Display;
use std::ops::{Add, Deref, DerefMut};

pub trait System {
    fn execute(&self, world: &World);
}

pub struct DynamicRef<'b, T> {
    value: &'b T,
    reference_state_cell: &'b Cell<ReferenceState>
}

impl <'b, T> DynamicRef<'b, T> {
    fn new(value: &'b T, reference_state_cell: &'b Cell<ReferenceState>) -> Option<Self> {
        reference_state_cell.set(reference_state_cell.get().increment()?);
        Some(DynamicRef { value, reference_state_cell })
    }
}

impl <'b, T> Drop for DynamicRef<'b, T> {
    fn drop(&mut self) {
        self.reference_state_cell.set(self.reference_state_cell.get().decrement().unwrap());
    }
}

impl <'b, T> Deref for DynamicRef<'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

pub struct DynamicRefMut<'b, T> {
    value: &'b mut T,
    reference_state_cell: &'b Cell<ReferenceState>
}

impl <'b, T> DynamicRefMut<'b, T> {
    fn new(value: &'b mut T, reference_state_cell: &'b Cell<ReferenceState>) -> Option<Self> {
        reference_state_cell.set(reference_state_cell.get().increment_mut()?);
        Some(DynamicRefMut { value, reference_state_cell })
    }
}

impl <'b, T> Drop for DynamicRefMut<'b, T> {
    fn drop(&mut self) {
        self.reference_state_cell.set(self.reference_state_cell.get().decrement_mut().unwrap());
    }
}

impl <'b, T> Deref for DynamicRefMut<'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl <'b, T> DerefMut for DynamicRefMut<'b, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

pub struct DynamicCell {
    data: UnsafeCell<Box<dyn Any>>,
    reference_state_cell: Cell<ReferenceState>
}

impl DynamicCell {
    pub fn new<T: Any>(data: T) -> Self {
        DynamicCell { data: UnsafeCell::new(Box::new(data)), reference_state_cell: Cell::new(ReferenceState::None) }
    }

    pub fn get<T: Any>(&self) -> Option<DynamicRef<'_, T>> {
        let value = unsafe {
            (**self.data.get()) // Convert data in UnsafeCell to `dyn Any`
                .downcast_ref::<T>()? // Downcast to a reference of type &T
        };

        DynamicRef::new(value, &self.reference_state_cell)
    }

    pub fn get_mut<T: Any>(&self) -> Option<DynamicRefMut<'_, T>> {
        let value = unsafe {
            (**self.data.get()) // Convert data in UnsafeCell to `dyn Any`
                .downcast_mut::<T>()? // Downcast to a reference of type &mut T
        };

        DynamicRefMut::new(value, &self.reference_state_cell)
    }

    pub unsafe fn get_unchecked<T: Any>(&self) -> &T {
        (**self.data.get()).downcast_ref_unchecked::<T>()
    }

    pub unsafe fn get_mut_unchecked<T: Any>(&self) -> &mut T {
        (**self.data.get()).downcast_mut_unchecked::<T>()
    }
}

#[derive(Clone, Copy)]
enum ReferenceState {
    Immutable(usize),
    Mutable,
    None
}

impl ReferenceState {
    fn increment(&self) -> Option<Self> {
        match self {
            ReferenceState::Immutable(count) => {
                let count = count.wrapping_add(1);
                if count > 0{
                    Some(ReferenceState::Immutable(count))
                } else {
                    None
                }
            },
            ReferenceState::Mutable => panic!("attempted to increment a mutable reference"),
            ReferenceState::None => Some(ReferenceState::Immutable(1)),
        }
    }

    fn increment_mut(&self) -> Option<Self> {
        match self {
            ReferenceState::Immutable(_) => panic!("attempted to increment_mut an immutable reference"),
            ReferenceState::Mutable => panic!("attempted to increment_mut a mutable reference"),
            ReferenceState::None => Some(ReferenceState::Mutable),
        }
    }

    fn decrement(&self) -> Option<Self> {
        match self {
            ReferenceState::Immutable(count) => {
                let count = count - 1;
                if count > 0 {
                    Some(ReferenceState::Immutable(count))
                } else {
                    Some(ReferenceState::None)
                }
            },
            ReferenceState::Mutable => panic!("attempted to decrement a mutable reference"),
            ReferenceState::None => panic!("attempted to decrement a value not currently referenced"),
        }
    }

    fn decrement_mut(&self) -> Option<Self> {
        match self {
            ReferenceState::Immutable(_) => panic!("attempted to decrement_mut an immutable reference"),
            ReferenceState::Mutable => Some(ReferenceState::None),
            ReferenceState::None => panic!("attempted to decrement a value not currently referenced"),
        }
    }
}

impl Display for ReferenceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReferenceState::Immutable(borrow_count) => write!(f, "Immutable - {}", borrow_count),
            ReferenceState::Mutable => write!(f, "Mutable"),
            ReferenceState::None => write!(f, "Not Borrowed"),
        }
    }
}

struct DynamicStore {
    data: HashMap<TypeId, DynamicCell>,
}

impl DynamicStore {
    pub fn new() -> Self {
        DynamicStore {
            data: HashMap::new(),
        }
    }

    pub fn insert<T: Any>(&mut self, data: T) -> Result<&mut Self, ECSError> {
        let id = data.type_id();

        if let Entry::Vacant(e) = self.data.entry(id) {
            e.insert(DynamicCell::new(data));

            Ok(self)
        } else {
            Err(ECSError::DataAlreadyExists)
        }
    }

    pub fn has_type_id(&self, type_id: &TypeId) -> bool {
        self.data.contains_key(type_id)
    }

    pub fn has<T: Any>(&self) -> bool {
        self.data.contains_key(&TypeId::of::<T>())
    }

    #[inline]
    fn get_cell<T: Any>(&self) -> Option<&DynamicCell> {
        self.data.get(&TypeId::of::<T>())
    }

    pub fn get<T: Any>(&self) -> Option<DynamicRef<'_, T>> {
        self.get_cell::<T>()?.get::<T>()
    }

    pub fn get_mut<T: Any>(&self) -> Option<DynamicRefMut<'_, T>> {
        self.get_cell::<T>()?.get_mut::<T>()
    }
}

impl Default for DynamicStore {
    fn default() -> Self {
        DynamicStore::new()
    }
}

pub struct Entity {
    id: usize,
    components: DynamicStore,
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Entity {
            id,
            components: Default::default(),
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

    pub fn id(&self) -> usize {
        self.id
    }
}

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
        let mut clone = self;
        clone.join(rhs);
        clone
    }
}

pub struct World {
    entities: Vec<Option<Entity>>,
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

    pub fn spawn(&mut self) -> Result<&mut Entity, ECSError> {
        let entity = Entity::new(self.entities.len());
        let id = entity.id;
        self.entities.push(Some(entity));

        if let Some(Some(entity)) = self.entities.get_mut(id) {
            Ok(entity)
        } else {
            Err(ECSError::CouldNotSpawn)
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

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter().flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.iter_mut().flatten()
    }

    pub fn query_entities<'a>(&'a self, query: &'a Query) -> impl Iterator<Item = &'a Entity> {
        self.iter().filter(|entity| query.contains(entity))
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

    pub fn add_system(&mut self, system: Box<dyn System>, priority: i32) -> &mut Self {
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

#[derive(Debug)]
pub enum ECSError {
    DataAlreadyExists,
    CouldNotSpawn,
}
