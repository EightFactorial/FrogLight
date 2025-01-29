use std::any::TypeId;

use downcast_rs::DowncastSync;
use froglight_common::{Identifier, Version};

pub trait StaticBlock: Send + Sync {
    fn as_static() -> &'static Self;
}

#[cfg(feature = "bevy")]
pub trait BlockType<V: Version>: bevy_reflect::Reflect + DowncastSync {
    fn identifier(&self) -> &'static Identifier;
}

#[cfg(not(feature = "bevy"))]
pub trait BlockType<V: Version>: DowncastSync {
    fn identifier(&self) -> &'static Identifier;
}

pub trait BlockTypeExt<V: Version>: BlockType<V> + StaticBlock + Sized + 'static {
    type Attributes: BlockAttributes;
}

pub trait BlockAttribute: Copy + PartialEq + Into<usize> + Sized + 'static {
    /// All possible states this attribute can be in, in ascending order.
    const STATES: &'static [Self];
}

pub trait BlockAttributes: Sized {
    /// The types of all block attributes.
    const TYPES: &'static [TypeId];
    /// The total number of block states.
    const COUNT: usize;

    fn from_index(index: usize) -> Self;
    fn to_index(&self) -> usize;
}

impl BlockAttributes for () {
    const TYPES: &'static [TypeId] = &[];
    const COUNT: usize = 1;

    fn from_index(index: usize) -> Self {
        match index {
            0 => (),
            _ => panic!("Invalid BlockAttributes index!"),
        }
    }
    fn to_index(&self) -> usize { 0 }
}

impl<A: BlockAttribute> BlockAttributes for A {
    const TYPES: &'static [TypeId] = &[TypeId::of::<A>()];
    const COUNT: usize = A::STATES.len();
    fn from_index(index: usize) -> Self { A::STATES[index] }
    fn to_index(&self) -> usize { Into::<usize>::into(*self) }
}
impl<A: BlockAttribute> BlockAttributes for (A,) {
    const TYPES: &'static [TypeId] = &[TypeId::of::<A>()];
    const COUNT: usize = A::STATES.len();
    fn from_index(index: usize) -> Self { (A::STATES[index],) }
    fn to_index(&self) -> usize { Into::<usize>::into(self.0) }
}
