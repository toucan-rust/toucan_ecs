use crate::component::{Component, DefaultStorage};
use crate::world::{Fetch, FetchError};
use crate::{Entity, World};

pub struct FetchOptionRead<'data, C>
where
    C: Component,
{
    storage: Option<&'data DefaultStorage<C>>,
}

impl<'data, C> TryFrom<&'data World> for FetchOptionRead<'data, C>
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

impl<'data, C> Fetch<'data> for FetchOptionRead<'data, C>
where
    C: Component,
{
    type Item = Option<&'data C>;

    fn fetch(&self, entity: Entity) -> Result<Self::Item, FetchError> {
        let item = self.storage.and_then(|storage| storage.get(entity));
        Ok(item)
    }
}