use std::fmt::Debug;

use bevy_reflect::Reflect;
use froglight_protocol::{common::ResourceKey, traits::Version};

/// A block
pub trait BlockType: 'static + Debug + Reflect + Send + Sync {
    /// The block's [`ResourceKey`](froglight_protocol::common::ResourceKey).
    ///
    /// This is used to identify the block in the resource pack.
    ///
    /// # Example
    /// ```rust
    /// let air = "minecraft:air";
    /// let dirt = "minecraft:dirt";
    /// let stone = "minecraft:stone";
    /// ```
    #[must_use]
    fn to_key(&self) -> &'static ResourceKey;
    /// The block's language key.
    ///
    /// This is used to identify the block in the language files.
    ///
    /// # Example
    /// ```rust
    /// let air = "block.minecraft.air";
    /// let dirt = "block.minecraft.dirt";
    /// let stone = "block.minecraft.stone";
    /// ```
    #[must_use]
    fn to_lang(&self) -> &'static str;

    /// Returns `true` if the block is air.
    #[must_use]
    fn is_air(&self) -> bool { false }
}

/// An extension trait for [`BlockType`].
pub trait BlockExt<V: Version>: Sized + BlockType {
    /// The total number of states for this block.
    ///
    /// This is `1` by default, and is equal
    /// to the number of states each block attribute
    /// has multiplied together.
    ///
    /// # Example
    /// ```rust
    /// use froglight_block::{
    ///     definitions::{
    ///         attributes::SnowyBooleanAttribute,
    ///         blocks::{Air, GrassBlock, Stone},
    ///     },
    ///     BlockExt,
    /// };
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    ///
    /// // Air has no attributes, so it has `1` block state.
    /// assert_eq!(<Air as BlockExt<V1_21_0>>::BLOCK_STATES, 1);
    ///
    /// // Stone has no attributes, so it has `1` block state.
    /// assert_eq!(<Stone as BlockExt<V1_21_0>>::BLOCK_STATES, 1);
    ///
    /// // Grass has one attribute, `SnowyBooleanAttribute`.
    /// // `SnowyBooleanAttribute` has `2` states, `true` and `false`.
    /// assert_eq!(<GrassBlock as BlockExt<V1_21_0>>::BLOCK_STATES, 2);
    /// ```
    const BLOCK_STATES: u32 = 1u32;

    /// The block's default state.
    #[must_use]
    fn default_state() -> Self;

    /// Get a block from it's *relative* `block state id`.
    ///
    /// This is equivalent to subtracting the first valid `block state id`
    /// for this block from it's actual `block state id`.
    ///
    /// # Example
    /// ```rust
    /// use froglight_block::{
    ///     definitions::{
    ///         attributes::{SnowyBooleanAttribute, WaterloggedBooleanAttribute},
    ///         blocks::{GrassBlock, HeavyCore},
    ///     },
    ///     BlockExt, BlockStorage,
    /// };
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    ///
    /// let storage = BlockStorage::<V1_21_0>::new();
    ///
    /// // GrassBlock has two valid states
    /// let grass_default = <GrassBlock as BlockExt<V1_21_0>>::default_state();
    /// let grass_range = storage.blockstate_range(&grass_default).unwrap();
    /// assert_eq!(grass_range, &(8u32..10u32));
    ///
    /// // The first variant of grass has `SnowyBooleanAttribute(true)`.
    /// let grass_snowy = <GrassBlock as BlockExt<V1_21_0>>::from_relative_id(0).unwrap();
    /// assert_eq!(grass_snowy, GrassBlock { snowy: SnowyBooleanAttribute(true) });
    ///
    /// // The second variant of grass has `SnowyBooleanAttribute(false)`.
    /// let grass_normal = <GrassBlock as BlockExt<V1_21_0>>::from_relative_id(1).unwrap();
    /// assert_eq!(grass_normal, GrassBlock { snowy: SnowyBooleanAttribute(false) });
    ///
    /// // HeavyCore has two valid states
    /// let heavy_core_default = <HeavyCore as BlockExt<V1_21_0>>::default_state();
    /// let heavy_core_range = storage.blockstate_range(&heavy_core_default).unwrap();
    /// assert_eq!(heavy_core_range, &(26682u32..26684u32));
    ///
    /// // The first variant of heavy core has `WaterloggedBooleanAttribute(true)`.
    /// let heavy_core_wet = <HeavyCore as BlockExt<V1_21_0>>::from_relative_id(0).unwrap();
    /// assert_eq!(heavy_core_wet, HeavyCore { waterlogged: WaterloggedBooleanAttribute(true) });
    ///
    /// // The second variant of heavy core has `WaterloggedBooleanAttribute(false)`.
    /// let heavy_core_dry = <HeavyCore as BlockExt<V1_21_0>>::from_relative_id(1).unwrap();
    /// assert_eq!(heavy_core_dry, HeavyCore { waterlogged: WaterloggedBooleanAttribute(false) });
    /// ```
    #[must_use]
    fn from_relative_id(relative_id: u32) -> Option<Self> {
        if relative_id == 0 {
            Some(Self::default_state())
        } else {
            None
        }
    }

    /// Get the block's *relative* `block state id`.
    ///
    /// This is equivalent to subtracting the first valid `block state id`
    /// for this block from it's actual `block state id`.
    ///
    /// # Example
    /// ```rust
    /// use froglight_block::{
    ///     definitions::{attributes::SnowyBooleanAttribute, blocks::GrassBlock},
    ///     BlockExt, BlockStorage,
    /// };
    /// use froglight_protocol::versions::v1_21_0::V1_21_0;
    ///
    /// let storage = BlockStorage::<V1_21_0>::new();
    ///
    /// // The first variant of grass has `SnowyBooleanAttribute(true)`.
    /// let grass_snowy = GrassBlock { snowy: SnowyBooleanAttribute(true) }.to_relative_id();
    /// assert_eq!(grass_snowy, 0);
    ///
    /// // The second variant of grass has `SnowyBooleanAttribute(false)`.
    /// let grass_normal = GrassBlock { snowy: SnowyBooleanAttribute(false) }.to_relative_id();
    /// assert_eq!(grass_normal, 1);
    /// ```
    #[must_use]
    fn to_relative_id(&self) -> u32 { 0 }
}
