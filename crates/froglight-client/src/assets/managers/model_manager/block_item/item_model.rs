use bevy::prelude::*;
use froglight_assets::assets::{
    model::{GuiLight, ModelDisplayTransform},
    ItemModelDefinition, ModelDefinition,
};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

/// An Item Model
#[derive(Debug, Reflect)]
pub struct ItemModel {
    /// Model transforms
    ///
    /// Indexed via
    /// [`DisplayPosition`](froglight_assets::assets::model::DisplayPosition).
    pub model_transforms: [ModelDisplayTransform; 7],

    /// The gui light for the model
    pub gui_light: GuiLight,

    /// Item model mesh
    ///
    /// This is used for rendering the item in the player's hand,
    /// in the inventory, in item frames, etc.
    pub item_model: Handle<Mesh>,

    /// Overrides for when alternative models should be used
    pub overrides: ItemModelOverrides,
}

impl ItemModel {
    /// Resolves an [`ItemModelDefinition`] into an [`ItemModel`].
    #[must_use]
    pub fn resolve_definition(
        _key: &ResourceKey,
        _def: &ItemModelDefinition,
        _definitions: &HashMap<ResourceKey, ModelDefinition>,
        _mesh_assets: &mut Assets<Mesh>,
    ) -> Self {
        todo!()
    }
}

/// Overrides for when alternative models should be used
#[derive(Debug, Default, Clone, Deref, DerefMut, Reflect)]
pub struct ItemModelOverrides(pub HashMap<String, (serde_json::Value, ResourceKey)>);

impl ItemModelOverrides {
    /// Returns a new empty [`ItemModelOverrides`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Returns `true` if the [`ItemModelOverrides`] is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

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
