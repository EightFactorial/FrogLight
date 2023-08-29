use bevy::prelude::{Deref, DerefMut};

mod attr_trait;
use attr_trait::BlockAttributeTrait;

mod enums;
pub use enums::*;

#[derive(Debug, Deref, DerefMut)]
pub struct BlockAttributes(pub Box<dyn BlockAttributeTrait>);
