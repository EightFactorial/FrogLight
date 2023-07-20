use bevy_derive::{Deref, DerefMut};

/// A Minecraft entity ID.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct EntityId(pub u32);
