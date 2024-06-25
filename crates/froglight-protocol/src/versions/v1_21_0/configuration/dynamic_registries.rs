use froglight_components::resourcekey::ResourceKey;
use froglight_macros::FrogWrite;

use crate::{
    packet::RegistryData,
    protocol::{FrogRead, FrogVarRead, ReadError},
};

#[derive(Debug, Clone, PartialEq, FrogWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct DynamicRegistriesPacket {
    pub identifier: ResourceKey,
    pub registry_data: Vec<RegistryData>,
}

// TODO: Fix NBT deserialization...
impl FrogRead for DynamicRegistriesPacket {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let identifier = FrogRead::fg_read(buf)?;

        let length = usize::try_from(u32::fg_var_read(buf)?).expect("Length overflow");
        let mut registry_data = Vec::with_capacity(length);
        for _ in 0..length {
            match FrogRead::fg_read(buf) {
                Ok(data) => registry_data.push(data),
                Err(_) => break,
            }
        }

        Ok(Self { identifier, registry_data })
    }
}
