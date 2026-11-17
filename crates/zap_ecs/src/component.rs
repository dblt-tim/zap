use crate::entity::Entity;
use std::any::Any;

pub trait Component: 'static + Send + Sync {}
impl<T: 'static + Send + Sync> Component for T {}

pub struct Storage<T: Component> {
    dense: Vec<T>,
    owners: Vec<Entity>,
    sparse: Vec<Option<usize>>,
}
impl<T: Component> Storage<T> {
    pub fn new() -> Self {
        Self {
            dense: vec![],
            owners: vec![],
            sparse: vec![],
        }
    }

    fn pos(&self, entity: Entity) -> Option<usize> {
        // get dense and owners index from entity
        let pos = self.sparse.get(entity.index()).copied().flatten()?;
        (self.owners[pos] == entity).then_some(pos)
    }

    pub fn attach(&mut self, entity: Entity, value: T) -> bool {
        if self.pos(entity).is_some() {
            // if the entity already has a value reject
            return false;
        }
        let index = entity.index();
        if index >= self.sparse.len() {
            self.sparse.resize(index + 1, None);
        }
        self.dense.push(value);
        self.owners.push(entity);
        self.sparse[index] = Some(self.dense.len() - 1);

        true
    }

    pub fn fetch(&self, entity: Entity) -> Option<&T> {
        self.pos(entity).map(|p| &self.dense[p])
    }
    pub fn fetch_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let p = self.pos(entity)?;
        Some(&mut self.dense[p])
    }

    pub fn detach(&mut self, entity: Entity) -> Option<T> {
        let p = self.pos(entity)?;

        self.sparse[entity.index()] = None;
        let val = self.dense.swap_remove(p);
        self.owners.swap_remove(p);

        // last element moved to 'p' -> repair sparse
        if p < self.dense.len() {
            self.sparse[self.owners[p].index()] = Some(p);
        }
        // else 'p' was last element and nothing moved

        Some(val)
    }
}

pub trait AnyStorage: Any + Send + Sync {
    fn detach_entity(&mut self, entity: Entity);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Component> AnyStorage for Storage<T> {
    fn detach_entity(&mut self, entity: Entity) {
        self.detach(entity); // the returned value is dropped
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entities;

    #[test]
    fn attach_and_fetch_first_entity() {
        let mut es = Entities::new();
        let mut s = Storage::<i32>::new();
        let e = es.create_entity(); // index 0 triggered the panic before
        assert!(s.attach(e, 5));
        assert_eq!(s.fetch(e), Some(&5));
        assert!(!s.attach(e, 6)); // already has one
    }

    #[test]
    fn stale_handle_is_rejected() {
        let mut es = Entities::new();
        let mut s = Storage::<i32>::new();
        let old = es.create_entity();
        s.attach(old, 1);
        s.detach(old);
        es.delete_entity(old);
        let new = es.create_entity(); // same index, new generation
        s.attach(new, 2);
        assert_eq!(s.fetch(old), None);
        assert_eq!(s.fetch(new), Some(&2));
    }

    #[test]
    fn detach_keeps_invariant() {
        let mut es = Entities::new();
        let mut s = Storage::<i32>::new();
        let a = es.create_entity();
        let b = es.create_entity();
        let c = es.create_entity();
        s.attach(a, 10);
        s.attach(b, 20);
        s.attach(c, 30);

        assert_eq!(s.detach(a), Some(10)); // c moves into a's slot
        assert_eq!(s.fetch(a), None);
        assert_eq!(s.fetch(b), Some(&20));
        assert_eq!(s.fetch(c), Some(&30));

        for (i, owner) in s.owners.iter().enumerate() {
            assert_eq!(s.sparse[owner.index()], Some(i));
        }
    }
}
