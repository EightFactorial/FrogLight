use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::Version;

use super::BlockType;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(no_field_bounds, PartialEq))]
pub struct Block<B: BlockType<V>, V: Version>(
    pub(crate) RelativeBlockState,
    #[cfg_attr(feature = "bevy", reflect(ignore))] pub(crate) PhantomData<(B, V)>,
);

#[derive(Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(PartialEq))]
pub struct UntypedBlock<V: Version>(
    pub(crate) RelativeBlockState,
    pub(crate) &'static dyn BlockType<V>,
);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq, Hash))]
pub struct GlobalBlockState(pub(crate) u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
pub struct RelativeBlockState(pub(crate) u16);

impl<B: BlockType<V> + 'static, V: Version> PartialEq for Block<B, V> {
    fn eq(&self, other: &Self) -> bool {
        (TypeId::of::<Self>() == other.type_id()) && (self.0 == other.0)
    }
}
impl<B: BlockType<V> + 'static, V: Version> Eq for Block<B, V> {}

impl<V: Version> PartialEq for UntypedBlock<V> {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self.1, other.1) && (self.0 == other.0) }
}
impl<V: Version> Eq for UntypedBlock<V> {}
