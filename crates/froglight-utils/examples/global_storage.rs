//! TODO
#![allow(missing_docs)]

use core::any::TypeId;

use downcast_rs::DowncastSync;
use froglight_utils::global_storage;

fn main() {
    static FIRST_STR: &str = "first";
    static SECOND_STR: &str = "second";
    static THIRD_STR: &str = "third";

    // Try storing and fetching `&str` values.
    AnyStorage::insert(&FIRST_STR);
    assert_eq!(AnyStorage::fetch::<&str>(), Some(&FIRST_STR));
    assert_ne!(AnyStorage::fetch::<&str>(), Some(&SECOND_STR));
    assert_ne!(AnyStorage::fetch::<&str>(), Some(&THIRD_STR));

    AnyStorage::insert(&SECOND_STR);
    assert_ne!(AnyStorage::fetch::<&str>(), Some(&FIRST_STR));
    assert_eq!(AnyStorage::fetch::<&str>(), Some(&SECOND_STR));
    assert_ne!(AnyStorage::fetch::<&str>(), Some(&THIRD_STR));

    static FIRST_NUM: usize = 0;
    static SECOND_NUM: usize = 42;
    static THIRD_NUM: usize = 123;

    // Try storing and fetching `usize` values.
    AnyStorage::insert(&FIRST_NUM);
    assert_eq!(AnyStorage::fetch::<usize>(), Some(&FIRST_NUM));
    assert_ne!(AnyStorage::fetch::<usize>(), Some(&SECOND_NUM));
    assert_ne!(AnyStorage::fetch::<usize>(), Some(&THIRD_NUM));

    AnyStorage::insert(&SECOND_NUM);
    assert_ne!(AnyStorage::fetch::<usize>(), Some(&FIRST_NUM));
    assert_eq!(AnyStorage::fetch::<usize>(), Some(&SECOND_NUM));
    assert_ne!(AnyStorage::fetch::<usize>(), Some(&THIRD_NUM));

    // See that the storage still holds `SECOND_STR`
    // even after inserting and retrieving a `usize`.
    assert_ne!(AnyStorage::fetch::<&str>(), Some(&FIRST_STR));
    assert_eq!(AnyStorage::fetch::<&str>(), Some(&SECOND_STR));
    assert_ne!(AnyStorage::fetch::<&str>(), Some(&THIRD_STR));
}

// -------------------------------------------------------------------------------------------------

/// A global storage that can store one of any type of static value.
#[derive(Debug, Default, Clone, Copy)]
pub struct AnyStorage;

global_storage!(AnyStorage<dyn DowncastSync, usize>);

impl AnyStorage {
    /// Insert a value into the global storage.
    pub fn insert<T: DowncastSync + 'static>(value: &'static T) {
        Self::global().write().store(value.as_any().type_id(), value);
    }

    /// Fetch a value from the global storage.
    #[must_use]
    pub fn fetch<T: DowncastSync + 'static>() -> Option<&'static T> {
        Self::global()
            .read()
            .get(&TypeId::of::<T>())
            .and_then(|val| val.inner().as_any().downcast_ref::<T>())
    }
}
