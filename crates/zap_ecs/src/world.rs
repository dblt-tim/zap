use std::any::TypeId;
use std::collections::HashMap;

use crate::{
    component::{AnyStorage, Component, Storage},
    entity::{Entities, Entity, EntityBuilder},
};

pub struct World {
    entities: Entities,
    storages: HashMap<TypeId, Box<dyn AnyStorage>>,
}
impl World {
    pub fn new() -> Self {
        Self {
            entities: Entities::new(),
            storages: HashMap::new(),
        }
    }

    fn storage<T: Component>(&self) -> Option<&Storage<T>> {
        self.storages
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<Storage<T>>()
    }

    fn storage_mut<T: Component>(&mut self) -> &mut Storage<T> {
        self.storages
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(Storage::<T>::new()))
            .as_any_mut()
            .downcast_mut::<Storage<T>>()
            .expect("bad TypeId")
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entities.is_alive(entity)
    }

    pub fn attach_component<T: Component>(&mut self, entity: Entity, value: T) -> bool {
        if !self.is_alive(entity) {
            return false;
        }
        self.storage_mut::<T>().attach(entity, value)
    }

    pub fn detach_component<T: Component>(&mut self, entity: Entity) -> Option<T> {
        if !self.is_alive(entity) {
            return None;
        }
        self.storage_mut::<T>().detach(entity)
    }

    pub fn spawn(&mut self) -> EntityBuilder<'_> {
        let entity = self.entities.create_entity();
        EntityBuilder::new(self, entity)
    }

    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        self.storage::<T>()?.fetch(entity)
    }

    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        self.storage_mut::<T>().fetch_mut(entity)
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.entities.delete_entity(entity) {
            return false;
        }
        for storage in self.storages.values_mut() {
            storage.detach_entity(entity);
        }
        true
    }
}
