use bevy::prelude::*;

/// A marker component for the local player
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub struct LocalPlayer;

/// A marker component for the local player's head
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub struct LocalPlayerHead;
