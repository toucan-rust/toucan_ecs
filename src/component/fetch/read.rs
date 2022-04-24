use crate::component::{Component, Storage, StorageImpl};
use crate::world::{Fetch, FetchError};
use crate::{Entity, World};

pub struct FetchRead<'data, C>
where
    C: Component,
{
    storage: &'data StorageImpl<C>,
}

impl<'data, C> TryFrom<&'data World> for FetchRead<'data, C>
where
    C: Component,
{
    type Error = FetchError;

    // noinspection DuplicatedCode
    fn try_from(world: &'data World) -> Result<Self, Self::Error> {
        let storage = world.registry().get_storage().ok_or(FetchError)?;
        Ok(Self { storage })
    }
}

impl<'data, C> Fetch<'data> for FetchRead<'data, C>
where
    C: Component,
{
    type Item = &'data C;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError> {
        self.storage.get(entity).ok_or(FetchError)
    }
}
