#![allow(clippy::too_many_lines, reason = "Complex logic function")]

use facet::{Def, Partial, SequenceType, Type, UserType};
use facet_path::PathStep;

use crate::{
    ReaderError,
    solver::tree::{TreeMap, solve_enum},
};

/// Navigate through a value tree based on the provided partial.
///
/// # Errors
///
/// Returns an error if the navigation fails.
pub fn navigate<'data, 'core: 'data, T: TreeMap, const BORROW: bool>(
    partial: &Partial<'_, BORROW>,
    mut value: T::Value<'data, 'core>,
) -> Result<T::Value<'data, 'core>, ReaderError> {
    let path = partial.path();
    let mut step_iter = path.steps().iter();

    let mut shape = partial.root_shape();
    while let Some(step) = step_iter.next() {
        match step {
            PathStep::Field(index) => match shape.ty {
                Type::User(UserType::Struct(ty)) => {
                    // Get the current value as a map.
                    let map = T::value_map(value).ok_or_else(|| {
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
                    let entry = T::map_get(map, field.effective_name()).ok_or_else(|| {
                        ReaderError::from_string(alloc::format!(
                            "Failed to get field with name {:?}",
                            field.effective_name()
                        ))
                    })?;

                    // Update the shape and value.
                    shape = field.shape();
                    value = entry;
                }

                Type::User(UserType::Enum(_ty)) => {
                    let (_index, variant) = solve_enum::<T, BORROW>(partial, value.clone())?;

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
                let list = T::value_list(value).ok_or_else(|| {
                    ReaderError::from_string(alloc::format!(
                        "Failed to get list for type {:?}",
                        shape.type_name()
                    ))
                })?;

                let item = T::list_get(list, *index as usize).ok_or_else(|| {
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
