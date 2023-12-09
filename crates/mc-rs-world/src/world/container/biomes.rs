use crate::world::palette::Palette;

use super::traits::ContainerType;

/// A type of [`Container`](crate::world::container::Container) for biome data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BiomeContainer;

impl ContainerType for BiomeContainer {
    fn palette_type(bits: &usize) -> Palette {
        match bits {
            0 => Palette::Single(0u32),
            1..=3 => Palette::Vector(Vec::<u32>::new()),
            _ => Palette::Global,
        }
    }
}
