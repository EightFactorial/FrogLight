use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;
use strum::{EnumString, FromRepr};

use crate::{
    buffer::{Decode, Encode},
    types::inventory::ItemSlot,
};

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct EntityEquipment(pub Vec<(EquipmentSlot, ItemSlot)>);

impl Encode for EntityEquipment {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        for (i, (slot, item)) in self.iter().enumerate() {
            let mut slot_byte = u8::from(*slot);
            if i != self.len() - 1 {
                slot_byte |= 128;
            }
            slot_byte.encode(buf)?;
            item.encode(buf)?;
        }

        Ok(())
    }
}

impl Decode for EntityEquipment {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let mut equipment = Vec::new();
        loop {
            let byte = u8::decode(buf)?;

            let slot_byte = vec![byte & 127];
            let slot = EquipmentSlot::decode(&mut slot_byte.as_slice())?;

            let item = ItemSlot::decode(buf)?;
            equipment.push((slot, item));

            if byte & 128 == 0 {
                break;
            }
        }

        Ok(Self(equipment))
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

impl From<EquipmentSlot> for u8 {
    fn from(slot: EquipmentSlot) -> Self {
        match slot {
            EquipmentSlot::MainHand => 0,
            EquipmentSlot::OffHand => 1,
            EquipmentSlot::Feet => 2,
            EquipmentSlot::Legs => 3,
            EquipmentSlot::Chest => 4,
            EquipmentSlot::Head => 5,
        }
    }
}
