#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, vec::Vec};

use crate::generated::data::EntityDataType;

/// A collection of [`EntityDataType`]s that used to create an entity.
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityDataSet<'a>(Cow<'a, [(u8, EntityDataType)]>);

/// A collection of [`EntityDataType`]s that used to create an entity.
#[cfg(not(feature = "alloc"))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityDataSet<'a>(&'a [(u8, EntityDataType)]);

impl<'a> EntityDataSet<'a> {
    /// Creates a new [`EntityDataSet`] from a set of [`EntityDataType`]s.
    #[inline]
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn new<T: Into<Cow<'a, [(u8, EntityDataType)]>>>(data: T) -> Self { Self(data.into()) }

    /// Creates a new [`EntityDataSet`] from a set of [`EntityDataType`]s.
    #[inline]
    #[must_use]
    pub const fn new_slice(data: &'a [(u8, EntityDataType)]) -> Self {
        #[cfg(feature = "alloc")]
        {
            Self(Cow::Borrowed(data))
        }
        #[cfg(not(feature = "alloc"))]
        {
            Self(data)
        }
    }

    /// Creates a new [`EntityDataSet`] from a set of [`EntityDataType`]s.
    #[inline]
    #[must_use]
    #[cfg(feature = "alloc")]
    pub const fn new_vec(data: Vec<(u8, EntityDataType)>) -> Self { Self(Cow::Owned(data)) }

    /// Returns the data types in this set as a slice.
    #[must_use]
    pub const fn to_ref(&self) -> &[(u8, EntityDataType)] {
        #[cfg(feature = "alloc")]
        {
            match &self.0 {
                Cow::Borrowed(slice) => slice,
                Cow::Owned(vec) => vec.as_slice(),
            }
        }
        #[cfg(not(feature = "alloc"))]
        {
            self.0
        }
    }

    /// Returns the data types in this set as a mutable vector.
    ///
    /// If the inner set is a reference, it will be cloned into a vector.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn to_mut(&mut self) -> &mut Vec<(u8, EntityDataType)> {
        match self.0 {
            Cow::Borrowed(slice) => {
                let vec = slice.to_vec();
                self.0 = Cow::Owned(vec);
                match &mut self.0 {
                    Cow::Borrowed(_) => unreachable!(),
                    Cow::Owned(vec) => vec,
                }
            }
            Cow::Owned(ref mut vec) => vec,
        }
    }
}
