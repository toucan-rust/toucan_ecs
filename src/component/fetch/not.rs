use std::marker::PhantomData;

use crate::component::marker::Not;
use crate::component::{Component, DefaultStorage};
use crate::world::{Fetch, FetchError};
use crate::{Entity, World};

pub struct FetchNot<'data, C>
where
    C: Component,
{
    storage: Option<&'data DefaultStorage<C>>,
}

impl<'data, C> TryFrom<&'data World> for FetchNot<'data, C>
where
    C: Component,
{
    type Error = FetchError;

    // noinspection DuplicatedCode
    fn try_from(world: &'data World) -> Result<Self, Self::Error> {
        let storage = world.registry().get_storage();
        Ok(Self { storage })
    }
}

impl<'data, C> Fetch<'data> for FetchNot<'data, C>
where
    C: Component,
{
    type Item = Not<'data, C>;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError> {
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