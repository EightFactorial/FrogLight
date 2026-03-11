//! TODO

#[cfg(feature = "bevy")]
use bevy_reflect::std_traits::ReflectDefault;
#[cfg(feature = "facet")]
use facet::{Partial, Peek};
#[cfg(feature = "facet")]
use facet_minecraft::{
    self as mc, DeserializeFn, SerializeFn,
    deserialize::{InputCursor, bytes_to_variable, error::DeserializeValueError},
    replace_with::replace_with_or_abort,
    serialize::{buffer::SerializeWriter, error::SerializeIterError, variable_to_bytes},
};
use froglight_common::prelude::Identifier;

/// A variable-length [`i32`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityVarInt(#[cfg_attr(feature = "facet", facet(mc::variable))] pub i32);

/// A variable-length [`i64`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityVarLong(#[cfg_attr(feature = "facet", facet(mc::variable))] pub i64);

/// An optional variable-length [`i32`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(mc::serialize = EntityOptionalVarInt::SERIALIZE))]
#[cfg_attr(feature = "facet", facet(mc::deserialize = EntityOptionalVarInt::DESERIALIZE))]
pub struct EntityOptionalVarInt(pub Option<i32>);

#[cfg(feature = "facet")]
#[expect(clippy::cast_sign_loss, reason = "Desired behavior")]
#[expect(clippy::cast_possible_truncation, reason = "Desired behavior")]
impl EntityOptionalVarInt {
    const DESERIALIZE: DeserializeFn =
        DeserializeFn::new(Self::facet_deserialize, Self::facet_deserialize);
    const SERIALIZE: SerializeFn = SerializeFn::new(Self::facet_serialize);

    fn facet_deserialize<'facet, const BORROW: bool>(
        partial: &mut Partial<'facet, BORROW>,
        cursor: &mut InputCursor<'_, 'facet>,
    ) -> Result<(), DeserializeValueError> {
        let (len, val) = bytes_to_variable(cursor.as_slice())?;
        cursor.consume(len)?;

        let val = if val == 0 { None } else { Some((val - 1) as i32) };
        replace_with_or_abort(partial, |partial| partial.set(Self(val)).unwrap());
        Ok(())
    }

    fn facet_serialize<'input, 'facet>(
        peek: Peek<'input, 'facet>,
        writer: &mut dyn SerializeWriter,
    ) -> Result<(), SerializeIterError<'input, 'facet>> {
        let val = peek.get::<Self>()?.0;

        let mut buffer = [0; _];
        let len = if let Some(val) = val {
            variable_to_bytes(i128::from(val + 1) as u128, &mut buffer)
        } else {
            1
        };
        if writer.write_data(&buffer[..len]) { Ok(()) } else { Err(SerializeIterError::new()) }
    }
}

// -------------------------------------------------------------------------------------------------

/// An entity's position.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityVec3 {
    /// The `X` coordinate.
    pub x: f32,
    /// The `Y` coordinate.
    pub y: f32,
    /// The `Z` coordinate.
    pub z: f32,
}

/// An entity's dimension and position.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityGlobalPosition {
    /// The entity's dimension.
    pub dimension: Identifier<'static>,
    /// The entity's position.
    pub position: EntityPosition,
}

/// An entity's position.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityPosition(u64);

impl EntityPosition {
    const PACKED_X_LENGTH: u64 = 1 + 25;
    const PACKED_X_MASK: u64 = (1 << Self::PACKED_X_LENGTH) - 1;
    const PACKED_Y_LENGTH: u64 = 64 - Self::PACKED_X_LENGTH - Self::PACKED_Z_LENGTH;
    const PACKED_Y_MASK: u64 = (1 << Self::PACKED_Y_LENGTH) - 1;
    const PACKED_Z_LENGTH: u64 = Self::PACKED_X_LENGTH;
    const PACKED_Z_MASK: u64 = (1 << Self::PACKED_Z_LENGTH) - 1;
    const X_OFFSET: u64 = Self::PACKED_Y_LENGTH + Self::PACKED_Z_LENGTH;
    const Z_OFFSET: u64 = Self::PACKED_Y_LENGTH;

    /// Create a new [`EntityPosition`] from the given coordinates.
    #[must_use]
    #[expect(clippy::cast_sign_loss, reason = "Desired behavior")]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        let x = (x as u64 & Self::PACKED_X_MASK) << Self::X_OFFSET;
        let y = y as u64 & Self::PACKED_Y_MASK;
        let z = (z as u64 & Self::PACKED_Z_MASK) << Self::Z_OFFSET;
        Self(x | y | z)
    }

    /// Get the `X` coordinate.
    #[must_use]
    pub const fn x(&self) -> i32 {
        (self.0 << (64 - Self::X_OFFSET - Self::PACKED_X_LENGTH) >> (64 - Self::PACKED_X_LENGTH))
            as i32
    }

    /// Get the `Y` coordinate.
    #[must_use]
    pub const fn y(&self) -> i32 {
        (self.0 << (64 - Self::PACKED_Y_LENGTH) >> (64 - Self::PACKED_Y_LENGTH)) as i32
    }

    /// Get the `Z` coordinate.
    #[must_use]
    pub const fn z(&self) -> i32 {
        (self.0 << (64 - Self::Z_OFFSET - Self::PACKED_Z_LENGTH) >> (64 - Self::PACKED_Z_LENGTH))
            as i32
    }

    /// Get the `X`, `Y`, and `Z` coordinates.
    #[must_use]
    pub const fn xyz(&self) -> (i32, i32, i32) { (self.x(), self.y(), self.z()) }
}

/// The rotation of an entity.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityRotation {
    /// Rotation on `X`
    pub x: f32,
    /// Rotation on `Y`
    pub y: f32,
    /// Rotation on `Z`
    pub z: f32,
}

/// The rotation of an entity as a quaternion.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityQuaternion {
    /// Rotation on `X`
    pub x: f32,
    /// Rotation on `Y`
    pub y: f32,
    /// Rotation on `Z`
    pub z: f32,
    /// Rotation on `W`
    pub w: f32,
}

/// The direction of an entity.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum EntityDirection {
    /// Down
    Down = 0,
    /// Up
    Up = 1,
    /// North
    North = 2,
    /// South
    South = 3,
    /// West
    West = 4,
    /// East
    East = 5,
}

// -------------------------------------------------------------------------------------------------

/// An entity's villager data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityVillagerData {
    /// The villager's type.
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub kind: u32,
    /// The villager's profession.
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub profession: u32,
    /// The villager's level.
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub level: u32,
}

// -------------------------------------------------------------------------------------------------

/// An entity's block state.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityBlockState(#[cfg_attr(feature = "facet", facet(mc::variable))] pub u32);

/// An entity's item slot.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityItemSlot {}
