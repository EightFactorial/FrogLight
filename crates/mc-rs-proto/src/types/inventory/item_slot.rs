use mc_rs_macros::Transcode;

use crate::{
    buffer::{Decode, Encode},
    types::{ResourceLocation, UnsizedByteBuffer},
};

#[derive(Debug, Default, Clone)]
pub enum ItemSlot {
    #[default]
    Empty,
    Item(ItemSlotData),
}

impl ItemSlot {
    pub fn count(&self) -> i8 {
        match self {
            ItemSlot::Empty => 0,
            ItemSlot::Item(slot) => slot.count,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ItemSlot::Empty => true,
            ItemSlot::Item(slot) => slot.is_empty(),
        }
    }

    pub fn update_slot(&mut self) {
        if self.is_empty() {
            *self = ItemSlot::Empty;
        }
    }
}

impl Encode for ItemSlot {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        match self {
            ItemSlot::Empty => false.encode(buf),
            ItemSlot::Item(data) => {
                true.encode(buf)?;
                data.encode(buf)
            }
        }
    }
}

impl Decode for ItemSlot {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let slot = Option::<ItemSlotData>::decode(buf)?;

        Ok(match slot {
            None => ItemSlot::Empty,
            Some(data) => ItemSlot::Item(data),
        })
    }
}

#[derive(Debug, Clone, Transcode)]
pub struct ItemSlotData {
    pub kind: ResourceLocation,
    pub count: i8,
    pub nbt: UnsizedByteBuffer,
}

impl ItemSlotData {
    pub fn new(kind: ResourceLocation, count: i8, nbt: UnsizedByteBuffer) -> Self {
        Self { kind, count, nbt }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0 || self.kind.to_string().as_str() == "minecraft:air"
    }
}
