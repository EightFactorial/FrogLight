use azalea_chat::FormattedText;
use azalea_nbt::Nbt;
use bevy_math::Quat;
use derive_more::{Deref, DerefMut};
use mc_rs_macros::Transcode;
use uuid::Uuid;

use crate::{
    buffer::{Decode, Encode},
    types::{
        enums::Direction,
        inventory::ItemSlot,
        position::{BlockPos, GlobalPos},
        NonZeroOption, Vec3,
    },
};

use super::{merchant::MerchantData, particle::Particle};

#[derive(Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct EntityDataVec(pub Vec<EntityDataItem>);

impl Encode for EntityDataVec {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        for item in &self.0 {
            item.index.encode(buf)?;
            item.value.encode(buf)?;
        }
        0xff.encode(buf)?;
        Ok(())
    }
}

impl Decode for EntityDataVec {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let mut vec = Vec::new();
        loop {
            let index = u8::decode(buf)?;
            if index == 0xff {
                break;
            }
            let value = EntityDataValue::decode(buf)?;
            vec.push(EntityDataItem { index, value });
        }
        Ok(Self(vec))
    }
}

#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct EntityDataItem {
    pub index: u8,
    pub value: EntityDataValue,
}

#[derive(Debug, Clone, PartialEq, Transcode)]
pub enum EntityDataValue {
    Byte(u8),
    Int(#[var] i32),
    Long(i64),
    Float(f32),
    String(String),
    FormattedText(FormattedText),
    OptionalFormattedText(Option<FormattedText>),
    ItemStack(ItemSlot),
    Boolean(bool),
    Rotations(f32, f32, f32),
    BlockPos(BlockPos),
    OptionalBlockPos(Option<BlockPos>),
    Direction(Direction),
    OptionalUuid(Option<Uuid>),
    BlockState(#[var] u32),
    OptionalBlockState(#[var] NonZeroOption<u32>),
    CompoundTag(Nbt),
    Particle(Particle),
    VillagerData(MerchantData),
    OptionalUnsignedInt(#[var] NonZeroOption<u32>),
    Pose(Pose),
    CatVariant(#[var] u32),
    FrogVariant(#[var] u32),
    OptionalGlobalPos(Option<GlobalPos>),
    PaintingVariant(#[var] u32),
    SnifferState(SnifferState),
    Vector3(Vec3),
    Quaternion(Quat),
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Transcode)]
pub enum Pose {
    #[default]
    Standing,
    FallFlying,
    Sleeping,
    Swimming,
    SpinAttack,
    Sneaking,
    LongJumping,
    Dying,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Transcode)]
pub enum SnifferState {
    #[default]
    Idling,
    FeelingHappy,
    Scenting,
    Sniffing,
    Searching,
    Digging,
    Rising,
}
