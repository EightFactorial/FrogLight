use bevy_app::App;
use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use compact_str::{CompactString, ToCompactString};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use super::SingleOrMultiModel;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<BlockAttributes>().register_type::<BlockStateVariants>();
}

/// A list of block state variants
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
pub struct BlockStateVariants {
    /// A list of block state variants
    pub variants: HashMap<BlockAttributes, SingleOrMultiModel>,
}

/// A list of attributes that define a block variant
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
pub struct BlockAttributes {
    /// A list of attributes that define the variant
    #[reflect(ignore)]
    pub attributes: Vec<(CompactString, CompactString)>,
}

impl Serialize for BlockAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut result = String::new();
        for (index, (key, value)) in self.attributes.iter().enumerate() {
            if index > 0 {
                result.push(',');
            }
            result.push_str(key);
            result.push('=');
            result.push_str(value);
        }
        serializer.serialize_str(&result)
    }
}

impl<'de> Deserialize<'de> for BlockAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = CompactString::deserialize(deserializer)?;
        if string.is_empty() {
            return Ok(BlockAttributes::default());
        }

        let mut attributes = Vec::new();
        for pair in string.split(',') {
            let mut split = pair.split('=');
            let key = split.next().unwrap();
            let value = split.next().unwrap();
            attributes.push((key.to_compact_string(), value.to_compact_string()));
        }
        Ok(BlockAttributes { attributes })
    }
}
