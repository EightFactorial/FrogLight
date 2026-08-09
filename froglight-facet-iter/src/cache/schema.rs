//! TODO

use facet::{Facet, Shape};
use facet_solver::{Schema, SchemaError, Solver};
use parking_lot::RwLock;

use super::{LazyLock, LazyMap};

/// A `static` map of cached [`Schema`]s.
static SCHEMAS: LazyMap<&'static Shape, &'static Schema> = LazyLock::new(RwLock::default);

/// Get a cached [`Schema`], or build then cache one.
///
/// Uses [`Schema::build_auto`] to build [`Schema`]s.
///
/// # Errors
///
/// Returns an error if a [`Schema`] cannot be built.
#[inline]
pub fn schema<'facet, T: Facet<'facet>>() -> Result<&'static Schema, SchemaError> {
    schema_for(T::SHAPE)
}

/// Create a [`Solver`] from a cached [`Schema`].
///
/// Uses [`Schema::build_auto`] to build [`Schema`]s.
///
/// # Errors
///
/// Returns an error if a [`Schema`] cannot be built.
#[inline]
pub fn solver<'facet, T: Facet<'facet>>() -> Result<Solver<'static>, SchemaError> {
    schema_for(T::SHAPE).map(Solver::new)
}

/// Get a cached [`Schema`], or build then cache one.
///
/// Uses [`Schema::build_auto`] to build [`Schema`]s.
///
/// # Errors
///
/// Returns an error if a [`Schema`] cannot be built.
pub fn schema_for(shape: &'static Shape) -> Result<&'static Schema, SchemaError> {
    use alloc::boxed::Box;

    let schemas = SCHEMAS.read();
    if let Some(schema) = schemas.get(shape) {
        Ok(schema)
    } else {
        drop(schemas);

        let built = Schema::build_auto(shape)?;
        let built = Box::leak(Box::new(built));
        SCHEMAS.write().insert(shape, built);

        Ok(built)
    }
}
