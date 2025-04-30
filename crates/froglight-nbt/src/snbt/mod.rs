//! TODO

use core::{fmt::Debug, marker::PhantomData};

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::{AsRef, Deref, Into};
use smol_str::SmolStr;

mod compat;
pub use compat::Compat;

mod standard;
pub use standard::Standard;

mod error;
pub use error::SnbtError;

/// A stringified [`NbtCompound`](crate::prelude::NbtCompound).
///
/// Uses [`SmolStr`] internally for cheap cloning.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Into, Deref, AsRef)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(no_field_bounds, Debug, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "reflect", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct Snbt<Type: SnbtType = Standard> {
    #[deref]
    #[as_ref]
    inner: SmolStr,
    #[cfg_attr(feature = "reflect", reflect(ignore))]
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
    /// This will cause errors later when converting to an
    /// [`NbtCompound`](crate::prelude::NbtCompound).
    #[must_use]
    pub const fn new_unchecked(inner: SmolStr) -> Self { Self { inner, _marker: PhantomData } }
}
