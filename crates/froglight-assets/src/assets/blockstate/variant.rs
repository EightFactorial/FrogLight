use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

/// A list of attributes that define a block variant
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
pub struct VariantAttributes {
    /// A list of attributes that define the variant
    pub attributes: Vec<(String, String)>,
}

impl Serialize for VariantAttributes {
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

impl<'de> Deserialize<'de> for VariantAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        if string.is_empty() {
            return Ok(VariantAttributes::default());
        }

        let mut attributes = Vec::new();
        for pair in string.split(',') {
            let mut split = pair.split('=');
            let key = split.next().unwrap();
            let value = split.next().unwrap();
            attributes.push((key.to_string(), value.to_string()));
        }
        Ok(VariantAttributes { attributes })
    }
}
