use bevy_derive::{Deref, DerefMut};
use compact_str::CompactString;
use froglight_protocol::common::ResourceKey;

use crate::{
    definitions::{BlockAttribute, BlockExt, BlockType},
    tests::TestVersion,
};

/// Blocks for testing the block registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestBlocks {
    Air(AirBlock),
    Stone(StoneBlock),
    Grass(GrassBlock),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AirBlock;

impl BlockType<TestVersion> for AirBlock {
    fn to_key(&self) -> ResourceKey { ResourceKey::new_inline("minecraft:air") }
    fn to_lang(&self) -> CompactString { CompactString::from("block.minecraft.air") }

    fn is_air(&self) -> bool { true }
    fn is_opaque(&self) -> bool { false }
    fn is_collidable(&self) -> bool { false }
}
impl BlockExt<TestVersion> for AirBlock {
    fn default_state() -> Self { AirBlock }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoneBlock;

impl BlockType<TestVersion> for StoneBlock {
    fn to_key(&self) -> ResourceKey { ResourceKey::new_inline("minecraft:stone") }
    fn to_lang(&self) -> CompactString { CompactString::from("block.minecraft.stone") }
}
impl BlockExt<TestVersion> for StoneBlock {
    fn default_state() -> Self { StoneBlock }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GrassBlock {
    pub grassy: GrassyAttribute,
}

impl BlockType<TestVersion> for GrassBlock {
    fn to_key(&self) -> ResourceKey { ResourceKey::new_inline("minecraft:grass") }
    fn to_lang(&self) -> CompactString { CompactString::from("block.minecraft.grass") }
}
impl BlockExt<TestVersion> for GrassBlock {
    const BLOCK_STATES: u32 = GrassyAttribute::STATES;
    fn default_state() -> Self { GrassBlock { grassy: GrassyAttribute(false) } }
    fn from_relative_id(id: u32) -> Option<Self> {
        match id {
            0 => Some(GrassBlock { grassy: GrassyAttribute(false) }),
            1 => Some(GrassBlock { grassy: GrassyAttribute(true) }),
            _ => None,
        }
    }
    fn to_relative_id(&self) -> u32 { u32::from(self.grassy.0) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct GrassyAttribute(pub bool);

impl BlockAttribute<TestVersion> for GrassyAttribute {
    const STATES: u32 = 2u32;
}
