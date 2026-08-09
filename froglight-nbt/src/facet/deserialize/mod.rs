//! TODO

use alloc::{borrow::Cow, string::String, vec::Vec};

use facet::{Def, HeapValue, Partial, SequenceType, Type, UserType};
use facet_path::PathStep;
use facet_solver::{KeyResult, Solver};
use froglight_facet_iter::{
    ReaderError,
    deserialize::{DeserializeError, DeserializeItem, Deserializer, Item},
};

use crate::{
    prelude::*,
    types::indexed::{
        core::{IndexCore, Ref, SliceCore},
        list::ValueList,
        reference::ValueReference,
    },
};

pub mod functions;

/// A trait for types that can be deserialized from [`Nbt`].
pub trait DeserializeNbt<'facet> {}

// -------------------------------------------------------------------------------------------------

#[inline(never)]
fn deserialize_owned<'facet>(
    partial: Partial<'facet, false>,
    nbt: &IndexedNbtSlice<'facet>,
) -> Result<HeapValue<'facet, false>, DeserializeError> {
    // Create and complete the deserializer.
    let mut core = deserialize_owned_core(nbt);

    let de = Deserializer::new(partial, false, &mut core, Some("nbt"));
    de.complete()?.build().map_err(DeserializeError::from)
}

/// The core logic behind [`deserialize_owned`], separated out for
/// readability.
#[doc(hidden)]
#[inline(always)]
#[allow(clippy::inline_always, reason = "Performance")]
pub fn deserialize_owned_core<'facet>(
    nbt: &IndexedNbtSlice<'facet>,
) -> impl FnMut(Item<'facet, false>) -> Result<Item<'facet, false>, ReaderError> {
    move |item: Item<'facet, false>| -> Result<Item<'facet, false>, ReaderError> {
        match item {
            Item::Item(item) => {
                let mut value = nbt.as_value();
                value = navigate_nbt(item.partial(), value)?;

                deserialize_value(item, value).map(Item::Item)
            }
            #[expect(clippy::cast_possible_truncation, reason = "Ignored")]
            Item::Hint(.., partial) => {
                let mut value = nbt.as_value();
                value = navigate_nbt(&partial, value)?;

                if matches!(partial.shape().ty, Type::User(UserType::Enum(_))) {
                    let variant = solve_enum_variant(&partial, value)?;
                    let (index, _) = partial.find_variant(variant).unwrap();

                    Ok(Item::Hint(index as u32, partial))
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
    nbt: &IndexedNbtSlice<'facet>,
) -> Result<HeapValue<'facet, true>, DeserializeError> {
    // Create and complete the deserializer.
    let mut core = deserialize_borrowed_core(nbt);

    let de = Deserializer::new(partial, false, &mut core, Some("nbt"));
    de.complete()?.build().map_err(DeserializeError::from)
}

/// The core logic behind [`deserialize_borrowed`], separated out for
/// readability.
#[doc(hidden)]
#[inline(always)]
#[allow(clippy::inline_always, reason = "Performance")]
pub fn deserialize_borrowed_core<'facet>(
    nbt: &IndexedNbtSlice<'facet>,
) -> impl FnMut(Item<'facet, true>) -> Result<Item<'facet, true>, ReaderError> {
    move |item: Item<'facet, true>| -> Result<Item<'facet, true>, ReaderError> {
        match item {
            Item::Item(item) => {
                let mut value = nbt.as_value();
                value = navigate_nbt(item.partial(), value)?;

                deserialize_value(item, value).map(Item::Item)
            }
            #[expect(clippy::cast_possible_truncation, reason = "Ignored")]
            Item::Hint(.., partial) => {
                let mut value = nbt.as_value();
                value = navigate_nbt(&partial, value)?;

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
fn navigate_nbt<'nbt, 'core, const BORROWED: bool>(
    partial: &Partial<'_, BORROWED>,
    mut value: ValueReference<'nbt, Ref, SliceCore<'core, Ref>>,
) -> Result<ValueReference<'nbt, Ref, SliceCore<'core, Ref>>, ReaderError> {
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
                    let entry = compound.get(field.effective_name()).ok_or_else(|| {
                        ReaderError::from_string(alloc::format!(
                            "Failed to get field with name {:?}",
                            field.effective_name()
                        ))
                    })?;

                    // Update the shape and value.
                    shape = field.shape();
                    value = entry.into_value();
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
    value: ValueReference<'_, Ref, SliceCore<'_, Ref>>,
) -> Result<&'static str, ReaderError> {
    fn collect_nbt_keys<'data>(
        value: ValueReference<'data, Ref, SliceCore<'_, Ref>>,
        depth: &mut Vec<Cow<'data, str>>,
        list: &mut Vec<(Vec<Cow<'data, str>>, Cow<'data, str>)>,
    ) {
        match value {
            ValueReference::Compound(compound) => {
                for entry in compound {
                    let (name, value) = entry.pair();
                    let name = name.get().to_utf8();
                    list.push((depth.clone(), name.clone()));

                    depth.push(name);
                    collect_nbt_keys(value.into_value(), depth, list);
                    let _ = depth.pop();
                }
            }
            ValueReference::List(ValueList::Compound(compounds)) => {
                for compound in compounds {
                    collect_nbt_keys(ValueReference::Compound(compound), depth, list);
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
        let path: Vec<_> = path.iter().map(AsRef::as_ref).collect();
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

fn deserialize_value<'facet, const BORROWED: bool, C: IndexCore<Ref>>(
    item: DeserializeItem<'facet, BORROWED>,
    mut value: ValueReference<'_, Ref, C>,
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

                    let value = value.get().to_vec();
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
        ValueReference::Compound(compound) => compound
            .get(item.field().unwrap().effective_name())
            .ok_or_else(|| {
                ReaderError::from_string(alloc::format!(
                    "Failed to deserialize value: Compound missing \"{}\" field",
                    item.field().unwrap().effective_name()
                ))
            })?
            .into_value(),
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

    static SLICE: &[u8] = &[
        0x0A, 0x01, 0x00, 0x06, 0x62, 0x61, 0x73, 0x65, 0x5F, 0x61, 0x40, 0x0A, 0x00, 0x06, 0x62,
        0x61, 0x73, 0x65, 0x5F, 0x62, 0x03, 0x00, 0x07, 0x69, 0x6E, 0x6E, 0x65, 0x72, 0x5F, 0x61,
        0x00, 0x00, 0x19, 0x28, 0x04, 0x00, 0x07, 0x69, 0x6E, 0x6E, 0x65, 0x72, 0x5F, 0x62, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x28, 0x00, 0x02, 0x00, 0x06, 0x62, 0x61, 0x73, 0x65,
        0x5F, 0x63, 0x19, 0x28, 0x00,
    ];

    let nbt = IndexedNbtSlice::new_unnamed(SLICE).unwrap();
    std::println!("{nbt:#?}");

    let partial = Partial::alloc_owned::<Base>().unwrap();
    let value = deserialize_owned(partial, &nbt).unwrap();
    let value = value.materialize::<Base>().unwrap();
    std::println!("{value:#?}");
}
