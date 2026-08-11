//! TODO

use alloc::{string::String, vec::Vec};

use facet::{Facet, HeapValue, Partial, Type, UserType};
use froglight_facet_iter::{
    ReaderError,
    deserialize::{DeserializeError, DeserializeItem, Deserializer, Item},
    solver::tree::{TreeMap, solve_enum, navigate},
};

use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, IndexedSnbtSlice, SliceCore},
    list::IndexedList,
    reference::ValueReference,
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
                let value =
                    navigate::<IndexedSnbtSlice, BORROW>(item.partial(), snbt.root_value())?;

                deserialize_value(item, value).map(Item::Item)
            }
            #[expect(clippy::cast_possible_truncation, reason = "Ignored")]
            Item::Hint(.., partial) => {
                let value = navigate::<IndexedSnbtSlice, BORROW>(&partial, snbt.root_value())?;

                if matches!(partial.shape().ty, Type::User(UserType::Enum(_))) {
                    let (index, _variant) =
                        solve_enum::<IndexedSnbtSlice, BORROW>(&partial, value)?;

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
    nbt: &IndexedSnbtSlice<'facet>,
) -> Result<HeapValue<'facet, true>, DeserializeError> {
    // Create and complete the deserializer.
    let mut core = deserialize_borrowed_core(nbt);

    let de = Deserializer::new(partial, false, &mut core, Some("snbt"));
    de.complete()?.build().map_err(DeserializeError::from)
}

/// The core logic behind [`deserialize_borrowed`], separated out for
/// readability.
#[doc(hidden)]
#[inline(always)]
#[allow(clippy::inline_always, reason = "Performance")]
pub fn deserialize_borrowed_core<'facet>(
    nbt: &IndexedSnbtSlice<'facet>,
) -> impl FnMut(Item<'facet, true>) -> Result<Item<'facet, true>, ReaderError> {
    move |item: Item<'facet, true>| -> Result<Item<'facet, true>, ReaderError> {
        match item {
            Item::Item(item) => {
                let value = navigate::<IndexedSnbtSlice, true>(item.partial(), nbt.root_value())?;

                deserialize_value(item, value).map(Item::Item)
            }
            #[expect(clippy::cast_possible_truncation, reason = "Ignored")]
            Item::Hint(.., partial) => {
                let value = navigate::<IndexedSnbtSlice, true>(&partial, nbt.root_value())?;

                if matches!(partial.shape().ty, Type::User(UserType::Enum(_))) {
                    let (index, _variant) = solve_enum::<IndexedSnbtSlice, true>(&partial, value)?;

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

fn deserialize_value<'facet, const BORROWED: bool, C: IndexCore>(
    item: DeserializeItem<'facet, BORROWED>,
    mut value: ValueReference<'_, C>,
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

                    #[allow(clippy::cast_possible_wrap, reason = "Ignored")]
                    #[allow(trivial_numeric_casts, reason = "Ignored")]
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

    value = match value {
        ValueReference::Compound(compound) => {
            compound.get(item.field().unwrap().effective_name()).ok_or_else(|| {
                ReaderError::from_string(alloc::format!(
                    "Failed to deserialize value: Compound missing \"{}\" field",
                    item.field().unwrap().effective_name()
                ))
            })?
        }
        _ => value,
    };

    match_type! {
        @int
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

    if item.is_type::<String>() {
        let value = value
            .as_string()
            .ok_or_else(|| ReaderError::from_str("Failed to deserialize value: expected String"))?;

        return item.set::<String>(String::from(value.get()));
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

    todo!()
}

// -------------------------------------------------------------------------------------------------

impl TreeMap for IndexedSnbtSlice<'_> {
    type Key<'data> = &'data str;
    type List<'data, 'core: 'data> = IndexedList<'data, SliceCore<'core>>;
    type Map<'data, 'core: 'data> = IndexedCompound<'data, SliceCore<'core>>;
    type Value<'data, 'core: 'data> = ValueReference<'data, SliceCore<'core>>;

    fn value_is_map(value: &Self::Value<'_, '_>) -> bool {
        matches!(value, ValueReference::Compound(..))
    }

    fn value_is_list(value: &Self::Value<'_, '_>) -> bool {
        matches!(value, ValueReference::List(..))
    }

    fn value_map<'data, 'core: 'data>(
        value: Self::Value<'data, 'core>,
    ) -> Option<Self::Map<'data, 'core>> {
        if let ValueReference::Compound(value) = value { Some(value) } else { None }
    }

    fn map_get<'data, 'core: 'data>(
        map: Self::Map<'data, 'core>,
        key: &str,
    ) -> Option<Self::Value<'data, 'core>> {
        map.get(key)
    }

    fn map_iter<'data, 'core: 'data>(
        map: Self::Map<'data, 'core>,
    ) -> impl IntoIterator<Item = (Self::Key<'data>, Self::Value<'data, 'core>)> {
        map.into_iter().map(|entry| (entry.name().get(), entry.value()))
    }

    fn value_list<'data, 'core: 'data>(
        value: Self::Value<'data, 'core>,
    ) -> Option<Self::List<'data, 'core>> {
        if let ValueReference::List(value) = value { Some(value) } else { None }
    }

    fn list_get<'data, 'core: 'data>(
        list: Self::List<'data, 'core>,
        index: usize,
    ) -> Option<Self::Value<'data, 'core>> {
        list.get(index)
    }

    fn list_iter<'data, 'core: 'data>(
        list: Self::List<'data, 'core>,
    ) -> impl IntoIterator<Item = Self::Value<'data, 'core>> {
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
    std::println!("{:#?}", snbt.root());

    let value = Base::from_snbt(&snbt).unwrap();

    #[cfg(feature = "std")]
    std::println!("{value:#?}");
}
