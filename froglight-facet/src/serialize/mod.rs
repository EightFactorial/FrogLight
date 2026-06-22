//! TODO

use alloc::vec::Vec;

use facet::{Facet, Peek};
use froglight_facet_iter::{
    Writer, WriterError,
    serialize::{Item, SerializeError, Serializer},
};

pub mod functions;
pub mod future;
pub mod varint;

/// A trait for types that can be serialized.
pub trait Serialize<'facet> {
    /// Serialize the value into a new [`Vec`].
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn to_vec(value: &Self) -> Result<Vec<u8>, SerializeError>;

    /// Serialize the value into the given [`Writer`],
    /// returning the number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn to_writer(value: &Self, variable: bool, writer: Writer<'_>)
    -> Result<usize, SerializeError>;
}

impl<'facet, T: Facet<'facet>> Serialize<'facet> for T {
    #[inline]
    fn to_vec(value: &Self) -> Result<Vec<u8>, SerializeError> {
        let mut buffer = Vec::with_capacity(8); // TODO: Size hint
        <Self as Serialize>::to_writer(value, false, Writer::new(&mut buffer)).map(|_| buffer)
    }

    #[inline]
    fn to_writer(
        value: &Self,
        variable: bool,
        writer: Writer<'_>,
    ) -> Result<usize, SerializeError> {
        serialize(Peek::new(value), variable, writer)
    }
}

// -------------------------------------------------------------------------------------------------

#[inline(never)]
fn serialize(
    peek: Peek<'_, '_>,
    variable: bool,
    mut writer: Writer<'_>,
) -> Result<usize, SerializeError> {
    // Create and complete the serializer.
    let mut core = serialize_core(&mut writer);
    Serializer::new(peek, variable, &mut core, Some("mc")).complete()?;

    // Return the number of bytes written.
    drop(core);
    Ok(writer.position())
}

// -------------------------------------------------------------------------------------------------

/// The core logic behind [`serialize`], separated out for readability.
#[doc(hidden)]
#[inline(always)]
#[allow(clippy::inline_always, reason = "Performance")]
pub fn serialize_core<'mem, 'facet>(
    writer: &mut Writer<'_>,
) -> impl FnMut(Item<'mem, 'facet>) -> Result<(), WriterError> {
    |item: Item<'mem, 'facet>| -> Result<(), WriterError> {
        let item = match item {
            Item::Item(item) => item,
            Item::Size(size) => return varint::encode_u32_into(size, writer),
        };

        // Handle field attributes.
        if let Some(attrs) = item.field_attr() {
            for attr in attrs {
                // Run the custom serializer.
                if attr.ns.is_some_and(|ns| ns == "mc")
                    && attr.key == "with"
                    && let Some(crate::facet::Attr::With(Some(with))) =
                        attr.get_as::<crate::facet::Attr>()
                {
                    return with.serialize(item, writer);
                }
            }
        }

        // Handle type attributes.
        for attr in item.shape_attr() {
            // Run the custom serializer.
            if attr.ns.is_some_and(|ns| ns == "mc")
                && attr.key == "with"
                && let Some(crate::facet::Attr::With(Some(with))) =
                    attr.get_as::<crate::facet::Attr>()
            {
                return with.serialize(item, writer);
            }
        }

        // Serialize the item.
        serialize_value(item.peek(), item.is_variable(), writer)
    }
}

/// The value logic behind [`serialize`], separated out for readability.
#[inline(always)]
#[allow(clippy::inline_always, reason = "Performance")]
fn serialize_value(
    peek: &Peek<'_, '_>,
    variable: bool,
    writer: &mut Writer<'_>,
) -> Result<(), WriterError> {
    macro_rules! handle {
        ($var:expr, $peek:expr => $($ty:ty: $fn:ident),*) => {
            $(
                if let Ok(value) = $peek.get::<$ty>() {
                    return if $var {
                        varint::$fn(*value, writer)
                    } else {
                        writer.write_bytes(&value.to_be_bytes())
                    }
                }
            )*
        };
        (@cast $var:expr, $peek:expr => $($ty:ty, $cast:ty => $fn:ident),*) => {
            $(
                if let Ok(value) = $peek.get::<$ty>() {
                    #[expect(clippy::cast_sign_loss, reason = "Desired behavior")]
                    let cast = *value as $cast;
                    return if $var {
                        varint::$fn(cast, writer)
                    } else {
                        writer.write_bytes(&cast.to_be_bytes())
                    }
                }
            )*
        };
    }

    // Handle units
    if peek.shape().is_type::<()>() {
        return Ok(());
    }

    // Handle booleans
    if let Ok(bool) = peek.get::<bool>() {
        return writer.write_byte(u8::from(*bool));
    }

    // Handle integer types
    handle!(variable, peek => u8: encode_u8_into, u16: encode_u16_into, u32: encode_u32_into, u64: encode_u64_into, u128: encode_u128_into);
    handle!(@cast variable, peek => i8, u8 => encode_u8_into, i16, u16 => encode_u16_into, i32, u32 => encode_u32_into, i64, u64 => encode_u64_into, i128, u128 => encode_u128_into);

    // Handle floating-point types
    if let Ok(f32) = peek.get::<f32>() {
        return writer.write_bytes(&f32.to_le_bytes());
    } else if let Ok(f64) = peek.get::<f64>() {
        return writer.write_bytes(&f64.to_le_bytes());
    }

    // Handle strings
    if let Some(string) = peek.as_str() {
        let (bytes, len) = varint::encode_u32(string.len().try_into().map_err(WriterError::other)?);
        writer.write_bytes(&bytes[..len as usize])?;
        return writer.write_bytes(string.as_bytes());
    }

    todo!("Unsupported type: `{}`", peek.shape().type_name());
}
