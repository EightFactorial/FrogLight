use compact_str::CompactString;
use mc_rs_protocol::Version;

/// A trait for blocks.
///
/// Because blocks can change between versions, this trait is used to represent
/// blocks generically.
///
/// # Example
/// ```rust
/// use mc_rs_protocol::versions::v1_20_0::V1_20_0;
/// use mc_rs_world::blocks::{structs::*, traits::BlockTrait};
///
/// let air = BlockAir::default();
/// assert_eq!(BlockTrait::<V1_20_0>::resource_location(&air), "minecraft:air");
/// assert_eq!(BlockTrait::<V1_20_0>::to_u32(&air), 0);
///
/// let stone = BlockStone::default();
/// assert_eq!(BlockTrait::<V1_20_0>::resource_location(&stone), "minecraft:stone");
/// assert_eq!(BlockTrait::<V1_20_0>::to_u32(&stone), 1);
/// ```
pub trait BlockTrait<V: Version>: std::fmt::Debug + Default + Clone + Copy {
    /// Get the resource location of the block.
    fn resource_location(&self) -> &'static str;
    /// Get the block from the state id.
    fn try_from_u32(id: u32) -> Option<Self>;
    /// Get the state id of the block.
    fn to_u32(&self) -> u32;

    /// Get the language key of the block.
    fn lang_key(&self) -> CompactString {
        let suffix = self.resource_location().split(':').last().unwrap();
        CompactString::from(format!("block.minecraft.{suffix}"))
    }
}

pub trait BlocksTrait<V: Version>: std::fmt::Debug + Default + Clone + Copy {
    fn from_u32(id: u32) -> Self;
    fn to_u32(&self) -> u32;
}
