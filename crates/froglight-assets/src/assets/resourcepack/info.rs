use bevy_app::App;
use bevy_asset::Handle;
use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize};
use bevy_render::texture::Image;
use hashbrown::HashMap;
use serde::Deserialize;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ResourcePackInfo>()
        .register_type::<ResourcePackMeta>()
        .register_type::<ResourcePackFormat>()
        .register_type::<ResourcePackDescription>()
        .register_type::<SupportedFormats>()
        .register_type::<ResourcePackLanguage>();
}

/// Information about a [`ResourcePack`](super::ResourcePack).
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect)]
#[reflect(Default)]
pub struct ResourcePackInfo {
    /// The [`ResourcePack`](super::ResourcePack) icon
    ///
    /// This is the `pack.png` file.
    pub icon: Handle<Image>,
    /// [`ResourcePack`](super::ResourcePack) metadata
    pub mcmeta: ResourcePackMeta,
}

/// [`ResourcePack`](super::ResourcePack) metadata
///
/// This is the `pack.mcmeta` file.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Reflect)]
#[reflect(Default, Deserialize)]
pub struct ResourcePackMeta {
    /// The [`ResourcePack`](super::ResourcePack) format
    #[serde(default)]
    pub pack: ResourcePackFormat,
    /// Supported languages
    ///
    /// The key is the language code, which has a corresponding
    /// file in the `assets/{namespace}/lang` directory.
    #[serde(default)]
    pub language: HashMap<String, ResourcePackLanguage>,
}

/// [`ResourcePack`](super::ResourcePack) format.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Reflect)]
#[reflect(Default, Deserialize)]
pub struct ResourcePackFormat {
    /// The [`ResourcePack`](super::ResourcePack) format version
    #[serde(default)]
    pub pack_format: u32,
    /// Supported [`ResourcePack`](super::ResourcePack) formats
    #[serde(default)]
    pub supported_formats: SupportedFormats,
    /// The [`ResourcePack`](super::ResourcePack) description
    #[serde(default)]
    pub description: ResourcePackDescription,
}

/// [`ResourcePack`](super::ResourcePack) description.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Reflect)]
#[reflect(Default, Deserialize)]
pub struct ResourcePackDescription {
    /// The [`ResourcePack`](super::ResourcePack) description
    #[reflect(ignore)]
    pub description: serde_json::Value,
}

/// Supported [`ResourcePack`](super::ResourcePack) formats.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Reflect)]
#[reflect(Deserialize)]
#[serde(untagged)]
pub enum SupportedFormats {
    /// A list of format versions
    List([u32; 2]),
    /// A range of format versions
    Range {
        /// The minimum format version
        min_inclusive: u32,
        /// The maximum format version
        max_inclusive: u32,
    },
}

impl Default for SupportedFormats {
    fn default() -> Self { SupportedFormats::List([0, 0]) }
}

impl SupportedFormats {
    /// Checks if a format is supported.
    #[must_use]
    pub fn supports(&self, format: u32) -> bool {
        match self {
            SupportedFormats::List([min, max]) => format >= *min && format <= *max,
            SupportedFormats::Range { min_inclusive, max_inclusive } => {
                format >= *min_inclusive && format <= *max_inclusive
            }
        }
    }
}

/// A language in a [`ResourcePack`](super::ResourcePack).
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deserialize, Reflect)]
#[reflect(Default, Deserialize)]
pub struct ResourcePackLanguage {
    /// The language name
    pub name: String,
    /// The language region
    pub region: String,
    /// The language bidirectional flag
    ///
    /// False: left-to-right
    /// True: right-to-left
    pub bidirectional: bool,
}
