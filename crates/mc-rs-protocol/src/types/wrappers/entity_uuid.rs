use std::fmt::Display;

use bevy_ecs::component::Component;
use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;
use uuid::Uuid;

/// A Minecraft entity UUID.
///
/// Very different from Bevy's [Entity](bevy_ecs::entity::Entity).
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    From,
    Into,
    Component,
    Deref,
    DerefMut,
    Transcode,
)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0 ,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct EntityUuid(pub Uuid);

impl Display for EntityUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}
