//! [`FrogNbt`](crate::prelude::FrogNbt) `with`-macro functions for common
//! scenarios

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};

use super::{NbtCompound, NbtTag};
use crate::convert::{ConvertError, FromCompound, FromTag, IntoTag};

/// A Nbt mapper that converts between `i8` and `bool`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ByteBool;

impl ByteBool {
    /// Convert from `i8` to `bool`
    #[expect(unreachable_code)]
    pub fn from_tag(tag: &i8) -> Result<bool, ConvertError> {
        match *tag {
            0i8 => Ok(false),
            1i8 => Ok(true),
            _ => Err(ConvertError::ConversionError(core::any::type_name::<bool>(), todo!())),
        }
    }

    /// Convert from `i8` to `bool`
    pub fn into_tag(val: &bool) -> Result<i8, ConvertError> {
        match *val {
            false => Ok(0i8),
            true => Ok(1i8),
        }
    }
}

/// A Nbt mapper that converts between `i8` and `Option<bool>`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ByteBoolOption;

impl ByteBoolOption {
    /// Convert from `i8` to `Option<bool>`
    #[expect(unreachable_code)]
    pub fn from_tag(tag: &i8) -> Result<Option<bool>, ConvertError> {
        match *tag {
            0i8 => Ok(Some(false)),
            1i8 => Ok(Some(true)),
            _ => Err(ConvertError::ConversionError(core::any::type_name::<bool>(), todo!())),
        }
    }

    /// Convert from `Option<bool>` to `i8`
    pub fn into_tag(val: &Option<bool>) -> Result<i8, ConvertError> {
        match val {
            None | Some(false) => Ok(0i8),
            Some(true) => Ok(1i8),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A Nbt mapper that wraps a type in an option.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WrapOption;

impl WrapOption {
    /// Attempt to convert the type and wrap it inside of an Option.
    pub fn from_tag<T: Clone + TryInto<R>, R>(tag: &T) -> Result<Option<R>, ConvertError>
    where T::Error: core::error::Error + 'static {
        tag.clone().try_into().map_or_else(
            |err| Err(ConvertError::ConversionError(core::any::type_name::<R>(), Box::new(err))),
            |val| Ok(Some(val)),
        )
    }
}

// -------------------------------------------------------------------------------------------------

/// A Nbt mapper that converts between `NbtTag` and `Option<R>`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TagOption;

impl TagOption {
    /// Attempt to convert the type and wrap it inside of an `Option`.
    pub fn from_tag<R: FromTag>(tag: &NbtTag) -> Result<Option<R>, ConvertError> {
        R::from_tag(tag).map(Some)
    }

    /// Attempt to unwrap the type and convert it into a `NbtTag`.
    #[expect(unreachable_code)]
    pub fn into_tag<R: IntoTag>(val: &Option<R>) -> Result<NbtTag, ConvertError> {
        match val {
            Some(value) => value.into_tag(),
            None => Err(ConvertError::ConversionError(core::any::type_name::<R>(), todo!())),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A Nbt mapper that converts between [`Vec`]s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ConvertVec;

impl ConvertVec {
    /// Attempt to convert each element in the Vec.
    pub fn from_tag<T: Clone + TryInto<R>, R>(tag: &[T]) -> Result<Vec<R>, ConvertError>
    where T::Error: core::error::Error + 'static {
        let mut vec = Vec::with_capacity(tag.len());
        for item in tag {
            vec.push(item.clone().try_into().map_err(|err| {
                ConvertError::ConversionError(core::any::type_name::<R>(), Box::new(err))
            })?);
        }
        Ok(vec)
    }
}

// -------------------------------------------------------------------------------------------------

/// A Nbt mapper that reads Vectors of [`NbtCompound`]s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CompoundVec;

impl CompoundVec {
    /// Attempt to convert each element in the Vec.
    pub fn from_tag<R: FromCompound>(tag: &[NbtCompound]) -> Result<Vec<R>, ConvertError> {
        let mut vec = Vec::with_capacity(tag.len());
        for compound in tag {
            vec.push(R::from_compound(compound)?);
        }
        Ok(vec)
    }
}
