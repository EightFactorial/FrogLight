//! TODO

use core::ops::{Deref, DerefMut};

#[cfg(feature = "bevy")]
use bevy_reflect::std_traits::ReflectDefault;
use froglight_common::prelude::Identifier;
#[cfg(feature = "facet")]
use froglight_facet::{self as mc, facet::prelude::*};
use glam::{IVec3, Vec3, Vec3A};

/// A variable-length [`i32`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityVarInt(#[cfg_attr(feature = "facet", facet(mc::variable))] pub i32);

impl Deref for EntityVarInt {
    type Target = i32;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for EntityVarInt {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<i32> for EntityVarInt {
    fn from(value: i32) -> Self { Self(value) }
}
impl From<EntityVarInt> for i32 {
    fn from(value: EntityVarInt) -> Self { value.0 }
}

/// A variable-length [`i64`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityVarLong(#[cfg_attr(feature = "facet", facet(mc::variable))] pub i64);

impl Deref for EntityVarLong {
    type Target = i64;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for EntityVarLong {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<i64> for EntityVarLong {
    fn from(value: i64) -> Self { Self(value) }
}
impl From<EntityVarLong> for i64 {
    fn from(value: EntityVarLong) -> Self { value.0 }
}

/// An optional variable-length [`i32`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(mc::with = EntityOptionalVarInt::WITH))]
pub struct EntityOptionalVarInt(pub Option<i32>);

#[cfg(feature = "facet")]
#[expect(clippy::cast_sign_loss, reason = "Desired behavior")]
#[expect(clippy::cast_possible_wrap, reason = "Desired behavior")]
impl FacetTemplate for EntityOptionalVarInt {
    fn serialize(item: SerializeItem<'_, '_>, writer: &mut Writer<'_>) -> Result<(), WriterError> {
        let value = item.get::<Self>()?;
        let value = value.0.map_or(0, |v| v + 1) as u32;
        encode_u32_into(value, writer)
    }

    fn deserialize<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        let value = decode_u32_from(reader)?;
        let value = value.checked_sub(1).map(|v| v as i32);
        item.set(Self(value))
    }
}

impl Deref for EntityOptionalVarInt {
    type Target = Option<i32>;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for EntityOptionalVarInt {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<Option<i32>> for EntityOptionalVarInt {
    #[inline]
    fn from(value: Option<i32>) -> Self { Self(value) }
}
impl From<EntityOptionalVarInt> for Option<i32> {
    #[inline]
    fn from(value: EntityOptionalVarInt) -> Self { value.0 }
}

// -------------------------------------------------------------------------------------------------

/// An entity's position.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityVec3 {
    /// The `X` coordinate.
    pub x: f32,
    /// The `Y` coordinate.
    pub y: f32,
    /// The `Z` coordinate.
    pub z: f32,
}

impl EntityVec3 {
    /// Create a new [`EntityVec3`] from the given coordinates.
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }

    /// Create a new [`EntityVec3`] from the given [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn new_vec3(vec: Vec3) -> Self { Self::new(vec.x, vec.y, vec.z) }

    /// Create a new [`EntityVec3`] from the given [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn new_vec3a(vec: Vec3A) -> Self {
        let [x, y, z] = vec.to_array();
        Self::new(x, y, z)
    }

    /// Get this [`EntityVec3`] as a [`Vec3`].
    #[inline]
    #[must_use]
    pub const fn as_vec3(self) -> Vec3 { Vec3::new(self.x, self.y, self.z) }

    /// Get this [`EntityVec3`] as a [`Vec3A`].
    #[inline]
    #[must_use]
    pub const fn as_vec3a(self) -> Vec3A { Vec3A::new(self.x, self.y, self.z) }
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
    // <3 Azalea

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

    /// Create a new [`EntityPosition`] from the given [`IVec3`].
    #[inline]
    #[must_use]
    pub const fn new_ivec3(vec: IVec3) -> Self { Self::new(vec.x, vec.y, vec.z) }

    /// Get the `X` coordinate.
    #[must_use]
    pub const fn x(self) -> i32 {
        (self.0 << (64 - Self::X_OFFSET - Self::PACKED_X_LENGTH) >> (64 - Self::PACKED_X_LENGTH))
            as i32
    }

    /// Get the `Y` coordinate.
    #[must_use]
    pub const fn y(self) -> i32 {
        (self.0 << (64 - Self::PACKED_Y_LENGTH) >> (64 - Self::PACKED_Y_LENGTH)) as i32
    }

    /// Get the `Z` coordinate.
    #[must_use]
    pub const fn z(self) -> i32 {
        (self.0 << (64 - Self::Z_OFFSET - Self::PACKED_Z_LENGTH) >> (64 - Self::PACKED_Z_LENGTH))
            as i32
    }

    /// Get the `X`, `Y`, and `Z` coordinates.
    #[inline]
    #[must_use]
    pub const fn xyz(self) -> (i32, i32, i32) { (self.x(), self.y(), self.z()) }

    /// Get the [`IVec3`] representation of this position.
    #[inline]
    #[must_use]
    pub const fn as_ivec3(self) -> IVec3 { IVec3::new(self.x(), self.y(), self.z()) }
}

// -------------------------------------------------------------------------------------------------

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
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
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
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityBlockState(#[cfg_attr(feature = "facet", facet(mc::variable))] pub u32);

impl Deref for EntityBlockState {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for EntityBlockState {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<u32> for EntityBlockState {
    #[inline]
    fn from(value: u32) -> Self { Self(value) }
}
impl From<EntityBlockState> for u32 {
    #[inline]
    fn from(value: EntityBlockState) -> Self { value.0 }
}

/// An entity's item slot.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityItemSlot {}
