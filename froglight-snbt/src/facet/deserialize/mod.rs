//! TODO

use alloc::{borrow::Cow, string::String, vec::Vec};

use facet::{Facet, HeapValue, Partial, Type, UserType};
use froglight_facet_iter::{
    ReaderError,
    deserialize::{DeserializeError, DeserializeItem, Deserializer, Item},
    solver::tree::{TreeMap, naviate_field, navigate_tree, solve_enum},
};

use crate::{
    prelude::*,
    types::indexed::{
        compound::IndexedCompound, core::SliceCore, list::IndexedList, reference::ValueReference,
    },
};

pub mod functions;

/// A trait for types that can be deserialized from [`Snbt`].
pub trait DeserializeSnbt<'facet>: Facet<'facet> + Sized {
    /// Deserialize a value from an SNBT string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not valid SNBT,
    /// or if the deserialization fails.
    #[inline]
    fn from_snbt_string(string: &str) -> Result<Self, DeserializeError>
    where
        'facet: 'static,
        'static: 'facet,
    {
        IndexedSnbtSlice::new_ref(string)
            .map_or_else(|()| Err(DeserializeError), |snbt| Self::from_snbt(&snbt))
    }

    /// Deserialize a value from an [`IndexedNbtSlice`].
    ///
    /// # Errors
    ///
    /// Returns an error if the deserialization fails.
    fn from_snbt(snbt: &IndexedSnbtSlice<'_>) -> Result<Self, DeserializeError>
    where
        'facet: 'static,
        'static: 'facet;

    /// Deserialize a value from an [`IndexedNbtSlice`].
    ///
    /// Borrows from the input slice where possible.
    ///
    /// # Errors
    ///
    /// Returns an error if the deserialization fails.
    fn from_snbt_borrowed(snbt: &IndexedSnbtSlice<'facet>) -> Result<Self, DeserializeError>;
}

impl<'facet, T: Facet<'facet> + Sized> DeserializeSnbt<'facet> for T {
    #[inline]
    fn from_snbt(snbt: &IndexedSnbtSlice<'_>) -> Result<Self, DeserializeError>
    where
        'facet: 'static,
        'static: 'facet,
    {
        let plan = froglight_facet_iter::cache::typeplan::typeplan::<T>()?;
        let value = deserialize_owned(Partial::alloc_owned_with_plan(plan)?, snbt)?;
        Ok(value.materialize::<T>()?)
    }

    #[inline]
    fn from_snbt_borrowed(snbt: &IndexedSnbtSlice<'facet>) -> Result<Self, DeserializeError> {
        let plan = froglight_facet_iter::cache::typeplan::typeplan::<T>()?;
        let value = deserialize_borrowed(Partial::alloc_with_plan(plan)?, snbt)?;
        Ok(value.materialize::<T>()?)
    }
}

// -------------------------------------------------------------------------------------------------

#[inline(never)]
fn deserialize_owned<const BORROW: bool>(
    partial: Partial<'static, BORROW>,
    snbt: &IndexedSnbtSlice<'_>,
) -> Result<HeapValue<'static, BORROW>, DeserializeError> {
    // Create and complete the deserializer.
    let mut core = deserialize_owned_core(snbt);

    let de = Deserializer::new(partial, false, &mut core, Some("snbt"));
    de.complete()?.build().map_err(DeserializeError::from)
}

/// The core logic behind [`deserialize_owned`], separated out for
/// readability.
#[doc(hidden)]
#[inline(always)]
#[allow(clippy::inline_always, reason = "Performance")]
pub fn deserialize_owned_core<'facet, const BORROW: bool>(
    snbt: &IndexedSnbtSlice<'_>,
) -> impl FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError> {
    move |item: Item<'facet, BORROW>| -> Result<Item<'facet, BORROW>, ReaderError> {
        match item {
            Item::Item(item) => {
                let mut value = navigate_tree::<Snbt, BORROW>(item.partial(), snbt.as_value())?;
                if let Some(field) = item.field() {
                    value = naviate_field::<Snbt>(field, value);
                }

                deserialize_value(item, value).map(Item::Item)
            }
            #[expect(clippy::cast_possible_truncation, reason = "Ignored")]
            Item::Hint(.., partial) => {
                let value = navigate_tree::<Snbt, BORROW>(&partial, snbt.as_value())?;

                if matches!(partial.shape().ty, Type::User(UserType::Enum(_))) {
                    let (index, _variant) = solve_enum::<Snbt, BORROW>(&partial, value)?;

                    Ok(Item::Hint(index as u32, partial)) // Enum Variant
                } else if let ValueReference::Compound(value) = &value {
                    Ok(Item::Hint(value.len() as u32, partial)) // Map
                } else if let ValueReference::List(value) = &value {
                    Ok(Item::Hint(value.len() as u32, partial)) // List
                } else {
                    todo!()
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[inline(never)]
fn deserialize_borrowed<'facet>(
    partial: Partial<'facet, true>,
    snbt: &IndexedSnbtSlice<'facet>,
) -> Result<HeapValue<'facet, true>, DeserializeError> {
    // Create and complete the deserializer.
    let mut core = deserialize_borrowed_core(snbt);

    let de = Deserializer::new(partial, false, &mut core, Some("snbt"));
    de.complete()?.build().map_err(DeserializeError::from)
}

/// The core logic behind [`deserialize_borrowed`], separated out for
/// readability.
#[doc(hidden)]
#[inline(always)]
#[allow(clippy::inline_always, reason = "Performance")]
pub fn deserialize_borrowed_core<'facet>(
    snbt: &IndexedSnbtSlice<'facet>,
) -> impl FnMut(Item<'facet, true>) -> Result<Item<'facet, true>, ReaderError> {
    move |item: Item<'facet, true>| -> Result<Item<'facet, true>, ReaderError> {
        match item {
            Item::Item(item) => {
                let mut value = navigate_tree::<Snbt, true>(item.partial(), snbt.as_value())?;
                if let Some(field) = item.field() {
                    value = naviate_field::<Snbt>(field, value);
                }

                // Borrow string values.
                if item.is_type::<&str>() || item.is_type::<Cow<'_, str>>() {
                    let value = value.as_string().ok_or_else(|| {
                        ReaderError::from_str("Failed to deserialize value: expected String")
                    })?;

                    // SAFETY: The lifetime is upgraded using the original SNBT string.
                    let string = unsafe { value.upgrade(snbt.as_str()).get() };

                    if item.is_type::<&str>() {
                        return item.set::<&str>(string).map(Item::Item);
                    } else if item.is_type::<Cow<'_, str>>() {
                        return item.set::<Cow<'_, str>>(Cow::Borrowed(string)).map(Item::Item);
                    }
                }

                deserialize_value(item, value).map(Item::Item)
            }
            #[expect(clippy::cast_possible_truncation, reason = "Ignored")]
            Item::Hint(.., partial) => {
                let value = navigate_tree::<Snbt, true>(&partial, snbt.as_value())?;

                if matches!(partial.shape().ty, Type::User(UserType::Enum(_))) {
                    let (index, _variant) = solve_enum::<Snbt, true>(&partial, value)?;

                    Ok(Item::Hint(index as u32, partial)) // Enum Variant
                } else if let ValueReference::Compound(value) = &value {
                    Ok(Item::Hint(value.len() as u32, partial)) // Map
                } else if let ValueReference::List(value) = &value {
                    Ok(Item::Hint(value.len() as u32, partial)) // List
                } else {
                    todo!()
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

fn deserialize_value<'facet, const BORROWED: bool>(
    item: DeserializeItem<'facet, BORROWED>,
    value: ValueReference<'_, SliceCore<'_>>,
) -> Result<DeserializeItem<'facet, BORROWED>, ReaderError> {
    macro_rules! match_type {
        ( @int $($ty:ty => $ty_fn:ident),* ) => {
            $(
                if item.is_type::<$ty>() {
                    let value = value.$ty_fn().ok_or_else(|| {
                        ReaderError::from_string(alloc::format!(
                            "Failed to deserialize value: expected {:?}", stringify!($ty)
                        ))
                    })?;

                    #[allow(trivial_casts, reason = "Ignored")]
                    #[allow(trivial_numeric_casts, reason = "Ignored")]
                    #[allow(clippy::cast_possible_wrap, reason = "Ignored")]
                    return item.set::<$ty>(value.get() as _);
                }
            )*
        };
        ( @vec $($ty:ty $(as $ty_cast:ty)? => $ty_fn:ident),* ) => {
            $(
                if item.is_type::<$ty>() {
                    let value = value.$ty_fn().ok_or_else(|| {
                        ReaderError::from_string(alloc::format!(
                            "Failed to deserialize value: expected {:?}", stringify!($ty)
                        ))
                    })?;

                    let value = value.into_iter().collect::<Vec<_>>();
                    $(
                        #[allow(clippy::cast_possible_wrap, reason = "Ignored")]
                        let value = value.into_iter().map(|v| v as $ty_cast).collect::<Vec<_>>();
                    )?

                    return item.set::<$ty>(value);
                }
            )*
        };
    }

    if item.is_type::<String>() || item.is_type::<Cow<'_, str>>() {
        let value = value
            .as_string()
            .ok_or_else(|| ReaderError::from_str("Failed to deserialize value: expected String"))?;

        let string = String::from(value.get());

        if item.is_type::<String>() {
            return item.set::<String>(string);
        } else if item.is_type::<Cow<'_, str>>() {
            return item.set::<Cow<'_, str>>(Cow::Owned(string));
        }
    }

    match_type! {
        @int
        bool => as_bool,
        u8 => as_byte,
        i8 => as_byte,
        u16 => as_short,
        i16 => as_short,
        u32 => as_int,
        i32 => as_int,
        u64 => as_long,
        i64 => as_long,
        f32 => as_float,
        f64 => as_double
    }

    match_type! {
        @vec
        Vec<u8> => as_byte_array,
        Vec<i8> as i8 => as_byte_array,
        Vec<u32> => as_int_array,
        Vec<i32> as i32 => as_int_array,
        Vec<u64> => as_long_array,
        Vec<i64> as i64 => as_long_array
    }

    todo!("Unhandled type: {:?}", item.shape().type_name())
}

// -------------------------------------------------------------------------------------------------

struct Snbt;

impl TreeMap for Snbt {
    type Key<'index> = &'index str;
    type List<'index, 'core: 'index> = IndexedList<'index, SliceCore<'core>>;
    type Map<'index, 'core: 'index> = IndexedCompound<'index, SliceCore<'core>>;
    type Value<'index, 'core: 'index> = ValueReference<'index, SliceCore<'core>>;

    fn value_is_map(value: &Self::Value<'_, '_>) -> bool {
        matches!(value, ValueReference::Compound(..))
    }

    fn value_is_list(value: &Self::Value<'_, '_>) -> bool {
        matches!(value, ValueReference::List(..))
    }

    fn value_map<'index, 'core>(
        value: Self::Value<'index, 'core>,
    ) -> Option<Self::Map<'index, 'core>> {
        if let ValueReference::Compound(value) = value { Some(value) } else { None }
    }

    fn map_contains(map: &Self::Map<'_, '_>, key: &str) -> bool { map.get(key).is_some() }

    fn map_get<'index, 'core>(
        map: Self::Map<'index, 'core>,
        key: &str,
    ) -> Option<Self::Value<'index, 'core>> {
        map.get(key)
    }

    fn map_iter<'index, 'core>(
        map: Self::Map<'index, 'core>,
    ) -> impl IntoIterator<Item = (Self::Key<'index>, Self::Value<'index, 'core>)> {
        map.into_iter().map(|entry| (entry.name().get(), entry.value()))
    }

    fn value_list<'index, 'core>(
        value: Self::Value<'index, 'core>,
    ) -> Option<Self::List<'index, 'core>> {
        if let ValueReference::List(value) = value { Some(value) } else { None }
    }

    fn list_get<'index, 'core>(
        list: Self::List<'index, 'core>,
        index: usize,
    ) -> Option<Self::Value<'index, 'core>> {
        list.get(index)
    }

    fn list_iter<'index, 'core>(
        list: Self::List<'index, 'core>,
    ) -> impl IntoIterator<Item = Self::Value<'index, 'core>> {
        list.into_iter()
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
#[allow(unused_variables, reason = "Testing")]
#[allow(clippy::struct_field_names, reason = "Testing")]
fn test() {
    use facet::Facet;

    #[derive(Debug, Facet)]
    struct Base {
        base_a: u8,
        base_b: BaseInner,
        base_c: u16,
    }

    #[repr(u8)]
    #[derive(Debug, Facet)]
    enum BaseInner {
        VariantA { inner_a: u32 },
        VariantB(BaseInnerValue),
    }

    #[derive(Debug, Facet)]
    struct BaseInnerValue {
        inner_a: u32,
        inner_b: u64,
    }

    static SLICE: &str = "{ base_a: 123b, base_b: { inner_a: 123, inner_b: 123L }, base_c: 123S }";

    let snbt = IndexedSnbtSlice::new_ref(SLICE).unwrap();

    #[cfg(feature = "std")]
    std::println!("{:#?}", snbt.as_compound());

    let value = Base::from_snbt(&snbt).unwrap();

    #[cfg(feature = "std")]
    std::println!("{value:#?}");
}
