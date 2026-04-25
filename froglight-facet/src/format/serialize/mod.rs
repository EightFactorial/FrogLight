//! TODO

use alloc::vec::Vec;

use facet::{Facet, Peek};

mod error;
pub use error::SerializeError;

pub mod functions;

pub(crate) mod iterator;
pub use iterator::{IteratorStack, SerializeItem};

pub(crate) mod logic;
pub use logic::{Item, Serializer};

pub mod varint;

use crate::format::writer::{Writer, WriterError};

/// A trait for types that can be deserialized.
#[expect(clippy::missing_errors_doc, missing_docs, reason = "WIP")]
pub trait Serialize<'facet> {
    #[inline]
    fn to_vec(value: &Self) -> Result<Vec<u8>, SerializeError> {
        let mut buffer = Vec::new();
        <Self as Serialize>::to_writer(value, false, Writer::new(&mut buffer)).map(|_| buffer)
    }

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

fn serialize<'mem, 'facet>(
    serialize_peek: Peek<'mem, 'facet>,
    serialize_variable: bool,
    mut writer: Writer<'_>,
) -> Result<usize, SerializeError> {
    let mut core = |item: Item<'mem, 'facet>| -> Result<(), WriterError> {
        let item = match item {
            Item::Item(item) => item,
            Item::Size(size) => {
                let (bytes, len) = varint::encode_u32(size);
                writer.write_bytes(&bytes[..len as usize])?;
                return Ok(());
            }
        };

        // Handle field attributes.
        if let Some(field) = item.field() {
            // Run the custom serializer.
            if let Some(attr) = field.get_attr(Some("mc"), "with")
                && let Some(crate::facet::Attr::With(Some(with))) =
                    attr.get_as::<crate::facet::Attr>()
            {
                return with.serialize(item, &mut writer);
            }
        }

        // Handle type attributes.
        for attr in item.shape().attributes {
            // Run the custom serializer.
            if attr.ns.is_some_and(|ns| ns == "mc")
                && attr.key == "with"
                && let Some(crate::facet::Attr::With(Some(with))) =
                    attr.get_as::<crate::facet::Attr>()
            {
                return with.serialize(item, &mut writer);
            }
        }

        // Serialize the item.
        serialize_core(item.peek(), item.is_variable(), &mut writer)
    };

    // Create and complete the serializer.
    let mut ser = Serializer::new(serialize_peek, serialize_variable, &mut core);
    while let Some(result) = Iterator::next(&mut ser) {
        result?;
    }

    // Return the number of bytes written.
    drop(ser);
    Ok(writer.total_written())
}

// -------------------------------------------------------------------------------------------------

/// The serializer logic behind [`serialize`], separated out for readability.
fn serialize_core(
    peek: &Peek<'_, '_>,
    variable: bool,
    writer: &mut Writer<'_>,
) -> Result<(), WriterError> {
    macro_rules! handle {
        (@num $var:expr, $peek:expr => $($ty:ty: $fn:ident),*) => {
            $(
                if let Ok(value) = $peek.get::<$ty>() {
                    return if $var {
                        let (bytes, len) = varint::$fn(*value);
                        writer.write_bytes(&bytes[..len as usize])
                    } else {
                        writer.write_bytes(&value.to_le_bytes())
                    }
                }
            )*
        };
        (@numcast $var:expr, $peek:expr => $($ty:ty, $cast:ty => $fn:ident),*) => {
            $(
                if let Ok(value) = $peek.get::<$ty>() {
                    #[expect(clippy::cast_sign_loss, reason = "Desired behavior")]
                    let cast = *value as $cast;
                    return if $var {
                        let (bytes, len) = varint::$fn(cast);
                        writer.write_bytes(&bytes[..len as usize])
                    } else {
                        writer.write_bytes(&cast.to_le_bytes())
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
    handle!(@num variable, peek => u8: encode_u8, u16: encode_u16, u32: encode_u32, u64: encode_u64, u128: encode_u128);
    handle!(@numcast variable, peek => i8, u8 => encode_u8, i16, u16 => encode_u16, i32, u32 => encode_u32, i64, u64 => encode_u64, i128, u128 => encode_u128);
    // Handle floating-point types
    if let Ok(f32) = peek.get::<f32>() {
        return writer.write_bytes(&f32.to_le_bytes());
    } else if let Ok(f64) = peek.get::<f64>() {
        return writer.write_bytes(&f64.to_le_bytes());
    }

    // Handle strings
    if let Some(string) = peek.as_str() {
        let (bytes, len) = varint::encode_u32(string.len().try_into()?);
        writer.write_bytes(&bytes[..len as usize])?;
        return writer.write_bytes(string.as_bytes());
    }

    todo!("Unsupported type: `{}`", peek.shape().type_name());
}
