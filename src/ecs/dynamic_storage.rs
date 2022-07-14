use std::{cell::{Cell, UnsafeCell}, ops::{Deref, DerefMut}, any::{Any, TypeId}, fmt::Display, collections::{HashMap, hash_map::Entry}};

use super::ECSError;

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

#[derive(Default)]
pub struct DynamicStore {
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