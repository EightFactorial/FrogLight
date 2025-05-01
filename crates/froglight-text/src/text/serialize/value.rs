use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::text::component::ValueComponent;

impl Serialize for ValueComponent {
    fn serialize<S>(&self, _ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        todo!()
    }
}
impl<'de> Deserialize<'de> for ValueComponent {
    fn deserialize<D>(_de: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        todo!()
    }
}
