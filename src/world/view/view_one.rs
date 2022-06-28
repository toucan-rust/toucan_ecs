use std::iter::Flatten;
use std::option::IntoIter;

use crate::component::{Component, Iter, Registry, StorageImpl};
use crate::Entity;

/// Iterator which returns *entity* of the world
/// with **shared** *borrow* of component attached to it.
///
/// Only entities that has generic component type will be returned.
#[repr(transparent)]
pub struct ViewOne<'data, C>
where
    C: Component,
{
    iter: Flatten<IntoIter<Iter<'data, C>>>,
}

impl<'data, C> ViewOne<'data, C>
where
    C: Component,
{
    pub(crate) fn new(registry: &'data Registry) -> Self {
        let iter = registry
            .get_storage()
            .map(StorageImpl::iter)
            .into_iter()
            .flatten();
        Self { iter }
    }
}

impl<'data, C> Iterator for ViewOne<'data, C>
where
    C: Component,
{
    type Item = (Entity, &'data C);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}