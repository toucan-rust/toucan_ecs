use crate::Entity;
use crate::resource::{marker, Resource};
use crate::world::{FetchError, FetchMut, WorldDataMut};

#[repr(transparent)]
pub struct FetchWriteMut<'data, R>
where
    R: Resource,
{
    resource: &'data mut R,
}

impl<'data, R> FetchMut<'data> for FetchWriteMut<'data, R>
where
    R: Resource,
{
    type Item = marker::ResourceMut<'data, R>;

    unsafe fn new(world: WorldDataMut<'data>) -> Result<Self, FetchError> {
        // SAFETY: must be checked by the caller.
        let resource = world.resources_mut().get_mut().ok_or(FetchError)?;
        Ok(Self { resource })
    }

    fn fetch_mut(&'data mut self, _: Entity) -> Result<Self::Item, FetchError> {
        let resource = marker::ResourceMut::new(self.resource);
        Ok(resource)
    }
}
