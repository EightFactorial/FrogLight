//! TODO

#[cfg(feature = "std")]
type CompoundMap = indexmap::IndexMap<crate::mutf8::Mutf8String, NbtTag>;
#[cfg(not(feature = "std"))]
type CompoundMap =
    indexmap::IndexMap<crate::mutf8::Mutf8String, NbtTag, bevy_platform::hash::FixedState>;

mod array;
pub use array::{ByteArray, DoubleArray, FloatArray, IntArray, LongArray, ShortArray};

mod compound;
pub use compound::NbtCompound;

pub mod mappings;

mod named;
pub use named::{NamedNbt, UnnamedNbt};

mod tag;
pub use tag::{NbtListTag, NbtTag};
