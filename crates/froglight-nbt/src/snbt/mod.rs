//! TODO

use std::{fmt::Debug, marker::PhantomData};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{AsRef, Deref, Into};
use smol_str::SmolStr;

mod compat;
pub use compat::Compat;

mod standard;
pub use standard::Standard;

/// A stringified [`NbtCompound`](crate::prelude::NbtCompound).
///
/// Uses [`SmolStr`] internally for cheap cloning.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Into, Deref, AsRef)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(no_field_bounds, Debug, PartialEq))]
pub struct Snbt<Type: SnbtType = Standard> {
    #[deref]
    #[as_ref]
    inner: SmolStr,
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    _marker: PhantomData<Type>,
}

/// A trait describing the possible types of SNBT.
pub trait SnbtType: Debug + Copy + Eq + Send + Sync + 'static {}

impl<T: SnbtType> Snbt<T> {
    /// Create a new [`Snbt`] from a [`String`].
    ///
    /// # Warning
    /// This function does not validate the input!
    ///
    /// Using invalid SNBT *will* result in panics!
    pub const fn new_unchecked(inner: SmolStr) -> Self { Self { inner, _marker: PhantomData } }
}
