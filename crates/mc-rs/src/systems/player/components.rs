use bevy::prelude::{Component, Deref, DerefMut};

/// A marker component for the player
///
/// There will only ever be one player entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct PlayerComponent;

/// Stores if the player is on the ground
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Deref, DerefMut)]
pub struct PlayerOnGround(pub bool);

/// Stores if the player is sneaking
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Deref, DerefMut)]
pub struct PlayerSneaking(pub bool);

/// Stores if the player is flying
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Deref, DerefMut)]
pub struct PlayerFlying(pub bool);

/// Stores if the player is in water
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Deref, DerefMut)]
pub struct PlayerInWater(pub bool);

/// Stores if the player is swimming
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Deref, DerefMut)]
pub struct PlayerSwimming(pub bool);
