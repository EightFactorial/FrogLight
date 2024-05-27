use bevy::prelude::*;
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

/// Overrides for when alternative models should be used
#[derive(Debug, Default, Clone, Deref, DerefMut, Reflect)]
pub struct ItemModelOverrides(pub HashMap<String, (serde_json::Value, ResourceKey)>);

impl ItemModelOverrides {
    /// Returns a new empty [`ItemModelOverrides`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Returns the override model key if it exists and
    /// the function returns `true`.
    ///
    /// # Example
    /// ```rust
    /// use froglight_client::assets::model_manager::ItemModelOverrides;
    /// use froglight_network::common::ResourceKey;
    /// use serde_json::json;
    ///
    /// let test_key = ResourceKey::new_inline("froglight:test");
    ///
    /// let mut overrides = ItemModelOverrides::default();
    /// overrides.insert(String::from("test"), (json!(1), test_key.clone()));
    ///
    /// // Check if the value is `1`, returns the key
    /// let result = overrides.get_override("test", |value| value.as_u64() == Some(1));
    /// assert_eq!(result, Some(&test_key));
    ///
    /// // Check if the value is `true`, returns `None`
    /// let result = overrides.get_override("test", |value| value.as_bool() == Some(true));
    /// assert_eq!(result, None);
    /// ```
    #[must_use]
    pub fn get_override<'a>(
        &'a self,
        key: &str,
        function: impl Fn(&serde_json::Value) -> bool,
    ) -> Option<&'a ResourceKey> {
        self.get(key).and_then(|(value, key)| if function(value) { Some(key) } else { None })
    }
}
