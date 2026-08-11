//! TODO

mod r#enum;
pub use r#enum::solve_enum;

mod navigate;
pub use navigate::navigate;

/// A trait for tree-like structures with key-value pairs.
pub trait TreeMap: Sized {
    /// The type of key.
    type Key<'data>: AsRef<str> + Clone;
    /// The type of value.
    type Value<'data, 'core: 'data>: Clone;

    /// The type of map.
    type Map<'data, 'core: 'data>;
    /// The type of list.
    type List<'data, 'core: 'data>;

    /// Whether the given value is a map.
    fn value_is_map(value: &Self::Value<'_, '_>) -> bool;

    /// Whether the given value is a list.
    fn value_is_list(value: &Self::Value<'_, '_>) -> bool;

    /// Convert a value into a map if possible.
    fn value_map<'data, 'core: 'data>(
        value: Self::Value<'data, 'core>,
    ) -> Option<Self::Map<'data, 'core>>;

    /// Get a value from a map by key.
    fn map_get<'data, 'core: 'data>(
        map: Self::Map<'data, 'core>,
        key: &str,
    ) -> Option<Self::Value<'data, 'core>>;

    /// Iterate over the key-value pairs in a map.
    fn map_iter<'data, 'core: 'data>(
        map: Self::Map<'data, 'core>,
    ) -> impl IntoIterator<Item = (Self::Key<'data>, Self::Value<'data, 'core>)>;

    /// Convert a value into a list if possible.
    fn value_list<'data, 'core: 'data>(
        value: Self::Value<'data, 'core>,
    ) -> Option<Self::List<'data, 'core>>;

    /// Get a value from a list by index.
    fn list_get<'data, 'core: 'data>(
        list: Self::List<'data, 'core>,
        index: usize,
    ) -> Option<Self::Value<'data, 'core>>;

    /// Iterate over the values in a list.
    fn list_iter<'data, 'core: 'data>(
        list: Self::List<'data, 'core>,
    ) -> impl IntoIterator<Item = Self::Value<'data, 'core>>;
}
