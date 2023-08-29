use bevy::prelude::{Deref, DerefMut};

mod attr_trait;
pub use attr_trait::BlockAttributeTrait;

mod attr_impl;

mod enums;
pub use enums::*;

#[derive(Debug, Deref, DerefMut)]
pub struct BlockAttributes(pub Box<dyn BlockAttributeTrait>);
