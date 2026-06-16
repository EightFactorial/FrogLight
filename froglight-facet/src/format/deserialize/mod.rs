//! TODO

use alloc::{
    borrow::Cow::{self, Borrowed},
    string::String,
};

use facet::{Facet, HeapValue, Partial};

mod error;
pub use error::DeserializeError;

pub mod functions;

pub(crate) mod iterator;
pub use iterator::{DeserializeItem, IteratorStack};

pub(crate) mod logic;
pub use logic::{Deserializer, Item};

pub mod varint;

use crate::format::{ReaderError, reader::Reader};

/// A trait for types that can be deserialized.
pub trait Deserialize<'facet>: Facet<'facet> + Sized {
    /// Deserialize a value from the given byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the deserialization fails.
    #[inline]
    fn from_slice(slice: &[u8], variable: bool) -> Result<Self, DeserializeError>
    where
        'facet: 'static,
        'static: 'facet,
    {
        <Self as Deserialize>::from_slice_remainder(slice, variable).map(|(val, _)| val)
    }

    /// Deserialize a value from the given byte slice,
    /// returning the remaining slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the deserialization fails.
    fn from_slice_remainder(
        slice: &[u8],
        variable: bool,
    ) -> Result<(Self, &[u8]), DeserializeError>
    where
        'facet: 'static,
        'static: 'facet;

    /// Deserialize a value from the given byte slice,
    /// returning the remaining slice.
    ///
    /// Borrows from the input slice where possible.
    ///
    /// # Errors
    ///
    /// Returns an error if the deserialization fails.
    fn from_slice_borrowed(
        slice: &'facet [u8],
        variable: bool,
    ) -> Result<(Self, &'facet [u8]), DeserializeError>
    where
        Self: Facet<'facet>;
}

impl<'facet, T: Facet<'facet> + Sized> Deserialize<'facet> for T {
    #[inline]
    fn from_slice_remainder(slice: &[u8], variable: bool) -> Result<(Self, &[u8]), DeserializeError>
    where
        'facet: 'static,
        'static: 'facet,
    {
        let mut cursor = Reader::new(slice);
        let value = deserialize_owned(Partial::alloc_owned::<T>()?, variable, &mut cursor)?;
        Ok((value.materialize::<T>()?, cursor.remaining()))
    }

    #[inline]
    fn from_slice_borrowed(
        slice: &'facet [u8],
        variable: bool,
    ) -> Result<(Self, &'facet [u8]), DeserializeError>
    where
        Self: Facet<'facet>,
    {
        let mut cursor = Reader::new(slice);
        let value = deserialize_borrowed(Partial::alloc::<T>()?, variable, &mut cursor)?;
        Ok((value.materialize::<T>()?, cursor.remaining()))
    }
}

// -------------------------------------------------------------------------------------------------

#[inline(never)]
fn deserialize_owned(
    partial: Partial<'static, false>,
    variable: bool,
    reader: &mut Reader<'_>,
) -> Result<HeapValue<'static, false>, DeserializeError> {
    let mut core = move |item: Item<'static, false>| {
        let item = match item {
            Item::Item(item) => item,
            Item::Size(..) => return varint::decode_u32_from(reader).map(Item::Size),
        };

        // Handle field attributes.
        if let Some(attrs) = item.field_attr() {
            for attr in attrs {
                // Run the custom deserializer.
                if attr.ns.is_some_and(|ns| ns == "mc")
                    && attr.key == "with"
                    && let Some(crate::facet::Attr::With(Some(with))) =
                        attr.get_as::<crate::facet::Attr>()
                {
                    return with.deserialize(item, reader).map(Item::Item);
                }
            }
        }

        // Handle type attributes.
        for attr in item.shape_attr() {
            // Run the custom deserializer.
            if attr.ns.is_some_and(|ns| ns == "mc")
                && attr.key == "with"
                && let Some(crate::facet::Attr::With(Some(with))) =
                    attr.get_as::<crate::facet::Attr>()
            {
                return with.deserialize(item, reader).map(Item::Item);
            }
        }

        deserialize_core(item, reader).map(Item::Item)
    };

    let mut de = Deserializer::new(partial, variable, &mut core);
    while let Some(result) = Iterator::next(&mut de) {
        result?;
    }
    de.into_partial()?.build().map_err(DeserializeError::from)
}

#[inline(never)]
fn deserialize_borrowed<'facet>(
    partial: Partial<'facet, true>,
    variable: bool,
    reader: &mut Reader<'facet>,
) -> Result<HeapValue<'facet, true>, DeserializeError> {
    let mut core = |item: Item<'facet, true>| {
        let item = match item {
            Item::Item(item) => item,
            Item::Size(..) => return varint::decode_u32_from(reader).map(Item::Size),
        };

        // Handle field attributes.
        if let Some(attrs) = item.field_attr() {
            for attr in attrs {
                // Run the custom deserializer.
                if attr.ns.is_some_and(|ns| ns == "mc")
                    && attr.key == "with"
                    && let Some(crate::facet::Attr::With(Some(with))) =
                        attr.get_as::<crate::facet::Attr>()
                {
                    return with.deserialize_borrowed(item, reader).map(Item::Item);
                }
            }
        }

        // Handle type attributes.
        for attr in item.shape_attr() {
            // Run the custom deserializer.
            if attr.ns.is_some_and(|ns| ns == "mc")
                && attr.key == "with"
                && let Some(crate::facet::Attr::With(Some(with))) =
                    attr.get_as::<crate::facet::Attr>()
            {
                return with.deserialize_borrowed(item, reader).map(Item::Item);
            }
        }

        // Handle borrowed strings.
        if item.is_type::<String>() || item.is_type::<Cow<'_, str>>() || item.is_type::<&str>() {
            let length = varint::decode_u32_from(reader)? as usize;
            let bytes = reader.get(length)?;

            #[cfg(feature = "simd")]
            let str: &str = simdutf8::compat::from_utf8(bytes).map_err(ReaderError::other)?;
            #[cfg(not(feature = "simd"))]
            let str: &str = core::str::from_utf8(bytes).map_err(ReaderError::other)?;

            return if item.is_type::<&str>() {
                item.set(str).map(Item::Item)
            } else if item.is_type::<Cow<'_, str>>() {
                item.set(Borrowed(str)).map(Item::Item)
            } else {
                item.set(String::from(str)).map(Item::Item)
            };
        }

        deserialize_core(item, reader).map(Item::Item)
    };

    let mut de = Deserializer::new(partial, variable, &mut core);
    while let Some(result) = Iterator::next(&mut de) {
        result?;
    }
    de.into_partial()?.build().map_err(DeserializeError::from)
}

// -------------------------------------------------------------------------------------------------

/// The deserializer logic behind [`deserialize_owned`] and
/// [`deserialize_borrowed`], separated out for readability.
#[inline(always)]
#[allow(clippy::inline_always, reason = "Performance")]
fn deserialize_core<'facet, const BORROW: bool>(
    item: DeserializeItem<'facet, BORROW>,
    reader: &mut Reader<'_>,
) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
    macro_rules! handle {
        ($item:expr => $($ty:ty: $fn:ident),*) => {
            $(
                if $item.is_type::<$ty>() {
                    return if $item.is_variable() {
                        item.set::<$ty>(varint::$fn(reader)?)
                    } else {
                        item.set::<$ty>(<$ty>::from_be_bytes(*reader.get_array()?))
                    }
                }
            )*
        };
        (@cast $item:expr => $($ty:ty, $cast:ty => $fn:ident),*) => {
            $(
                if $item.is_type::<$ty>() {
                    return if $item.is_variable() {
                        #[expect(clippy::cast_possible_wrap, reason = "Desired behavior")]
                        item.set::<$ty>(varint::$fn(reader)? as $ty)
                    } else {
                        item.set::<$ty>(<$ty>::from_be_bytes(*reader.get_array()?))
                    }
                }
            )*
        };
    }

    // Handle units.
    if item.is_type::<()>() {
        return item.set(());
    }

    // Handle booleans.
    if item.is_type::<bool>() {
        return match reader.get_array::<1>()? {
            [0] => item.set(false),
            [1] => item.set(true),
            [unk] => Err(ReaderError::InvalidBool(*unk))?,
        };
    }

    // Handle integer types.
    handle!(item => u8: decode_u8_from, u16: decode_u16_from, u32: decode_u32_from, u64: decode_u64_from, u128: decode_u128_from);
    handle!(@cast item => i8, u8 => decode_u8_from, i16, u16 => decode_u16_from, i32, u32 => decode_u32_from, i64, u64 => decode_u64_from, i128, u128 => decode_u128_from);

    // Handle floating-point types.
    if item.is_type::<f32>() {
        return item.set(f32::from_be_bytes(*reader.get_array()?));
    } else if item.is_type::<f64>() {
        return item.set(f64::from_be_bytes(*reader.get_array()?));
    }

    // Handle strings.
    if item.is_type::<String>() || item.is_type::<Cow<'_, str>>() {
        let length = varint::decode_u32_from(reader)? as usize;
        let bytes = reader.get(length)?;

        #[cfg(feature = "simd")]
        let str: &str = simdutf8::compat::from_utf8(bytes).map_err(ReaderError::other)?;
        #[cfg(not(feature = "simd"))]
        let str: &str = core::str::from_utf8(bytes).map_err(ReaderError::other)?;

        return if item.is_type::<String>() {
            item.set(String::from(str))
        } else {
            item.set(Cow::<'_, str>::Owned(String::from(str)))
        };
    }

    todo!("Unsupported type: `{}`", item.shape().type_name());
}
