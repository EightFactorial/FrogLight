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
/// use mc_rs_world::blocks::{structs::*, traits::Block};
///
/// let air = BlockAir::default();
/// assert_eq!(Block::<V1_20_0>::resource_location(&air), "minecraft:air");
/// assert_eq!(Block::<V1_20_0>::state_id(&air), 0);
///
/// let stone = BlockStone::default();
/// assert_eq!(Block::<V1_20_0>::resource_location(&stone), "minecraft:stone");
/// assert_eq!(Block::<V1_20_0>::state_id(&stone), 1);
/// ```
pub trait Block<V: Version>: std::fmt::Debug + Default + Clone + Copy {
    /// Get the resource location of the block.
    fn resource_location(&self) -> &'static str;
    /// Get the state id of the block.
    fn state_id(&self) -> u32;

    /// Get the language key of the block.
    fn lang_key(&self) -> CompactString {
        let suffix = self.resource_location().split(':').last().unwrap();
        CompactString::from(format!("block.minecraft.{suffix}"))
    }
}
