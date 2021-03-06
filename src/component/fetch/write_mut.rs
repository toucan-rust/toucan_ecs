use crate::component::{Component, Storage, StorageImpl};
use crate::world::{FetchError, FetchMut, WorldDataMut};
use crate::Entity;

#[repr(transparent)]
pub struct FetchWriteMut<'data, C>
where
    C: Component,
{
    storage: &'data mut StorageImpl<C>,
}

impl<'data, C> TryFrom<WorldDataMut<'data>> for FetchWriteMut<'data, C>
where
    C: Component,
{
    type Error = FetchError;

    fn try_from(world: WorldDataMut<'data>) -> Result<Self, Self::Error> {
        // SAFETY: must be checked by the caller.
        let storage = unsafe { world.components_mut() }
            .get_storage_mut()
            .ok_or(FetchError)?;
        Ok(Self { storage })
    }
}

impl<'data, C> FetchMut<'data> for FetchWriteMut<'data, C>
where
    C: Component,
{
    type Item = &'data mut C;

    unsafe fn fetch_mut(&'data mut self, entity: Entity) -> Result<Self::Item, FetchError> {
        self.storage.get_mut(entity).ok_or(FetchError)
    }
}
