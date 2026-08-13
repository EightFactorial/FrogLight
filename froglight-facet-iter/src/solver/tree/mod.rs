//! TODO

mod r#enum;
pub use r#enum::solve_enum;

mod navigate;
pub use navigate::{naviate_field, navigate_tree};

/// A trait for tree-like structures with key-value pairs.
pub trait TreeMap: Sized {
    /// The type of key.
    type Key<'index>: AsRef<str> + Clone;
    /// The type of value.
    type Value<'index, 'core: 'index>: Clone;

    /// The type of map.
    type Map<'index, 'core: 'index>: Clone;
    /// The type of list.
    type List<'index, 'core: 'index>: Clone;

    /// Whether the given value is a map.
    fn value_is_map(value: &Self::Value<'_, '_>) -> bool;

    /// Whether the given value is a list.
    fn value_is_list(value: &Self::Value<'_, '_>) -> bool;

    /// Convert a value into a map if possible.
    fn value_map<'index, 'core>(
        value: Self::Value<'index, 'core>,
    ) -> Option<Self::Map<'index, 'core>>;

    /// Returns whether the given map contains the specified key.
    fn map_contains(map: &Self::Map<'_, '_>, key: &str) -> bool;

    /// Get a value from a map by key.
    fn map_get<'index, 'core>(
        map: Self::Map<'index, 'core>,
        key: &str,
    ) -> Option<Self::Value<'index, 'core>>;

    /// Iterate over the key-value pairs in a map.
    fn map_iter<'index, 'core: 'index>(
        map: Self::Map<'index, 'core>,
    ) -> impl IntoIterator<Item = (Self::Key<'index>, Self::Value<'index, 'core>)>;

    /// Convert a value into a list if possible.
    fn value_list<'index, 'core>(
        value: Self::Value<'index, 'core>,
    ) -> Option<Self::List<'index, 'core>>;

    /// Get a value from a list by index.
    fn list_get<'index, 'core>(
        list: Self::List<'index, 'core>,
        index: usize,
    ) -> Option<Self::Value<'index, 'core>>;

    /// Iterate over the values in a list.
    fn list_iter<'index, 'core: 'index>(
        list: Self::List<'index, 'core>,
    ) -> impl IntoIterator<Item = Self::Value<'index, 'core>>;
}
