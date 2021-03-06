use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use crate::component::storage::{ErasedStorageHolder, StorageHolder};
use crate::component::view_one::{ViewOne, ViewOneMut};
use crate::component::{Component, ComponentSet, ComponentTypeId, Entry, Storage, StorageImpl};
use crate::entity::{Entity, Iter, Registry as EntityRegistry};
use crate::world::TypeIdHasher;

#[derive(Default)]
pub struct Registry {
    entities: EntityRegistry,
    extended_entities: Vec<Entity>,
    storages: StorageMap,
}

impl Registry {
    pub fn create(&mut self) -> Entity {
        self.entities.create()
    }

    pub fn create_with_one<C>(&mut self, component: C) -> Entity
    where
        C: Component,
    {
        let entity = self.create();
        self.attach_one(entity, component);
        entity
    }

    pub fn create_with<S>(&mut self, set: S) -> Entity
    where
        S: ComponentSet,
    {
        let entity = self.create();
        self.attach(entity, set);
        entity
    }

    pub fn create_entry(&mut self) -> Entry {
        let entity = self.create();
        Entry::new(entity, self)
    }

    pub fn create_entry_with_one<C>(&mut self, component: C) -> Entry
    where
        C: Component,
    {
        let entity = self.create_with_one(component);
        Entry::new(entity, self)
    }

    pub fn create_entry_with<S>(&mut self, set: S) -> Entry
    where
        S: ComponentSet,
    {
        let entity = self.create_with(set);
        Entry::new(entity, self)
    }

    pub fn entry(&mut self, entity: Entity) -> Option<Entry> {
        self.contains(entity).then(|| Entry::new(entity, self))
    }

    pub fn extend(&mut self, count: u32) -> &[Entity] {
        self.extended_entities.clear();
        (0..count).for_each(|_| {
            let entity = self.create();
            self.extended_entities.push(entity);
        });
        self.extended_entities.as_slice()
    }

    pub fn extend_with_one<I, C>(&mut self, into_iter: I) -> &[Entity]
    where
        I: IntoIterator<Item = C>,
        C: Component,
    {
        self.extended_entities.clear();
        let iter = into_iter.into_iter();
        iter.for_each(|component| {
            let entity = self.create_with_one(component);
            self.extended_entities.push(entity);
        });
        self.extended_entities.as_slice()
    }

    pub fn extend_with<I, S>(&mut self, into_iter: I) -> &[Entity]
    where
        I: IntoIterator<Item = S>,
        S: ComponentSet,
    {
        self.extended_entities.clear();
        let iter = into_iter.into_iter();
        iter.for_each(|set| {
            let entity = self.create_with(set);
            self.extended_entities.push(entity);
        });
        self.extended_entities.as_slice()
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    pub fn destroy(&mut self, entity: Entity) {
        self.remove_all(entity);
        self.entities.destroy(entity);
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.extended_entities.clear();
        self.storages.clear();
    }

    pub fn register<C>(&mut self)
    where
        C: Component,
    {
        if !self.has_storage::<C>() {
            self.create_storage::<C>();
        }
    }

    pub fn attach_one<C>(&mut self, entity: Entity, component: C)
    where
        C: Component,
    {
        self.register::<C>();
        let storage = self.get_storage_mut().unwrap();
        storage.attach(entity, component);
    }

    pub fn attach<S>(&mut self, entity: Entity, set: S)
    where
        S: ComponentSet,
    {
        set.attach(self, entity)
    }

    pub fn attached_one<C>(&self, entity: Entity) -> bool
    where
        C: Component,
    {
        let storage = self.get_storage::<C>();
        storage
            .map(|storage| storage.attached(entity))
            .unwrap_or(false)
    }

    pub fn attached<S>(&self, entity: Entity) -> bool
    where
        S: ComponentSet,
    {
        S::attached(self, entity)
    }

    pub fn is_entity_empty(&self, entity: Entity) -> bool {
        self.storages.is_entity_empty(entity)
    }

    pub fn remove_one<C>(&mut self, entity: Entity)
    where
        C: Component,
    {
        let storage = self.get_storage_mut::<C>();
        if let Some(storage) = storage {
            storage.remove(entity)
        }
    }

    pub fn remove<S>(&mut self, entity: Entity)
    where
        S: ComponentSet,
    {
        S::remove(self, entity)
    }

    pub fn remove_all(&mut self, entity: Entity) {
        self.storages.remove_all(entity)
    }

    pub fn get<C>(&self, entity: Entity) -> Option<&C>
    where
        C: Component,
    {
        self.storages.get(entity)
    }

    pub fn get_mut<C>(&mut self, entity: Entity) -> Option<&mut C>
    where
        C: Component,
    {
        self.storages.get_mut(entity)
    }

    pub fn view_one<C>(&self) -> ViewOne<C>
    where
        C: Component,
    {
        ViewOne::new(self)
    }

    pub fn view_one_mut<C>(&mut self) -> ViewOneMut<C>
    where
        C: Component,
    {
        ViewOneMut::new(self)
    }

    pub(super) fn get_storage<C>(&self) -> Option<&StorageImpl<C>>
    where
        C: Component,
    {
        self.storages.get_storage()
    }

    pub(super) fn get_storage_mut<C>(&mut self) -> Option<&mut StorageImpl<C>>
    where
        C: Component,
    {
        self.storages.get_storage_mut()
    }

    pub fn get_storage_holder<C>(&mut self) -> Option<StorageHolder<C>>
    where
        C: Component,
    {
        self.storages.get_storage_holder()
    }

    pub fn put_storage_holder<C>(&mut self, storage_holder: StorageHolder<C>)
    where
        C: Component,
    {
        self.storages.put_storage_holder(storage_holder)
    }

    pub fn has_storage<C>(&self) -> bool
    where
        C: Component,
    {
        self.storages.has_storage::<C>()
    }

    fn create_storage<C>(&mut self)
    where
        C: Component,
    {
        self.storages.create_storage::<C>()
    }

    pub fn entities(&self) -> Iter {
        self.entities.iter()
    }

    pub fn split(&self) -> (&EntityRegistry, &StorageMap) {
        (&self.entities, &self.storages)
    }

    pub fn split_mut(&mut self) -> (&EntityRegistry, &mut StorageMap) {
        (&self.entities, &mut self.storages)
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct StorageMap {
    storages: HashMap<ComponentTypeId, ErasedStorageHolder, BuildHasherDefault<TypeIdHasher>>,
}

impl StorageMap {
    pub fn has_storage<C>(&self) -> bool
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        self.storages.contains_key(&type_id)
    }

    pub fn is_entity_empty(&self, entity: Entity) -> bool {
        self.storages
            .values()
            .all(|storage| !storage.attached(entity))
    }

    pub fn get<C>(&self, entity: Entity) -> Option<&C>
    where
        C: Component,
    {
        let storage = self.get_storage::<C>()?;
        storage.get(entity)
    }

    pub fn get_mut<C>(&mut self, entity: Entity) -> Option<&mut C>
    where
        C: Component,
    {
        let storage = self.get_storage_mut::<C>()?;
        storage.get_mut(entity)
    }

    fn create_storage<C>(&mut self)
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = StorageImpl::<C>::default();
        self.storages.insert(type_id, storage.into());
    }

    fn remove_all(&mut self, entity: Entity) {
        self.storages
            .values_mut()
            .for_each(|storage| storage.remove(entity))
    }

    fn clear(&mut self) {
        self.storages
            .values_mut()
            .for_each(|storage| storage.clear());
    }

    pub fn get_storage<C>(&self) -> Option<&StorageImpl<C>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.storages.get(&type_id)?;
        let storage = storage.downcast_ref().expect("downcast error");
        Some(storage)
    }

    pub fn get_storage_mut<C>(&mut self) -> Option<&mut StorageImpl<C>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.storages.get_mut(&type_id)?;
        let storage = storage.downcast_mut().expect("downcast error");
        Some(storage)
    }

    fn get_storage_holder<C>(&mut self) -> Option<StorageHolder<C>>
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        let storage = self.storages.remove(&type_id)?;
        Some(storage.into())
    }

    fn put_storage_holder<C>(&mut self, storage_holder: StorageHolder<C>)
    where
        C: Component,
    {
        let type_id = ComponentTypeId::of::<C>();
        self.storages.insert(type_id, storage_holder.into());
    }
}
