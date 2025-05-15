use uuid::Uuid;

use crate::prelude::*;

impl FromTag for Uuid {
    fn from_tag(_tag: &NbtTag) -> Result<Self, NbtError> { todo!() }
}

impl IntoTag for Uuid {
    fn into_tag(&self) -> Result<NbtTag, NbtError> { todo!() }
}
