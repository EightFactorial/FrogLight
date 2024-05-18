use bevy_app::App;
use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use froglight_components::resourcekey::ResourceKey;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<FontDefinition>(); }

/// A font definition
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize)]

pub struct FontDefinition {
    /// A list of font providers.
    pub providers: Vec<FontProvider>,
}

/// A font provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FontProvider {
    /// A reference to another font.
    #[serde(rename = "reference")]
    Reference {
        /// A key to another font.
        id: ResourceKey,
    },
    /// A bitmap font.
    #[serde(rename = "bitmap")]
    Bitmap {
        /// The font texture.
        file: ResourceKey,
        /// The font size?
        ascent: i32,
        /// The characters in the font.
        chars: Vec<String>,
    },
    /// Character widths.
    #[serde(rename = "space")]
    Space {
        /// Character widths.
        advances: HashMap<String, i32>,
    },
    /// A `TrueType` or `OpenType` font.
    #[serde(rename = "ttf")]
    Ttf {
        /// The font file.
        file: ResourceKey,
        /// How much to shift the font glyphs.
        shift: [i32; 2],
        /// The font size.
        size: f32,
        /// The resolution to render the font at.
        oversample: i32,
        /// Characters to skip
        skip: Vec<String>,
    },
    /// A `Unihex` font.
    #[serde(rename = "unihex")]
    Unihex {
        /// The font archive.
        hex_file: ResourceKey,
        /// Custom character widths.
        size_overrides: UnihexSizeOverrides,
    },
}

/// Custom character widths for a [`FontProvider::Unihex`] font.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct UnihexSizeOverrides {
    /// The starting character.
    pub from: String,
    /// The ending character.
    pub to: String,
    /// The left-most column
    ///
    /// Must be between 0 and 32.
    pub left: u8,
    /// The right-most column
    ///
    /// Must be between 0 and 32.
    pub right: u8,
}
