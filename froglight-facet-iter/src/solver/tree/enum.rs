use alloc::{string::String, vec::Vec};

use facet::{FieldKey, Partial, Variant};
use facet_solver::{KeyResult, SatisfyResult, Solver, SolverError};

use crate::{ReaderError, cache::schema::schema_for, solver::tree::TreeMap};

/// Solve for an enum variant based on the provided partial and value tree.
///
/// # Errors
///
/// Returns an error if the solver fails to find a solution.
#[expect(clippy::missing_panics_doc, reason = "Shouldn't panic")]
pub fn solve_enum<'data, 'core: 'data, T: TreeMap, const BORROW: bool>(
    partial: &Partial<'_, BORROW>,
    value: T::Value<'data, 'core>,
) -> Result<(usize, &'static Variant), ReaderError> {
    // Create a solver for the enum variant.
    let schema = schema_for(partial.shape()).map_err(ReaderError::other)?;
    let mut solver = Solver::new(schema);

    // Collect all the keys in the nbt value.
    let mut key_list = Vec::new();
    collect_keys::<T>(value.clone(), &mut Vec::new(), &mut key_list);

    // Solve the enum variant using the collected keys.
    let mut satisfied = Vec::new();
    let mut solution = None;
    for (path, key) in key_list {
        if path.is_empty() {
            match solver.see_key(FieldKey::flat(String::from(key.as_ref()))) {
                KeyResult::Ambiguous { fields } => {
                    if !T::value_is_map(&value) {
                        continue;
                    }

                    satisfied.clear();
                    let map = T::value_map(value.clone()).unwrap();

                    for (field, _score) in fields {
                        if let Some(_value) = T::map_get(map.clone(), field.serialized_name) {
                            satisfied.push(field);
                        }
                    }

                    if let SatisfyResult::Solved(handle) = solver.satisfy(satisfied.as_slice()) {
                        solution = Some(handle);
                        break;
                    }
                }
                KeyResult::Solved(handle) => {
                    solution = Some(handle);
                    break;
                }
                _ => {}
            }
        } else {
            let path: Vec<_> = path.iter().map(AsRef::as_ref).collect();
            if let KeyResult::Solved(handle) = solver.probe_key(path.as_slice(), key.as_ref()) {
                solution = Some(handle);
                break;
            }
        }
    }

    #[allow(clippy::result_large_err, reason = "Ignored")]
    match solution.map_or_else(|| solver.finish(), Ok) {
        // Return the variant from the resolution.
        Ok(handle) => {
            let variant = handle.resolution().variant_selections().first().unwrap();
            Ok(partial.find_variant(variant.variant_name).unwrap())
        }
        // Use the first matching candidate.
        Err(SolverError::Ambiguous { candidates, .. }) if !candidates.is_empty() => {
            let mut candidate = candidates.first().unwrap().as_str();
            if let Some(variant) = candidate.split("::").last() {
                candidate = variant;
            }
            Ok(partial.find_variant(candidate).unwrap())
        }
        // If no solution was found, return an error.
        Err(err) => Err(ReaderError::from_string(alloc::format!(
            "Failed to find a solution for enum {:?}: {err}",
            partial.shape().type_name()
        ))),
    }
}

fn collect_keys<'data, 'core: 'data, T: TreeMap>(
    value: T::Value<'data, 'core>,
    depth: &mut Vec<T::Key<'data>>,
    list: &mut Vec<(Vec<T::Key<'data>>, T::Key<'data>)>,
) {
    if T::value_is_map(&value) {
        let map = T::value_map(value).unwrap();
        for (name, value) in T::map_iter(map) {
            list.push((depth.clone(), name.clone()));

            depth.push(name);
            collect_keys::<T>(value, depth, list);
            let _ = depth.pop();
        }
    } else if T::value_is_list(&value) {
        let vlist = T::value_list(value).unwrap();
        for item in T::list_iter(vlist) {
            collect_keys::<T>(item, depth, list);
        }
    }
}
