//! TODO

use froglight_common::version::Version;

use crate::{item::UntypedItem, storage::ItemStorage};

/// A trait for resolving item types from global item IDs.
pub trait ItemResolver<V: Version> {
    /// The possible item types that can be resolved.
    type ItemEnum: Sized;

    /// Register all known [`ItemType`](crate::prelude::ItemType)s with the
    /// given [`ItemStorage`].
    fn register(storage: &mut ItemStorage<V>);

    /// Resolve the item type for the given [`UntypedItem`].
    fn resolve(item: UntypedItem<V>) -> Option<Self::ItemEnum>;
}
