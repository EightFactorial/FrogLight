//! TODO
#![allow(missing_docs, reason = "TODO")]

#[cfg(feature = "facet")]
use facet::{Partial, Peek};
#[cfg(feature = "facet")]
use facet_minecraft::{
    self as mc, DeserializeFn, SerializeFn,
    deserialize::{InputCursor, error::DeserializeValueError},
    replace_with::replace_with_or_abort,
    serialize::{buffer::SerializeWriter, error::SerializeIterError},
};
use froglight_common::prelude::Identifier;
use froglight_world::component::DimensionPos;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct PlayerSpawnInfo {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub dimension_type: u32,
    pub dimension: Identifier<'static>,
    pub seed: i64,
    pub gamemode: u8,
    #[cfg_attr(feature = "facet", facet(mc::serialize = PlayerSpawnInfo::PREVIOUS_GAMEMODE_SERIALIZE))]
    #[cfg_attr(feature = "facet", facet(mc::deserialize = PlayerSpawnInfo::PREVIOUS_GAMEMODE_DESERIALIZE))]
    pub previous_gamemode: Option<u8>,
    pub is_debug: bool,
    pub is_flat: bool,
    pub last_death: Option<DimensionPos>,
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub portal_cooldown: u32,
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub sea_level: i32,
}

#[cfg(feature = "facet")]
impl PlayerSpawnInfo {
    const PREVIOUS_GAMEMODE_DESERIALIZE: DeserializeFn =
        DeserializeFn::new(Self::facet_deserialize, Self::facet_deserialize);
    const PREVIOUS_GAMEMODE_SERIALIZE: SerializeFn = SerializeFn::new(Self::facet_serialize);

    #[expect(clippy::cast_sign_loss, reason = "Desired behavior")]
    fn facet_deserialize<'facet, const BORROW: bool>(
        partial: &mut Partial<'facet, BORROW>,
        cursor: &mut InputCursor<'_, 'facet>,
    ) -> Result<(), DeserializeValueError> {
        let byte = cursor.take(1)?[0];
        let value = if byte == -1i8 as u8 { None } else { Some(byte) };
        replace_with_or_abort(partial, |partial| partial.set(value).unwrap());
        Ok(())
    }

    #[expect(clippy::cast_sign_loss, reason = "Desired behavior")]
    fn facet_serialize<'mem, 'facet>(
        peek: Peek<'mem, 'facet>,
        writer: &mut dyn SerializeWriter,
    ) -> Result<(), SerializeIterError<'mem, 'facet>> {
        let value = peek.get::<Option<u8>>()?;
        let byte = value.map_or(-1i8 as u8, |v| v);
        if writer.write_data(&[byte]) { Ok(()) } else { Err(SerializeIterError::new()) }
    }
}
