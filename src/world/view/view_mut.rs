use std::mem::transmute;

use crate::entity::Entities;
use crate::world::query::{CheckedQuery, QueryMut};
use crate::world::{FetchMut, World};

/// Iterator which returns **unique** borrows of components.
///
/// It will be constructed from the query which is determined by the generic type.
/// Only entities that satisfy the query will be returned.
///
/// List of available types to query is located in [`world::query`](crate::world::query) module.
// TODO: turn into the lending iterator because
//  resources' mutable references could be copied freely
pub struct ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    entities: Entities<'data>,
    fetch: Option<Q::Fetch>,
}

impl<'data, Q> ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    pub(crate) fn new(world: &'data mut World) -> Self {
        let _checked = CheckedQuery::<'data, Q>::new();
        let (entities, data) = world.split_mut();
        // SAFETY: query was checked by `CheckedQuery` earlier
        let fetch = unsafe { Q::Fetch::new(data) }.ok();
        let entities = fetch
            .as_ref()
            .and_then(FetchMut::entities)
            .map(Entities::Optimized)
            .unwrap_or_else(|| Entities::All(entities.iter()));
        Self { entities, fetch }
    }
}

impl<'data, Q> Iterator for ViewMut<'data, Q>
where
    Q: QueryMut<'data>,
{
    type Item = Q;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entity = self.entities.next()?;
            // SAFETY: data returned from the fetch cannot overlap
            // with each other because entities are unique, so are the borrows
            let fetch: &'data mut Q::Fetch = unsafe { transmute(self.fetch.as_mut()?) };
            match fetch.fetch_mut(entity) {
                Ok(item) => return Some(item.into()),
                Err(_) => continue,
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let upper = self.entities.len();
        (0, Some(upper))
    }
}
