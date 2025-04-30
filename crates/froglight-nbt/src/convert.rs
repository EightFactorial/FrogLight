//! TODO

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, string::String, vec::Vec};

use crate::nbt::{NbtCompound, NbtTag, UnnamedNbt};

/// A trait for parsing types from [`NbtCompound`]s.
pub trait FromCompound: Sized {
    /// Parse the type from an [`NbtCompound`].
    ///
    /// # Errors
    /// Returns an error if the type fails to parse.
    fn from_compound(compound: &NbtCompound) -> Result<Self, ConvertError>;

    /// Parse the type from an [`UnnamedNbt`].
    ///
    /// Returns `None` if the [`UnnamedNbt`] is empty.
    ///
    /// # Errors
    /// Returns an error if the type fails to parse.
    fn from_nbt(nbt: &UnnamedNbt) -> Result<Option<Self>, ConvertError> {
        match nbt.compound() {
            Some(compound) => Self::from_compound(compound).map(Some),
            None => Ok(None),
        }
    }
}

/// A trait for converting types into [`NbtCompound`]s.
pub trait IntoCompound {
    /// Convert the type into an [`NbtCompound`].
    ///
    /// # Errors
    /// Returns an error if the type fails to convert.
    #[expect(clippy::wrong_self_convention)]
    fn into_compound(&self) -> Result<NbtCompound, ConvertError>;
}

impl<T: From<NbtCompound>> FromCompound for T {
    #[inline]
    fn from_compound(compound: &NbtCompound) -> Result<Self, ConvertError> {
        Ok(T::from(compound.clone()))
    }
}

impl<T: Clone + Into<NbtCompound>> IntoCompound for T {
    #[inline]
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> { Ok(self.clone().into()) }
}

// -------------------------------------------------------------------------------------------------

/// A trait for parsing types from [`NbtTag`]s.
pub trait FromTag: Sized {
    /// Parse the type from an [`NbtTag`].
    ///
    /// # Errors
    /// Returns an error if the type fails to parse.
    fn from_tag(tag: &NbtTag) -> Result<Self, ConvertError>;
}

/// A trait for converting types into [`NbtTag`]s.
pub trait IntoTag {
    /// Convert the type into an [`NbtTag`].
    ///
    /// # Errors
    /// Returns an error if the type fails to convert.
    #[expect(clippy::wrong_self_convention)]
    fn into_tag(&self) -> Result<NbtTag, ConvertError>;
}

impl<T: FromCompound> FromTag for T {
    fn from_tag(tag: &NbtTag) -> Result<Self, ConvertError> {
        match tag {
            NbtTag::Compound(compound) => T::from_compound(compound),
            _ => Err(ConvertError::MismatchedTag(core::any::type_name::<T>(), "Compound")),
        }
    }
}

impl<T: IntoCompound> IntoTag for T {
    #[inline]
    fn into_tag(&self) -> Result<NbtTag, ConvertError> {
        self.into_compound().map(NbtTag::Compound)
    }
}

// -------------------------------------------------------------------------------------------------

macro_rules! impl_from_into_tag {
    // Implement `FromTag` and `IntoTag` using casts.
    (@cast $(($ty:ident, $inner:ty)),*) => {
        $(
            impl FromTag for $inner {
                #[allow(trivial_numeric_casts, clippy::cast_sign_loss)]
                fn from_tag(tag: &NbtTag) -> Result<Self, ConvertError> {
                    match tag {
                        NbtTag::$ty(value) => Ok(*value as Self),
                        _ => Err(ConvertError::MismatchedTag(core::any::type_name::<Self>(), stringify!($ty))),
                    }
                }
            }

            impl IntoTag for $inner {
                #[allow(trivial_numeric_casts, clippy::cast_possible_wrap)]
                fn into_tag(&self) -> Result<NbtTag, ConvertError> {
                    Ok(NbtTag::$ty(*self as _))
                }
            }
        )*
    };
    // Implement `FromTag` and `IntoTag` using `TryInto::try_into`.
    (@try_into $(($ty:ident, $inner:ty)),*) => {
        $(
            impl FromTag for $inner {
                fn from_tag(tag: &NbtTag) -> Result<Self, ConvertError> {
                    match tag {
                        NbtTag::$ty(value) => value.clone().try_into().map_err(|err| ConvertError::ConversionError(core::any::type_name::<Self>(), Box::new(err))),
                        _ => Err(ConvertError::MismatchedTag(core::any::type_name::<Self>(), stringify!($ty))),
                    }
                }
            }

            impl IntoTag for $inner {
                fn into_tag(&self) -> Result<NbtTag, ConvertError> {
                    Ok(NbtTag::from(self.clone()))
                }
            }
        )*
    };
}

impl_from_into_tag!(@cast (Byte, i8), (Byte, u8), (Short, i16), (Short, u16), (Int, i32), (Int, u32), (Long, i64), (Long, u64), (Float, f32), (Double, f64));
impl_from_into_tag!(@try_into (ByteArray, Vec<i8>), (IntArray, Vec<i32>), (LongArray, Vec<i64>), (String, String));

// -------------------------------------------------------------------------------------------------

/// A trait for inserting types into [`NbtCompound`]s.
pub trait InsertAsNbt: IntoTag {
    /// The key to use when inserting the type into an [`NbtCompound`].
    const KEY: &'static str;

    /// Insert the type into an [`NbtCompound`].
    ///
    /// Returns the previous value associated with the key, if any.
    ///
    /// # Errors
    /// Returns an error if the type fails to convert.
    fn insert_into(&self, compound: &mut NbtCompound) -> Result<Option<NbtTag>, ConvertError> {
        self.into_tag().map(|tag| compound.insert(Self::KEY, tag))
    }

    /// Insert the type into an [`UnnamedNbt`].
    ///
    /// Returns the previous value associated with the key, if any.
    ///
    /// # Errors
    /// Returns an error if the type fails to convert.
    fn insert_into_nbt(&self, nbt: &mut UnnamedNbt) -> Result<Option<NbtTag>, ConvertError> {
        self.into_tag().map(|tag| nbt.insert(Self::KEY, tag))
    }
}

// -------------------------------------------------------------------------------------------------

/// An error that can occur when converting between a type and NBT.
#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    /// A field was missing from the NBT compound.
    #[error("Missing field for `{0}`: \"{1}\"")]
    MissingField(&'static str, &'static str),
    /// A field's tag did not match the expected tag.
    #[error("Mismatched tag for `{0}`, expected \"{1}\"")]
    MismatchedTag(&'static str, &'static str),
    /// An error occurred while converting a field.
    #[error("Failed to create \"{0}\": {1}")]
    ConversionError(&'static str, Box<dyn core::error::Error>),
}
