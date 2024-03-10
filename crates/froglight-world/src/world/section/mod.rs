mod palette;
pub use palette::Palette;

#[allow(clippy::module_inception)]
mod section;
pub use section::Section;

mod iterator;
pub use iterator::SectionIdIterator;
