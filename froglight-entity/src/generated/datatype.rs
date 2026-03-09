//! Placeholder

use alloc::borrow::Cow;

#[cfg(feature = "facet")]
use facet_minecraft as mc;

generate! {
    @datatypes
    as_byte => Byte(u8),
    as_integer => Integer(#[cfg_attr(feature = "facet", facet(mc::variable))] i32),
    as_long => Long(#[cfg_attr(feature = "facet", facet(mc::variable))] i64),
    as_float => Float(f32),
    as_string => String(Cow<'static, str>),
}
