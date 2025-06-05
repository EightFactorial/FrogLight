//! TODO: Check byte ordering and create an error type for length mismatches.

#[cfg(not(feature = "std"))]
use alloc::{borrow::Cow, boxed::Box};
use core::any::type_name;
#[cfg(feature = "std")]
use std::borrow::Cow;

use froglight_common::entity::EntityUuid;
use uuid::Uuid;

use crate::prelude::*;

impl FromTag for EntityUuid {
    #[inline]
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        Uuid::from_tag(tag).map(EntityUuid::from)
    }
}

impl IntoTag for EntityUuid {
    #[inline]
    fn into_tag(&self) -> Result<NbtTag, NbtError> { Uuid::into_tag(self.as_ref()) }
}

// -------------------------------------------------------------------------------------------------

impl FromTag for Uuid {
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            NbtTag::ByteArray(array) | NbtTag::List(NbtListTag::Byte(array)) => {
                if array.len() == 16 {
                    #[expect(clippy::cast_sign_loss)]
                    let bytes = core::array::from_fn::<u8, 16, _>(|i| array[i] as u8);
                    Ok(Uuid::from_bytes(bytes))
                } else {
                    #[expect(unreachable_code)]
                    Err(NbtError::ConversionError(
                        type_name::<Self>(),
                        todo!("Create an error type"),
                    ))
                }
            }
            NbtTag::IntArray(array) | NbtTag::List(NbtListTag::Int(array)) => {
                if array.len() == 4 {
                    let bytes = core::array::from_fn::<[u8; 4], 4, _>(|i| array[i].to_le_bytes());
                    Ok(Uuid::from_slice_le(bytes.as_flattened()).unwrap())
                } else {
                    #[expect(unreachable_code)]
                    Err(NbtError::ConversionError(
                        type_name::<Self>(),
                        todo!("Create an error type"),
                    ))
                }
            }
            NbtTag::String(string) => {
                let string = string.try_as_str().map_err(|err| {
                    NbtError::ConversionError(type_name::<Cow<'_, str>>(), Box::new(err))
                })?;
                Uuid::parse_str(&string).map_err(|_err| {
                    #[expect(unreachable_code)]
                    NbtError::ConversionError(type_name::<Self>(), todo!("Create an error type"))
                })
            }
            _ => Err(NbtError::MismatchedTag(
                type_name::<Self>(),
                "ByteArray, IntArray, List, or String",
            )),
        }
    }
}

impl IntoTag for Uuid {
    fn into_tag(&self) -> Result<NbtTag, NbtError> { todo!() }
}
