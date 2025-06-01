#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, From, Into};
use parking_lot::RwLock;

use super::IndexedLocalStorage;

/// A globally accessible, run-time modifiable storage
/// for static values accessible by either a [`TypeId`](core::any::TypeId)
/// or an index.
#[repr(transparent)]
#[derive(Debug, Deref, From, Into)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IndexedGlobalStorage<V: ?Sized + 'static, I: From<usize> + Into<usize> + 'static>(
    &'static RwLock<IndexedLocalStorage<V, I>>,
);

impl<V: ?Sized + 'static, I: From<usize> + Into<usize> + 'static> IndexedGlobalStorage<V, I> {
    /// Create a new [`IndexedGlobalStorage`].
    #[must_use]
    pub const fn new(storage: &'static RwLock<IndexedLocalStorage<V, I>>) -> Self { Self(storage) }

    /// Get a reference to the [`IndexedLocalStorage`]'s [`RwLock`].
    #[must_use]
    pub const fn inner(&self) -> &'static RwLock<IndexedLocalStorage<V, I>> { self.0 }
}

// -------------------------------------------------------------------------------------------------
// Manual implementations for `IndexedGlobalStorage` to avoid trait bounds

impl<V: ?Sized + 'static, I: From<usize> + Into<usize>> Clone for IndexedGlobalStorage<V, I> {
    fn clone(&self) -> Self { *self }
}
impl<V: ?Sized + 'static, I: From<usize> + Into<usize>> Copy for IndexedGlobalStorage<V, I> {}

// -------------------------------------------------------------------------------------------------

/// TODO
#[macro_export]
macro_rules! global_storage {
    ($vis:vis $ident:ident < $V:ty, $I:ty > ; $vis2:vis $ident2:ident $($tt:tt)+) => {
        global_storage!($vis $ident <$V,$I>);
        global_storage!($vis2 $ident2 $($tt)+);
    };
    ($vis:vis $ident:ident < $V:ty, $I:ty > ; $($tt:tt)+) => {
        global_storage!($vis $ident <$V,$I>);
        global_storage!($vis $($tt)+);
    };


    ($vis:vis $ident:ident < $V:ty, $I:ty > $(;)?) => {
        impl $ident {
            #[must_use]
            $vis fn global() -> ::froglight_utils::storage::IndexedGlobalStorage<$V, $I> {
                global_storage!(@storage <$V,$I>)
            }
        }
    };
    (@storage < $V:ty, $I:ty >) => {{
        static STORAGE: ::parking_lot::RwLock<::froglight_utils::storage::IndexedLocalStorage<$V, $I>> =
            ::parking_lot::RwLock::new(::froglight_utils::storage::IndexedLocalStorage::<$V, $I>::new());

        ::froglight_utils::storage::IndexedGlobalStorage::new(&STORAGE)
    }}
}
