use crate::world::palette::Palette;

use super::traits::ContainerType;

/// A container for biome data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BiomeContainer;

impl ContainerType for BiomeContainer {
    fn palette_type(bits: &u8) -> Palette {
        match bits {
            0 => Palette::Single(0u32),
            1..=3 => Palette::Vector(Vec::<u32>::new()),
            _ => Palette::Global,
        }
    }
}
