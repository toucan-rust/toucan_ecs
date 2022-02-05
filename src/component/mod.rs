pub mod set;
pub mod storage;
pub mod type_id;

/// Trait for data that can be attached to the entity.
///
/// This trait is implemented for all the types which implement [`Copy`], [`Send`], [`Sync`] traits
/// and contain no non-static references.
///
/// It implements [`Copy`] trait to ensure that type does not manage some resource
/// because copyable types cannot implement [`Drop`].
pub trait Component: Copy + Send + Sync + 'static {}

impl<T> Component for T where T: Copy + Send + Sync + 'static {}
