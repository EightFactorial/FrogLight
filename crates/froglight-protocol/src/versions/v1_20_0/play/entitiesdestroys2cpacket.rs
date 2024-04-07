use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use crate::common::EntityId;

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite,
)]
#[frog(tests = ["read_example"], bytes = [1, 42])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct EntitiesDestroyS2CPacket(pub Vec<EntityId>);
