//! [`FrogNbt`](crate::prelude::FrogNbt) `with`-macro functions for common
//! scenarios

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};

use super::{NbtCompound, NbtTag};
use crate::convert::{FromCompound, FromTag, IntoTag, NbtError};

/// A Nbt mapper that converts between `i8` and `bool`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ByteBool;

impl ByteBool {
    /// Convert from `i8` to `bool`
    #[expect(unreachable_code)]
    pub fn read_from_tag(tag: &i8) -> Result<bool, NbtError> {
        match *tag {
            0i8 => Ok(false),
            1i8 => Ok(true),
            _ => Err(NbtError::ConversionError(
                core::any::type_name::<bool>(),
                todo!("Handle invalid boolean value"),
            )),
        }
    }

    /// Convert from `i8` to `bool`
    pub fn write_as_tag(val: &bool) -> Result<i8, NbtError> {
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
    pub fn read_from_tag(tag: &i8) -> Result<Option<bool>, NbtError> {
        ByteBool::read_from_tag(tag).map(Some)
    }

    /// Convert from `Option<bool>` to `i8`
    #[expect(unreachable_code)]
    pub fn write_as_tag(val: &Option<bool>) -> Result<i8, NbtError> {
        val.as_ref().map_or_else(
            || {
                Err(NbtError::ConversionError(
                    core::any::type_name::<bool>(),
                    todo!("Handle `None` case of an optional bool"),
                ))
            },
            ByteBool::write_as_tag,
        )
    }
}

// -------------------------------------------------------------------------------------------------

/// A Nbt mapper that wraps a type in an option.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WrapOption;

impl WrapOption {
    /// Attempt to convert the type and wrap it inside of an Option.
    pub fn read_from_tag<T: Clone + TryInto<R>, R>(tag: &T) -> Result<Option<R>, NbtError>
    where T::Error: core::error::Error + 'static {
        tag.clone().try_into().map_or_else(
            |err| Err(NbtError::ConversionError(core::any::type_name::<R>(), Box::new(err))),
            |val| Ok(Some(val)),
        )
    }

    /// Attempt to unwrap the type and convert it.
    #[expect(unreachable_code)]
    pub fn write_as_tag<T: Clone + TryInto<R>, R>(tag: &Option<T>) -> Result<R, NbtError>
    where T::Error: core::error::Error + 'static {
        tag.as_ref().map_or_else(
            || {
                Err(NbtError::ConversionError(
                    core::any::type_name::<R>(),
                    todo!("Handle `None` case of an optional value"),
                ))
            },
            |val| {
                val.clone().try_into().map_err(|err| {
                    NbtError::ConversionError(core::any::type_name::<R>(), Box::new(err))
                })
            },
        )
    }
}

// -------------------------------------------------------------------------------------------------

/// A Nbt mapper that converts between `NbtTag` and `Option<R>`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TagOption;

impl TagOption {
    /// Attempt to convert the type and wrap it inside of an `Option`.
    pub fn read_from_tag<R: FromTag>(tag: &NbtTag) -> Result<Option<R>, NbtError> {
        R::from_tag(tag).map(Some)
    }

    /// Attempt to unwrap the type and convert it into a `NbtTag`.
    #[expect(unreachable_code)]
    pub fn write_as_tag<R: IntoTag>(val: &Option<R>) -> Result<NbtTag, NbtError> {
        match val {
            Some(value) => value.into_tag(),
            None => Err(NbtError::ConversionError(core::any::type_name::<R>(), todo!())),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A Nbt mapper that converts between [`Vec`]s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ConvertVec;

impl ConvertVec {
    /// Attempt to convert each element in the Vec.
    pub fn read_from_tag<T: Clone + TryInto<R>, R>(tag: &[T]) -> Result<Vec<R>, NbtError>
    where T::Error: core::error::Error + 'static {
        let mut vec = Vec::with_capacity(tag.len());
        for item in tag {
            vec.push(item.clone().try_into().map_err(|err| {
                NbtError::ConversionError(core::any::type_name::<R>(), Box::new(err))
            })?);
        }
        Ok(vec)
    }
}

// -------------------------------------------------------------------------------------------------

/// A Nbt mapper that reads a list of [`NbtCompound`]s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CompoundVec;

impl CompoundVec {
    /// Attempt to convert each element in the Vec.
    pub fn read_from_tag<R: FromCompound>(tag: &[NbtCompound]) -> Result<Vec<R>, NbtError> {
        let mut vec = Vec::with_capacity(tag.len());
        for compound in tag {
            vec.push(R::from_compound(compound)?);
        }
        Ok(vec)
    }
}
