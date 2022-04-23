use crate::world::Query;

use super::fetch::{FetchNot, FetchOptionRead, FetchRead};
use super::marker::Not;
use super::Component;

impl<'data, C> Query<'data> for &'data C
where
    C: Component,
{
    type Fetch = FetchRead<'data, C>;
}

impl<'data, C> Query<'data> for Option<&'data C>
where
    C: Component,
{
    type Fetch = FetchOptionRead<'data, C>;
}

impl<'data, C> Query<'data> for Not<'data, C>
where
    C: Component,
{
    type Fetch = FetchNot<'data, C>;
}