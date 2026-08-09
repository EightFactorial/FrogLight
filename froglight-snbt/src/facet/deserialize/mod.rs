//! TODO

use alloc::{string::String, vec::Vec};

use facet::{Def, Facet, HeapValue, Partial, SequenceType, Type, UserType};
use facet_path::PathStep;
use facet_solver::{KeyResult, Solver};
use froglight_facet_iter::{
    ReaderError,
    deserialize::{DeserializeError, DeserializeItem, Deserializer, Item},
};

use crate::types::indexed::{
    core::{IndexCore, IndexedSnbtSlice, SliceCore},
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
fn deserialize_owned(
    partial: Partial<'static, false>,
    snbt: &IndexedSnbtSlice<'_>,
) -> Result<HeapValue<'static, false>, DeserializeError> {
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
pub fn deserialize_owned_core<'facet>(
    snbt: &IndexedSnbtSlice<'_>,
) -> impl FnMut(Item<'facet, false>) -> Result<Item<'facet, false>, ReaderError> {
    move |item: Item<'facet, false>| -> Result<Item<'facet, false>, ReaderError> {
        match item {
            Item::Item(item) => {
                let mut value = snbt.root_value();
                value = navigate_snbt(item.partial(), value)?;

                deserialize_value(item, value).map(Item::Item)
            }
            #[expect(clippy::cast_possible_truncation, reason = "Ignored")]
            Item::Hint(.., partial) => {
                let mut value = snbt.root_value();
                value = navigate_snbt(&partial, value)?;

                if matches!(partial.shape().ty, Type::User(UserType::Enum(_))) {
                    let variant = solve_enum_variant(&partial, value)?;
                    let (index, _) = partial.find_variant(variant).unwrap();

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
                let mut value = nbt.root_value();
                value = navigate_snbt(item.partial(), value)?;

                deserialize_value(item, value).map(Item::Item)
            }
            #[expect(clippy::cast_possible_truncation, reason = "Ignored")]
            Item::Hint(.., partial) => {
                let mut value = nbt.root_value();
                value = navigate_snbt(&partial, value)?;

                if matches!(partial.shape().ty, Type::User(UserType::Enum(_))) {
                    let variant = solve_enum_variant(&partial, value)?;
                    let (index, _) = partial.find_variant(variant).unwrap();

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

#[allow(clippy::too_many_lines, reason = "Complex logic function")]
fn navigate_snbt<'core, const BORROWED: bool>(
    partial: &Partial<'_, BORROWED>,
    mut value: ValueReference<'core, SliceCore<'core>>,
) -> Result<ValueReference<'core, SliceCore<'core>>, ReaderError> {
    let path = partial.path();
    let mut step_iter = path.steps().iter();

    let mut shape = partial.root_shape();
    while let Some(step) = step_iter.next() {
        match step {
            PathStep::Field(index) => match shape.ty {
                Type::User(UserType::Struct(ty)) => {
                    // Get the current nbt value as a compound.
                    let compound = value.as_compound().ok_or_else(|| {
                        ReaderError::from_string(alloc::format!(
                            "Failed to get compound for struct {:?}",
                            shape.type_name()
                        ))
                    })?;

                    // Get the field with the given index.
                    let field = ty.fields.get(*index as usize).ok_or_else(|| {
                        ReaderError::from_string(alloc::format!(
                            "Failed to get field with index {index} in struct {:?}",
                            shape.type_name()
                        ))
                    })?;

                    // Get the entry with the given field name.
                    value = compound.get(field.effective_name()).ok_or_else(|| {
                        ReaderError::from_string(alloc::format!(
                            "Failed to get field with name {:?}",
                            field.effective_name()
                        ))
                    })?;

                    // Update the shape.
                    shape = field.shape();
                }

                Type::User(UserType::Enum(ty)) => {
                    let variant = solve_enum_variant(partial, value)?;
                    let variant =
                        ty.variants.iter().find(|v| v.name == variant).ok_or_else(|| {
                            ReaderError::from_string(alloc::format!(
                                "Failed to get variant with name {:?} in enum {:?}",
                                variant,
                                shape.type_name()
                            ))
                        })?;

                    // Get the field with the given index.
                    let field = variant.data.fields.get(*index as usize).ok_or_else(|| {
                        ReaderError::from_string(alloc::format!(
                            "Failed to get field with index {index} in struct {:?}",
                            shape.type_name()
                        ))
                    })?;

                    // Update the shape.
                    shape = field.shape();
                }

                _ => todo!(),
            },

            PathStep::Variant(index) => match shape.ty {
                Type::User(UserType::Enum(ty)) => {
                    let variant = ty.variants.get(*index as usize).ok_or_else(|| {
                        ReaderError::from_string(alloc::format!(
                            "Failed to get variant with index {index} in enum {:?}",
                            shape.type_name()
                        ))
                    })?;

                    let Some(PathStep::Field(index)) = step_iter.next() else {
                        return Err(ReaderError::from_string(alloc::format!(
                            "Failed to get field index for variant {:?} in enum {:?}",
                            variant.name,
                            shape.type_name()
                        )));
                    };

                    // Get the field with the given index.
                    let field = variant.data.fields.get(*index as usize).ok_or_else(|| {
                        ReaderError::from_string(alloc::format!(
                            "Failed to get field with index {index} in struct {:?}",
                            shape.type_name()
                        ))
                    })?;

                    // Update the shape.
                    shape = field.shape();
                }
                _ => todo!(),
            },

            PathStep::Index(index) => {
                let list = value.as_list().ok_or_else(|| {
                    ReaderError::from_string(alloc::format!(
                        "Failed to get list for type {:?}",
                        shape.type_name()
                    ))
                })?;

                let item = list.get(*index as usize).ok_or_else(|| {
                    ReaderError::from_string(alloc::format!(
                        "Failed to get list item with index {index} for type {:?}",
                        shape.type_name()
                    ))
                })?;

                // Update the shape and value.
                value = item;
                match (shape.def, shape.ty) {
                    (Def::Array(def), _) => shape = def.t,
                    (Def::List(def), _) => shape = def.t,
                    (Def::Slice(def), _) => shape = def.t,
                    (_, Type::Sequence(SequenceType::Array(ty))) => shape = ty.t,
                    (_, Type::Sequence(SequenceType::Slice(ty))) => shape = ty.t,
                    _ => Err(ReaderError::from_string(alloc::format!(
                        "Failed to get list item type for list type {:?}",
                        shape.type_name()
                    )))?,
                }
            }

            _ => todo!(),
        }
    }

    Ok(value)
}

fn solve_enum_variant<const BORROWED: bool>(
    partial: &Partial<'_, BORROWED>,
    value: ValueReference<'_, SliceCore<'_>>,
) -> Result<&'static str, ReaderError> {
    fn collect_nbt_keys<'data>(
        value: ValueReference<'data, SliceCore<'_>>,
        depth: &mut Vec<&'data str>,
        list: &mut Vec<(Vec<&'data str>, &'data str)>,
    ) {
        match value {
            ValueReference::Compound(compound) => {
                for entry in compound {
                    let (name, value) = entry.pair();

                    let name = name.get();
                    list.push((depth.clone(), name));

                    depth.push(name);
                    collect_nbt_keys(value, depth, list);
                    let _ = depth.pop();
                }
            }
            ValueReference::List(vlist) => {
                for item in vlist {
                    collect_nbt_keys(item, depth, list);
                }
            }
            _ => {}
        }
    }

    // Create a solver for the enum variant.
    let schema = froglight_facet_iter::cache::schema::schema_for(partial.shape())
        .map_err(ReaderError::other)?;
    let mut solver = Solver::new(schema);

    // Collect all the keys in the nbt value.
    let mut key_list = Vec::new();
    collect_nbt_keys(value, &mut Vec::new(), &mut key_list);

    // Solve the enum variant using the collected keys.
    let mut solution = None;
    for (path, key) in key_list {
        if let KeyResult::Solved(resolution) = solver.probe_key(path.as_slice(), key.as_ref()) {
            solution = Some(resolution.resolution());
            break;
        }
    }

    // If no solution was found, return an error.
    let solution = solution.ok_or_else(|| {
        ReaderError::from_string(alloc::format!(
            "Failed to find a solution for enum {:?}",
            partial.shape().type_name()
        ))
    })?;

    // Return the variant name from the solution.
    Ok(solution.variant_selections().first().unwrap().variant_name)
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

    let partial = Partial::alloc_owned::<Base>().unwrap();
    let value = deserialize_owned(partial, &snbt).unwrap();
    let value = value.materialize::<Base>().unwrap();

    #[cfg(feature = "std")]
    std::println!("{value:#?}");
}
