use crate::world::palette::Palette;

use super::traits::ContainerType;

/// A container for block data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockContainer;

impl ContainerType for BlockContainer {
    fn palette_type(bits: &u8) -> Palette {
        match bits {
            0 => Palette::Single(0u32),
            1..=8 => Palette::Vector(Vec::<u32>::new()),
            _ => Palette::Global,
        }
    }
}
