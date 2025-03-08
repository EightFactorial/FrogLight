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
    ///
    /// If the item type cannot be resolved,
    /// the original [`UntypedItem`] is returned.
    #[expect(clippy::missing_errors_doc)]
    fn resolve(item: UntypedItem<V>) -> Result<Self::ItemEnum, UntypedItem<V>>;
}
