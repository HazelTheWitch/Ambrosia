pub mod world;
pub mod query;
pub mod system;
pub mod dynamic_storage;
pub mod entity;
pub mod archetype;

#[derive(Debug)]
pub enum ECSError {
    DataAlreadyExists,
    CouldNotSpawn,
}
