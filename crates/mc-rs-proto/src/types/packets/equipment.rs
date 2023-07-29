use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;
use strum::{EnumString, FromRepr};

#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;
#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;

use crate::{
    buffer::{Decode, Encode},
    types::inventory::ItemSlot,
};

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct EntityEquipment(pub HashMap<EquipmentSlot, ItemSlot>);

impl Encode for EntityEquipment {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        for (index, (slot, item)) in self.0.iter().enumerate() {
            let mut byte = *slot as u8;
            if index != self.0.len() - 1 {
                byte |= 128;
            }

            byte.encode(buf)?;
            item.encode(buf)?;
        }

        Ok(())
    }
}

impl Decode for EntityEquipment {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let mut slots = HashMap::new();

        loop {
            let byte = u8::decode(buf)?;
            let slot = EquipmentSlot::from_repr(byte.into())
                .ok_or(crate::buffer::DecodeError::InvalidEnumId(byte as i32))?;

            slots.insert(slot, ItemSlot::decode(buf)?);
            if byte & 128 == 0 {
                break;
            }
        }

        Ok(Self(slots))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, FromRepr, Transcode)]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Feet,
    Legs,
    Chest,
    Head,
}
