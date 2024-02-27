mod heightmap_container;
use froglight_core::data::SectionBlockPosition;
pub use heightmap_container::*;

mod chunk_container;
pub use chunk_container::*;

impl<T: ContainerType> ChunkDataContainer<T> {
    /// Gets the value at the given coordinates.
    #[must_use]
    pub const fn get(&self, _pos: &SectionBlockPosition) -> usize { todo!() }

    /// Sets the value at the given coordinates.
    ///
    /// Returns the previous value at the given coordinates.
    pub fn set(&mut self, _pos: &SectionBlockPosition, _value: usize) -> usize { todo!() }
}
