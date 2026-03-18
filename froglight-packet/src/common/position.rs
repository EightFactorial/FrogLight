//! TODO

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
#[cfg(feature = "facet")]
use facet::Facet;
use froglight_common::prelude::EntityId;
use glam::{DVec3, Vec3};

use crate::generated::v26_1::play::{
    MoveEntityPosRotS2CPacket, MoveEntityPosS2CPacket, MoveEntityRotS2CPacket,
};

/// Data about an entity's position.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(Facet))]
pub struct EntityPositionUpdateData {
    /// The entity's ID.
    pub entity_id: EntityId,
    /// A position delta.
    pub delta: Option<PositionDelta>,
    /// A `y` and `x` rotation.
    pub yaw_pitch: Option<(i8, i8)>,
    /// Whether the entity is on the ground.
    pub on_ground: bool,
}

impl From<MoveEntityPosS2CPacket> for EntityPositionUpdateData {
    fn from(value: MoveEntityPosS2CPacket) -> Self {
        Self {
            entity_id: value.entity_id.0,
            delta: Some(value.delta),
            yaw_pitch: None,
            on_ground: value.on_ground,
        }
    }
}
impl From<MoveEntityPosRotS2CPacket> for EntityPositionUpdateData {
    fn from(value: MoveEntityPosRotS2CPacket) -> Self {
        Self {
            entity_id: value.entity_id.0,
            delta: Some(value.delta),
            yaw_pitch: Some((value.yaw, value.pitch)),
            on_ground: value.on_ground,
        }
    }
}
impl From<MoveEntityRotS2CPacket> for EntityPositionUpdateData {
    fn from(value: MoveEntityRotS2CPacket) -> Self {
        Self {
            entity_id: value.entity_id.0,
            delta: None,
            yaw_pitch: Some((value.yaw, value.pitch)),
            on_ground: value.on_ground,
        }
    }
}

// -------------------------------------------------------------------------------------------------

// TODO: Fix arrays, also DVec3?
/// Data about an entity's position and rotation.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(Facet))]
pub struct EntityPositionRotationData {
    /// The entity's position.
    pub position_x: f64,
    /// The entity's y position.
    pub position_y: f64,
    /// The entity's z position.
    pub position_z: f64,
    /// The entity's x velocity.
    pub velocity_x: f64,
    /// The entity's y velocity.
    pub velocity_y: f64,
    /// The entity's z velocity.
    pub velocity_z: f64,
    /// The entity's yaw in degrees.
    pub yaw: f32,
    /// The entity's pitch in degrees.
    pub pitch: f32,
}

// -------------------------------------------------------------------------------------------------

/// A position delta.
///
/// Cannot exceed 8 blocks in any direction.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
pub struct PositionDelta(i16, i16, i16);

impl PositionDelta {
    const DELTA_CONV: f64 = 4096.0;

    /// Create a new [`PositionDelta`] from the given x, y, and z values.
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "Cannot truncate")]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        debug_assert!(
            x <= 8. && y <= 8. && z <= 8.,
            "PositionDelta values must be between -8 and 8"
        );

        Self(
            (x * Self::DELTA_CONV) as i16,
            (y * Self::DELTA_CONV) as i16,
            (z * Self::DELTA_CONV) as i16,
        )
    }

    /// Convert the delta into `x`, `y`, and `z` values.
    #[must_use]
    pub const fn into_xyz(self) -> (f64, f64, f64) {
        (
            self.0 as f64 / Self::DELTA_CONV,
            self.1 as f64 / Self::DELTA_CONV,
            self.2 as f64 / Self::DELTA_CONV,
        )
    }

    /// Create a new [`PositionDelta`] between two [`Vec3`]s.
    #[must_use]
    pub const fn between_vecs(from: Vec3, to: Vec3) -> Self {
        Self::new((to.x - from.x) as f64, (to.y - from.y) as f64, (to.z - from.z) as f64)
    }

    /// Add the delta to the given [`Vec3`].
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "Cannot truncate")]
    pub const fn add_to_vec(self, vec: Vec3) -> Vec3 {
        Vec3::new(
            vec.x + (self.0 as f64 / Self::DELTA_CONV) as f32,
            vec.y + (self.1 as f64 / Self::DELTA_CONV) as f32,
            vec.z + (self.2 as f64 / Self::DELTA_CONV) as f32,
        )
    }

    /// Create a new [`PositionDelta`] between two [`DVec3`]s.
    #[must_use]
    pub const fn between_dvecs(from: DVec3, to: DVec3) -> Self {
        Self::new(to.x - from.x, to.y - from.y, to.z - from.z)
    }

    /// Add the delta to the given [`DVec3`].
    #[must_use]
    pub const fn add_to_dvec(self, vec: DVec3) -> DVec3 {
        DVec3::new(
            vec.x + (self.0 as f64 / Self::DELTA_CONV),
            vec.y + (self.1 as f64 / Self::DELTA_CONV),
            vec.z + (self.2 as f64 / Self::DELTA_CONV),
        )
    }
}
