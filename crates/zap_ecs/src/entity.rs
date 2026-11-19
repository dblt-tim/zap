// the base entity system

use crate::{component::Component, world::World};

// each entity holds an index for components
// recycled handles are distinguishable using generation variable
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Entity {
    index: usize,
    generation: u32,
}
impl Entity {
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn generation(&self) -> u32 {
        self.generation
    }
}

// englobes entity management
#[derive(Default)]
pub struct Entities {
    generations: Vec<u32>,
    free: Vec<usize>,
}
impl Entities {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn create_entity(&mut self) -> Entity {
        if let Some(index) = self.free.pop() {
            return Entity {
                index,
                generation: self.generations[index],
            };
        } else {
            self.generations.push(0);

            return Entity {
                index: self.generations.len() - 1,
                generation: 0,
            };
        }
    }
    pub fn delete_entity(&mut self, entity: Entity) -> bool {
        if !self.is_alive(entity) {
            return false;
        }

        self.free.push(entity.index);
        self.generations[entity.index()] = self.generations[entity.index()].wrapping_add(1);

        true
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.generations.get(entity.index()) == Some(&entity.generation())
    }
}

pub struct EntityBuilder<'w> {
    world: &'w mut World,
    entity: Entity,
}
impl<'w> EntityBuilder<'w> {
    pub(crate) fn new(world: &'w mut World, entity: Entity) -> Self {
        Self { world, entity }
    }
    pub fn with<T: Component>(self, value: T) -> Self {
        self.world.attach_component::<T>(self.entity, value);
        self
    }
    pub fn done(self) -> Entity {
        self.entity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_creation() {
        let mut es = Entities::new();

        let e1 = es.create_entity();
        let e2 = es.create_entity();

        assert_eq!(e1.index(), 0);
        assert_eq!(e2.index(), 1);
    }

    #[test]
    fn entity_deletion() {
        let mut es = Entities::new();

        let e1 = es.create_entity();

        es.delete_entity(e1);

        assert!(!es.is_alive(e1));
    }

    #[test]
    fn generation_increments_on_delete() {
        let mut es = Entities::new();

        let mut e = es.create_entity();
        assert_eq!(e.generation(), 0);

        es.delete_entity(e); // kill e
        e = es.create_entity(); // reuse index 0
        assert_eq!(e.generation(), 1); // generation should have gone up 1
    }

    #[test]
    fn entity_index_reuse() {
        let mut es = Entities::new();

        let e1 = es.create_entity();

        es.delete_entity(e1);

        let e2 = es.create_entity(); // reuse index 0

        assert_eq!(e1.index(), e2.index());
        assert!(!es.is_alive(e1)); // e1 should be dead
        assert!(es.is_alive(e2)); // and e2 alive
    }

    #[test]
    fn safe_from_double_delete() {
        let mut es = Entities::new();

        let e1 = es.create_entity();

        assert!(es.delete_entity(e1)); // deleted e1 -> true
        assert!(!es.delete_entity(e1)); // e1 is already dead -> false
    }

    #[test]
    fn new_entity_safe_from_double_delete() {
        let mut es = Entities::new();

        let e1 = es.create_entity();

        es.delete_entity(e1);

        let _ = es.create_entity(); // reuse index 0

        assert!(!es.delete_entity(e1)); // e1 is already dead -> false
    }
}
