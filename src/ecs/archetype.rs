use std::{any::{TypeId, type_name, Any}, fmt::Display};

#[derive(Hash, PartialEq, Eq, Default, Clone, Debug)]
struct SortedVec<T: Ord> {
    data: Vec<T>
}

impl <T: Ord> SortedVec<T> {
    pub fn new() -> Self {
        SortedVec { data: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        let mut index = 0;

        while let Some(other) = self.data.get(index) {
            if *other == item {
                return;
            }

            if *other > item {
                break;
            }

            index += 1;
        }

        self.data.insert(index, item);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn contains(&self, other: &SortedVec<T>) -> bool {
        let mut index: usize = 0;

        'theirs: for type_id in other.data.iter() {
            while let Some(mine) = self.data.get(index) {
                index += 1;

                if type_id == mine {
                    continue 'theirs;
                }
            }

            return false;
        }

        true
    }

    pub fn has(&self, item: &T) -> bool {
        for other in self.data.iter() {
            if item == other {
                return true;
            }

            if item < other {
                return false;
            }
        }

        false
    }
}

impl <T: Ord, F: IntoIterator<Item = T>> From<F> for SortedVec<T> {
    fn from(iterator: F) -> Self {
        let mut sorted = SortedVec::new();

        for item in iterator {
            sorted.push(item);
        }

        sorted
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Archetype {
    types: SortedVec<TypeId>,
    names: Vec<String>,
}

impl Archetype {
    pub fn new() -> Self {
        Archetype { types: SortedVec::new(), names: Vec::new() }
    }

    pub fn add_type_id(&mut self, type_id: TypeId, name: String) -> &mut Self {
        self.types.push(type_id);
        self.names.push(name);

        self
    }

    pub fn add<T: Any>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<T>();
        let name = type_name::<T>().to_owned();

        self.add_type_id(type_id, name)
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }

    pub fn contains(&self, other: &Archetype) -> bool {
        self.types.contains(&other.types)
    }

    pub fn has_type_id(&self, item: &TypeId) -> bool {
        self.types.has(item)
    }

    pub fn has<T: Any>(&self) -> bool {
        self.has_type_id(&TypeId::of::<T>())
    }
}

impl Display for Archetype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.names.join(", "))
    }
}

impl Default for Archetype {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! archetype {
    ($t: ty) => {
        $crate::ecs::archetype::Archetype::new().add::<$t>()
    };

    ($t: ty, $($ts: ty),+) => {
        archetype!($($ts),+).add::<$t>()
    }
}

#[cfg(test)]
mod tests {
    use super::SortedVec;

    #[test]
    fn test_sorted_vec_push() {
        let mut sorted = SortedVec::new();

        sorted.push(1);
        sorted.push(2);
        sorted.push(4);
        sorted.push(3);
        sorted.push(1);

        assert_eq!(sorted.len(), 4);
    }

    #[test]
    fn test_sorted_vec_has() {
        let mut sorted = SortedVec::new();

        sorted.push(1);
        sorted.push(2);
        sorted.push(3);
        sorted.push(4);
        sorted.push(5);

        assert!(sorted.has(&1));
        assert!(sorted.has(&2));
        assert!(sorted.has(&3));
        assert!(sorted.has(&4));
        assert!(sorted.has(&5));

        assert!(!sorted.has(&0));
        assert!(!sorted.has(&-10));
        assert!(!sorted.has(&1000));
    }

    #[test]
    fn test_sorted_vec_contains() {
        let s0: SortedVec<i32> = vec![1, 2, 3, 4].into();
        let s1: SortedVec<i32> = vec![2, 3].into();
        let s2: SortedVec<i32> = vec![2, 3, 4].into();
        let s3: SortedVec<i32> = vec![2, 3, 4, 5].into();
        let s4: SortedVec<i32> = vec![0, 2, 3, 4].into();

        assert!(s0.contains(&s1));
        assert!(s0.contains(&s2));
        assert!(!s0.contains(&s3));
        assert!(!s0.contains(&s4));
        assert!(s0.contains(&s1));
        assert!(s3.contains(&s1));
        assert!(s3.contains(&s2));
    }
}