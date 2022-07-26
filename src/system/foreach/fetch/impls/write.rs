use std::collections::HashSet;

use atomicell::RefMut;

use crate::component::storage::{DynIterMut, Storage};
use crate::component::{Component, ComponentTypeId, Registry};
use crate::entity::Entity;
use crate::error::{FetchError, FetchResult};
#[cfg(feature = "resource")]
use crate::resource::{marker, Resource};
use crate::system::foreach::fetch::{Fetch, FetchData, FetchStrategy};
use crate::world::WorldData;

pub enum FetchWrite<'data, C>
where
    C: Component,
{
    Storage(RefMut<'data, C::Storage>),
    Iter(Box<DynIterMut<'data, C>>),
}

impl<'data, C> Fetch<'data> for FetchWrite<'data, C>
where
    C: Component,
{
    type Item = &'data mut C;

    // noinspection DuplicatedCode
    fn push_fetch_data(data: WorldData<'data>, fetch_data: &mut HashSet<FetchData>) {
        let type_id = ComponentTypeId::of::<C>();
        let storage = data.components().get_storage::<C>();
        if let Some(storage) = storage {
            let len = storage.iter().len();
            let data = FetchData::new(type_id, len);
            fetch_data.insert(data);
        }
    }

    fn register(registry: &mut Registry) {
        registry.register::<C>();
    }

    fn new(data: WorldData<'data>, optimal: Option<ComponentTypeId>) -> FetchResult<Self> {
        let storage = data
            .components()
            .get_storage_mut_guarded::<C>()
            .ok_or(FetchError)?;
        if optimal == Some(ComponentTypeId::of::<C>()) {
            let storage = RefMut::leak(storage);
            let iter = storage.iter_mut();
            Ok(Self::Iter(iter))
        } else {
            Ok(Self::Storage(storage))
        }
    }

    fn is_iter(&self) -> bool {
        matches!(self, Self::Iter(_))
    }

    fn fetch_entity(&'data mut self, entity: Entity) -> FetchResult<Self::Item> {
        match self {
            Self::Iter(_) => Err(FetchError),
            Self::Storage(storage) => {
                let item = storage.get_mut(entity).ok_or(FetchError)?;
                Ok(item)
            }
        }
    }

    // noinspection DuplicatedCode
    fn fetch_iter(
        &'data mut self,
        strategy: FetchStrategy<'data>,
    ) -> FetchResult<Option<(Entity, Self::Item)>> {
        match strategy {
            FetchStrategy::All(entities) => match self {
                Self::Storage(_) => {
                    let entity = match entities.next() {
                        None => return Ok(None),
                        Some(entity) => entity,
                    };
                    let item = self.fetch_entity(entity)?;
                    Ok(Some((entity, item)))
                }
                Self::Iter(_) => Err(FetchError),
            },
            FetchStrategy::Optimized => match self {
                Self::Storage(_) => Err(FetchError),
                Self::Iter(iter) => Ok(iter.next()),
            },
        }
    }
}

cfg_resource! {
    #[repr(transparent)]
    pub struct FetchResourceWrite<'data, R>
    where
        R: Resource,
    {
        resource: RefMut<'data, R>,
    }

    impl<'data, R> Fetch<'data> for FetchResourceWrite<'data, R>
    where
        R: Resource,
    {
        type Item = marker::ResourceMut<'data, R>;

        fn push_fetch_data(_: WorldData<'data>, _: &mut HashSet<FetchData>) {}

        fn register(_: &mut Registry) {}

        fn new(data: WorldData<'data>, _: Option<ComponentTypeId>) -> FetchResult<Self> {
            let resource = data.resources().get_mut_guarded().ok_or(FetchError)?;
            Ok(Self { resource })
        }

        fn is_iter(&self) -> bool {
            false
        }

        fn fetch_entity(&'data mut self, _: Entity) -> FetchResult<Self::Item> {
            let resource = marker::ResourceMut::new(&mut *self.resource);
            Ok(resource)
        }

        fn fetch_iter(
            &'data mut self,
            strategy: FetchStrategy<'data>,
        ) -> FetchResult<Option<(Entity, Self::Item)>> {
            match strategy {
                FetchStrategy::Optimized => Err(FetchError),
                FetchStrategy::All(entities) => {
                    let entity = match entities.next() {
                        None => return Ok(None),
                        Some(entity) => entity,
                    };
                    let item = self.fetch_entity(entity)?;
                    Ok(Some((entity, item)))
                },
            }
        }
    }
}
