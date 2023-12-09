use crate::world::palette::Palette;

use super::traits::ContainerType;

/// A type of [`Container`](crate::world::container::Container) for block data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockContainer;

impl ContainerType for BlockContainer {
    fn palette_type(bits: &usize) -> Palette {
        match bits {
            0 => Palette::Single(0u32),
            1..=8 => Palette::Vector(Vec::<u32>::new()),
            _ => Palette::Global,
        }
    }
}
