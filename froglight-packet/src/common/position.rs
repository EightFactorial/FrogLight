//! TODO

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
#[cfg(feature = "facet")]
use facet::{Facet, Partial, Peek};
#[cfg(feature = "facet")]
use facet_minecraft::{
    self as mc, DeserializeFn, SerializeFn,
    deserialize::{InputCursor, error::DeserializeValueError},
    replace_with::replace_with_or_abort,
    serialize::{buffer::SerializeWriter, error::SerializeIterError},
};
use froglight_common::prelude::EntityId;
use glam::{DVec3, Quat, Vec3};

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

impl EntityPositionRotationData {
    /// Apply the position and rotation using the given relative flags.
    #[expect(clippy::cast_possible_truncation, reason = "Ignored")]
    pub fn apply_relative(
        &self,
        position: &mut Vec3,
        _rotation: &mut Quat,
        velocity: &mut Vec3,
        flags: &EntityRelativeFlags,
    ) {
        macro_rules! apply {
            ($data:expr, $field:expr, $condition:expr) => {
                if $condition {
                    *$field += $data;
                } else {
                    *$field = $data;
                }
            };
        }

        apply!(self.position_x as f32, &mut position.x, flags.x);
        apply!(self.position_y as f32, &mut position.y, flags.y);
        apply!(self.position_z as f32, &mut position.z, flags.z);

        apply!(self.velocity_x as f32, &mut velocity.x, flags.delta_x);
        apply!(self.velocity_y as f32, &mut velocity.y, flags.delta_y);
        apply!(self.velocity_z as f32, &mut velocity.z, flags.delta_z);
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
#[cfg_attr(feature = "facet", facet(mc::serialize = EntityRelativeFlags::SERIALIZE))]
#[cfg_attr(feature = "facet", facet(mc::deserialize = EntityRelativeFlags::DESERIALIZE))]
#[expect(clippy::struct_excessive_bools, reason = "That's just how it is")]
#[expect(missing_docs, reason = "TODO")]
pub struct EntityRelativeFlags {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub y_rot: bool,
    pub x_rot: bool,
    pub delta_x: bool,
    pub delta_y: bool,
    pub delta_z: bool,
    pub rotate_delta: bool,
}

impl EntityRelativeFlags {
    /// An [`EntityRelativeMovements`] where all data is absolute.
    pub const ABSOLUTE: Self = Self {
        x: false,
        y: false,
        z: false,
        y_rot: false,
        x_rot: false,
        delta_x: false,
        delta_y: false,
        delta_z: false,
        rotate_delta: false,
    };
    /// An [`EntityRelativeMovements`] where all data is relative.
    pub const RELATIVE: Self = Self {
        x: true,
        y: true,
        z: true,
        y_rot: true,
        x_rot: true,
        delta_x: true,
        delta_y: true,
        delta_z: true,
        rotate_delta: true,
    };
}

#[cfg(feature = "facet")]
impl EntityRelativeFlags {
    const DESERIALIZE: DeserializeFn =
        DeserializeFn::new(Self::facet_deserialize, Self::facet_deserialize);
    const SERIALIZE: SerializeFn = SerializeFn::new(Self::facet_serialize);

    fn facet_deserialize<'facet, const BORROW: bool>(
        partial: &mut Partial<'facet, BORROW>,
        cursor: &mut InputCursor<'_, 'facet>,
    ) -> Result<(), DeserializeValueError> {
        let data = u32::from_be_bytes(*cursor.take_array::<4>()?);
        replace_with_or_abort(partial, |partial| {
            partial
                .set(Self {
                    x: data & 0b0000_0000_0001 != 0,
                    y: data & 0b0000_0000_0010 != 0,
                    z: data & 0b0000_0000_0100 != 0,
                    y_rot: data & 0b0000_0000_1000 != 0,
                    x_rot: data & 0b0000_0001_0000 != 0,
                    delta_x: data & 0b0000_0010_0000 != 0,
                    delta_y: data & 0b0000_0100_0000 != 0,
                    delta_z: data & 0b0000_1000_0000 != 0,
                    rotate_delta: data & 0b0001_0000_0000 != 0,
                })
                .unwrap()
        });
        Ok(())
    }

    fn facet_serialize<'mem, 'facet>(
        peek: Peek<'mem, 'facet>,
        writer: &mut dyn SerializeWriter,
    ) -> Result<(), SerializeIterError<'mem, 'facet>> {
        let data = peek.get::<Self>()?;
        let mut output = 0u32;
        if data.x {
            output |= 0b0000_0000_0001;
        }
        if data.y {
            output |= 0b0000_0000_0010;
        }
        if data.z {
            output |= 0b0000_0000_0100;
        }
        if data.y_rot {
            output |= 0b0000_0000_1000;
        }
        if data.x_rot {
            output |= 0b0000_0001_0000;
        }
        if data.delta_x {
            output |= 0b0000_0010_0000;
        }
        if data.delta_y {
            output |= 0b0000_0100_0000;
        }
        if data.delta_z {
            output |= 0b0000_1000_0000;
        }
        if data.rotate_delta {
            output |= 0b0001_0000_0000;
        }

        if writer.write_data(&output.to_be_bytes()) {
            Ok(())
        } else {
            Err(SerializeIterError::new())
        }
    }
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
