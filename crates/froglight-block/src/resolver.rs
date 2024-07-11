use std::fmt::Debug;

use froglight_protocol::traits::Version;

use crate::BlockStorage;

/// A block state resolver for a specific [`Version`].
///
/// # Example
/// ```rust
/// use bevy_reflect::Reflect;
/// use froglight_block::{BlockExt, BlockStateResolver, BlockStorage, BlockType};
/// use froglight_protocol::{common::ResourceKey, versions::v1_21_0::V1_21_0};
///
/// /// A custom block type.
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
/// struct MyBlock;
///
/// impl MyBlock {
///     pub const BLOCK_KEY: &'static ResourceKey = &ResourceKey::const_new("froglight:my_block");
/// }
///
/// impl BlockType for MyBlock {
///     fn to_key(&self) -> &'static ResourceKey { Self::BLOCK_KEY }
///     fn to_lang(&self) -> &'static str { "block.froglight.my_block" }
/// }
///
/// impl BlockExt<V1_21_0> for MyBlock {
///     fn default_state() -> Self { MyBlock }
/// }
///
/// /// A custom block state resolver.
/// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
/// struct MyResolver;
///
/// impl BlockStateResolver<V1_21_0> for MyResolver {
///     /// We only care about `MyBlock`.
///     type Resolved = Option<MyBlock>;
///
///     /// We only check for `MyBlock`, return `None` for all other blocks.
///     fn resolve_state(blockstate_id: u32, storage: &BlockStorage<V1_21_0>) -> Self::Resolved {
///         let default_dyn = storage.default_blockstate(blockstate_id).unwrap();
///         default_dyn.as_any().downcast_ref::<MyBlock>().cloned()
///     }
///
///     /// Register `MyBlock` with the storage.
///     fn register_blocks(storage: &mut BlockStorage<V1_21_0>) { storage.register::<MyBlock>(); }
/// }
/// ```
pub trait BlockStateResolver<V>
where
    Self: 'static + Debug + Send + Sync,
    V: Version,
{
    /// The type of block being resolved.
    type Resolved;

    /// Resolve a [`Self::Resolved`] from it's `block state id`.
    #[must_use]
    fn resolve_state(blockstate_id: u32, storage: &BlockStorage<V>) -> Self::Resolved;

    /// Register all blocks for this resolver.
    ///
    /// This should call [`BlockStorage::register`] for each block type.
    fn register_blocks(storage: &mut BlockStorage<V>);
}

/// [`BlockStateResolver`](super::BlockStateResolver) for vanilla blocks.
///
/// To be used with
/// [`BlockStorage::resolve_blockstate`](super::BlockStorage::resolve_blockstate).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VanillaResolver;
