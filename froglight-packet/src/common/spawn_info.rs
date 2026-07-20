//! TODO
#![allow(missing_docs, reason = "TODO")]

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
#[cfg(feature = "facet")]
use facet::Facet;
use froglight_common::prelude::Identifier;
#[cfg(feature = "facet")]
use froglight_facet::facet::prelude::*;
use froglight_world::component::DimensionPos;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
pub struct PlayerSpawnInfo {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub dimension_type: u32,
    pub dimension: Identifier<'static>,
    pub seed: i64,
    pub gamemode: u8,
    #[cfg_attr(feature = "facet", facet(mc::with = PlayerSpawnInfo::WITH))]
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
#[allow(clippy::cast_sign_loss, reason = "Desired behavior")]
impl FacetTemplate for PlayerSpawnInfo {
    fn serialize(item: SerializeItem<'_, '_>, writer: &mut Writer<'_>) -> Result<(), WriterError> {
        let value = item.get::<Option<u8>>()?;
        let byte = value.unwrap_or(-1i8 as u8);
        writer.write_byte(byte)
    }

    fn deserialize<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        let byte = reader.read_byte()?;
        let value = if byte == (-1i8 as u8) { None } else { Some(byte) };
        item.set(value)
    }
}
