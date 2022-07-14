use std::{collections::HashMap, ops::Index, cmp::Ordering};

use rand::{prelude::SliceRandom, thread_rng};

use crate::{vectors::Vector, ecs::{World, EntityId}, query, components::Position};

type Entities = Vec<EntityId>;
type Entry = (Vector, Entities);

pub struct Node {
    point: Vector,
    entities: Entities,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    depth: u16,
}

impl Node {
    pub fn get(&self, point: Vector) -> Option<&Entities> {
        if point == self.point {
            Some(&self.entities)
        } else {
            match self.get_side(point) {
                Ordering::Less => self.left.as_ref()?.get(point),
                Ordering::Equal | Ordering::Greater => self.right.as_ref()?.get(point)
            }
        }
    }

    pub fn iter(&self) -> KDNodeIterator {
        KDNodeIterator::new(self)
    }

    fn get_side(&self, point: Vector) -> Ordering {
        get_component(&point, self.depth).cmp(&get_component(&self.point, self.depth))
    }
}

pub struct KDNodeIterator<'n> {
    stack: Vec<&'n Node>
}

impl <'n> KDNodeIterator<'n> {
    fn new(root: &'n Node) -> Self {
        KDNodeIterator { stack: vec![root] }
    }
}

impl <'n> Iterator for KDNodeIterator<'n> {
    type Item = (Vector, &'n Entities);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        if let Some(left) = &node.left { self.stack.push(left) };
        if let Some(right) = &node.right { self.stack.push(right) };

        Some((node.point, &node.entities))
    }
}

pub fn kdtree(world: &World) -> Option<Box<Node>> {
    let mut entities: HashMap<Vector, Entities> = HashMap::new();

    for entity in world.query_entities(&query!(Position)) {
        if let Some(position) = entity.get_component::<Position>() {
            let entry = entities.entry(position.coords());

            if let Some(id) = entity.id() {
                entry.or_insert(Vec::new()).push(id.clone());
            }
        }
    }

    let data: Vec<Entry> = entities.into_iter().collect();

    _kdtree(data, 0)
}

fn _kdtree(data: Vec<Entry>, depth: u16) -> Option<Box<Node>> {
    let pivot = median(&data, depth)?;

    let pivot_value = get_component(&pivot.0, depth);

    Some(Box::new(Node {
        point: pivot.0,
        entities: pivot.1,
        left: _kdtree(data.clone().into_iter().filter(|(pos, _)| { get_component(pos, depth) < pivot_value }).collect(), depth + 1),
        right: _kdtree(data.clone().into_iter().filter(|(pos, _)| { get_component(pos, depth) >= pivot_value }).collect(), depth + 1),
        depth
    }))
}

fn median(data: &Vec<Entry>, depth: u16) -> Option<Entry> {
    let mut sample: Vec<Entry> = data.choose_multiple(&mut thread_rng(), 5).cloned().collect();

    sample.sort_by_cached_key(|(pos, _)| { get_component(pos, depth) });

    Some(sample.get(sample.len() / 2)?.clone())
}

#[inline]
fn get_component(v: &Vector, depth: u16) -> i32 {
    if depth & 1 == 0 { v.x } else { v.y }
}
