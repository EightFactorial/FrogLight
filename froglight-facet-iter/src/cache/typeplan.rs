//! TODO

use alloc::sync::Arc;

use facet::{AllocError, Facet, Shape, TypePlanCore};
use parking_lot::RwLock;

use super::LazyMap;

/// A `static` map of cached [`TypePlanCore`]s.
static SCHEMAS: LazyMap<&'static Shape, Arc<TypePlanCore>> = LazyMap::new(RwLock::default);

/// Get a cached [`TypePlanCore`], or build then cache one.
///
/// Uses [`TypePlanCore::from_shape`] to build [`TypePlanCore`]s.
///
/// # Errors
///
/// Returns an error if a [`Schema`] cannot be built.
#[inline]
pub fn typeplan<'facet, T: Facet<'facet>>() -> Result<Arc<TypePlanCore>, AllocError> {
    // SAFETY: T::SHAPE comes from Facet metadata for a real type T.
    unsafe { typeplan_for(T::SHAPE) }
}

/// Get a cached [`TypePlanCore`], or build then cache one.
///
/// Uses [`TypePlanCore::from_shape`] to build [`TypePlanCore`]s.
///
/// # Errors
///
/// Returns an error if a [`Schema`] cannot be built.
///
/// # Safety
///
/// The caller must ensure that the shape is valid and corresponds to a real
/// type. Using an incorrect or maliciously crafted shape can lead to undefined
/// behavior when materializing values.
pub unsafe fn typeplan_for(shape: &'static Shape) -> Result<Arc<TypePlanCore>, AllocError> {
    let schemas = SCHEMAS.read();
    if let Some(core) = schemas.get(&shape) {
        Ok(Arc::clone(core))
    } else {
        drop(schemas);

        // SAFETY: T::SHAPE comes from Facet metadata for a real type (caller-ensured).
        let built = unsafe { TypePlanCore::from_shape(shape) }?;
        SCHEMAS.write().insert(shape, Arc::clone(&built));

        Ok(built)
    }
}
