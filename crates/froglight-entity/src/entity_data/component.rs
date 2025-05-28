use core::any::type_name;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut};
use froglight_nbt::prelude::*;
use froglight_text::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "reflect", feature = "bevy"), reflect(Component))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct EntityBreath(i16);

impl Default for EntityBreath {
    fn default() -> Self { Self(300) }
}

impl FromTag for EntityBreath {
    #[inline]
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            NbtTag::Short(short) => Ok(Self(*short)),
            _ => Err(NbtError::MismatchedTag(type_name::<Self>(), "Short")),
        }
    }
}
impl IntoTag for EntityBreath {
    #[inline]
    fn into_tag(&self) -> Result<NbtTag, NbtError> { Ok(NbtTag::Short(self.0)) }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component), require(CustomNameVisible))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "reflect", feature = "bevy"), reflect(Component))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct CustomName(Option<FormattedText>);

impl FromTag for CustomName {
    #[inline]
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        FormattedText::from_tag(tag).map(|text| Self(Some(text)))
    }
}
impl IntoTag for CustomName {
    #[inline]
    fn into_tag(&self) -> Result<NbtTag, NbtError> {
        match &self.0 {
            Some(text) => text.into_tag(),
            None => todo!("Should never be called with `None`"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "reflect", feature = "bevy"), reflect(Component))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct CustomNameVisible(bool);

impl Default for CustomNameVisible {
    fn default() -> Self { Self(true) }
}

impl FromTag for CustomNameVisible {
    #[inline]
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            NbtTag::Byte(byte) if byte == &0 => Ok(Self(false)),
            NbtTag::Byte(byte) if byte == &1 => Ok(Self(true)),
            _ => Err(NbtError::MismatchedTag(type_name::<Self>(), "Byte")),
        }
    }
}
impl IntoTag for CustomNameVisible {
    #[inline]
    fn into_tag(&self) -> Result<NbtTag, NbtError> { Ok(NbtTag::Byte(i8::from(self.0))) }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component), require(AppearsOnFire))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "reflect", feature = "bevy"), reflect(Component))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct OnFire(i16);

impl Default for OnFire {
    fn default() -> Self { Self(-20) }
}

impl FromTag for OnFire {
    #[inline]
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            NbtTag::Short(short) => Ok(Self(*short)),
            _ => Err(NbtError::MismatchedTag(type_name::<Self>(), "Short")),
        }
    }
}
impl IntoTag for OnFire {
    #[inline]
    fn into_tag(&self) -> Result<NbtTag, NbtError> { Ok(NbtTag::Short(self.0)) }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "reflect", feature = "bevy"), reflect(Component))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct AppearsOnFire(bool);

impl FromTag for AppearsOnFire {
    #[inline]
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            NbtTag::Byte(byte) if byte == &0 => Ok(Self(false)),
            NbtTag::Byte(byte) if byte == &1 => Ok(Self(true)),
            _ => Err(NbtError::MismatchedTag(type_name::<Self>(), "Byte")),
        }
    }
}
impl IntoTag for AppearsOnFire {
    #[inline]
    fn into_tag(&self) -> Result<NbtTag, NbtError> { Ok(NbtTag::Byte(i8::from(self.0))) }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "reflect", feature = "bevy"), reflect(Component))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct IsGlowing(bool);

impl FromTag for IsGlowing {
    #[inline]
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            NbtTag::Byte(byte) if byte == &0 => Ok(Self(false)),
            NbtTag::Byte(byte) if byte == &1 => Ok(Self(true)),
            _ => Err(NbtError::MismatchedTag(type_name::<Self>(), "Byte")),
        }
    }
}
impl IntoTag for IsGlowing {
    #[inline]
    fn into_tag(&self) -> Result<NbtTag, NbtError> { Ok(NbtTag::Byte(i8::from(self.0))) }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "reflect", feature = "bevy"), reflect(Component))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct IsInvulnerable(bool);

impl FromTag for IsInvulnerable {
    #[inline]
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            NbtTag::Byte(byte) if byte == &0 => Ok(Self(false)),
            NbtTag::Byte(byte) if byte == &1 => Ok(Self(true)),
            _ => Err(NbtError::MismatchedTag(type_name::<Self>(), "Byte")),
        }
    }
}
impl IntoTag for IsInvulnerable {
    #[inline]
    fn into_tag(&self) -> Result<NbtTag, NbtError> { Ok(NbtTag::Byte(i8::from(self.0))) }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "reflect", feature = "bevy"), reflect(Component))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct HasGravity(bool);

impl Default for HasGravity {
    fn default() -> Self { Self(true) }
}

impl FromTag for HasGravity {
    #[inline]
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            NbtTag::Byte(byte) if byte == &0 => Ok(Self(false)),
            NbtTag::Byte(byte) if byte == &1 => Ok(Self(true)),
            _ => Err(NbtError::MismatchedTag(type_name::<Self>(), "Byte")),
        }
    }
}
impl IntoTag for HasGravity {
    #[inline]
    fn into_tag(&self) -> Result<NbtTag, NbtError> { Ok(NbtTag::Byte(i8::from(self.0))) }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "reflect", feature = "bevy"), reflect(Component))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct PortalCooldown(u32);

impl FromTag for PortalCooldown {
    #[inline]
    #[expect(clippy::cast_sign_loss)]
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            NbtTag::Int(int) => Ok(Self(*int as u32)),
            _ => Err(NbtError::MismatchedTag(type_name::<Self>(), "Int")),
        }
    }
}
impl IntoTag for PortalCooldown {
    #[inline]
    #[expect(clippy::cast_possible_wrap)]
    fn into_tag(&self) -> Result<NbtTag, NbtError> { Ok(NbtTag::Int(self.0 as i32)) }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "reflect", feature = "bevy"), reflect(Component))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct IsSilent(bool);

impl FromTag for IsSilent {
    #[inline]
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            NbtTag::Byte(byte) if byte == &0 => Ok(Self(false)),
            NbtTag::Byte(byte) if byte == &1 => Ok(Self(true)),
            _ => Err(NbtError::MismatchedTag(type_name::<Self>(), "Byte")),
        }
    }
}
impl IntoTag for IsSilent {
    #[inline]
    fn into_tag(&self) -> Result<NbtTag, NbtError> { Ok(NbtTag::Byte(i8::from(self.0))) }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "reflect", feature = "bevy"), reflect(Component))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TicksFrozen(u32);

impl FromTag for TicksFrozen {
    #[inline]
    #[expect(clippy::cast_sign_loss)]
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            NbtTag::Int(int) => Ok(Self(*int as u32)),
            _ => Err(NbtError::MismatchedTag(type_name::<Self>(), "Int")),
        }
    }
}
impl IntoTag for TicksFrozen {
    #[inline]
    #[expect(clippy::cast_possible_wrap)]
    fn into_tag(&self) -> Result<NbtTag, NbtError> { Ok(NbtTag::Int(self.0 as i32)) }
}
