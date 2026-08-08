//! TODO

use core::any::TypeId;
#[cfg(feature = "std")]
use std::sync::LazyLock;

use facet::Shape;
use facet_solver::{Schema, SchemaError, Solver};
use foldhash::fast::RandomState;
use hashbrown::HashMap;
#[cfg(not(feature = "std"))]
use once_cell::sync::Lazy as LazyLock;
use parking_lot::RwLock;

/// A `static` map of cached [`Schema`]s.
static SCHEMAS: LazyLock<RwLock<HashMap<TypeId, &'static Schema, RandomState>>> =
    LazyLock::new(RwLock::default);

/// Get a cached [`Schema`], or build then cache one.
///
/// Uses [`Schema::build_auto`] to build [`Schema`]s.
///
/// # Errors
///
/// Returns an error if a [`Schema`] cannot be built.
pub fn schema(shape: &'static Shape) -> Result<&'static Schema, SchemaError> {
    use alloc::boxed::Box;

    let schemas = SCHEMAS.read();
    if let Some(schema) = schemas.get(&shape.id.get()) {
        Ok(schema)
    } else {
        drop(schemas);

        let built = Schema::build_auto(shape)?;
        let built = Box::leak(Box::new(built));
        SCHEMAS.write().insert(shape.id.get(), built);

        Ok(built)
    }
}

/// Create a [`Solver`] from a cached [`Schema`].
///
/// Uses [`Schema::build_auto`] to build [`Schema`]s.
///
/// # Errors
///
/// Returns an error if a [`Schema`] cannot be built.
#[inline]
pub fn solver(shape: &'static Shape) -> Result<Solver<'static>, SchemaError> {
    schema(shape).map(Solver::new)
}
