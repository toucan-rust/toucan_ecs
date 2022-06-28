use std::marker::PhantomData;

use crate::component::marker::Not;
use crate::component::{Component, Storage, StorageImpl};
use crate::error::{FetchError, FetchResult};
use crate::world::{FetchMut, WorldDataMut};
use crate::Entity;

#[repr(transparent)]
pub struct FetchNotMut<'data, C>
where
    C: Component,
{
    storage: Option<&'data StorageImpl<C>>,
}

impl<'data, C> FetchMut<'data> for FetchNotMut<'data, C>
where
    C: Component,
{
    type Item = Not<C>;

    // noinspection DuplicatedCode
    unsafe fn new(data: WorldDataMut<'data>) -> FetchResult<Self> {
        // SAFETY: must be checked by the caller.
        let storage = data.components().get_storage();
        Ok(Self { storage })
    }

    // noinspection DuplicatedCode
    fn fetch_mut(&mut self, entity: Entity) -> FetchResult<Self::Item> {
        match self.storage {
            None => Ok(Not(PhantomData)),
            Some(storage) => {
                let component = storage.get(entity);
                match component {
                    None => Ok(Not(PhantomData)),
                    Some(_) => Err(FetchError),
                }
            }
        }
    }
}
