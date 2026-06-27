//! TODO
#![allow(clippy::result_large_err, reason = "Facet's error type")]

use alloc::vec::Vec;
use core::ops::Deref;

use facet::Facet;
use facet_format::SerializeError;
use facet_value::{ToValueError, Value, ValueError};
use froglight_common::prelude::Identifier;

/// A set of biome attributes.
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BiomeAttributeSet(Vec<(Identifier<'static>, Value)>);

impl BiomeAttributeSet {
    /// Create an empty [`BiomeAttributeSet`] instance.
    #[must_use]
    pub const fn empty() -> Self { Self(Vec::new()) }

    /// Create a new static [`BiomeAttributeSet`].
    ///
    /// # Panics
    ///
    /// Panics if the provided slice contains duplicate entries.
    #[must_use]
    pub const fn new(vec: Vec<(Identifier<'static>, Value)>) -> Self {
        assert_no_duplicates(vec.as_slice());
        Self(vec)
    }

    /// Returns `true` if the set contains the specified attribute type.
    #[must_use]
    pub fn contains<A: AttributeType>(&self) -> bool {
        self.0.iter().any(|(id, _)| id == &A::IDENTIFIER)
    }

    /// Inserts the attribute type into the set,
    /// appending it to the end of the set.
    ///
    /// Returns `true` if the attribute type was inserted into the set.
    ///
    /// # Errors
    ///
    /// Returns an error if the attribute could not be converted into a
    /// [`Value`].
    pub fn insert<A: AttributeType>(
        &mut self,
        attribute: &A,
    ) -> Result<bool, SerializeError<ToValueError>> {
        let data = attribute.to_attribute_data()?;

        // SAFETY: `!self.contains` ensures no duplicates are added.
        let inserted = (!self.contains::<A>()).then(|| self.0.push((A::IDENTIFIER, data)));
        Ok(inserted.is_some())
    }

    /// Get the specified attribute type from the set.
    ///
    /// Returns `None` if the attribute is not present in the set.
    ///
    /// # Errors
    ///
    /// Returns an error if the attribute type could not be converted from its
    /// [`Value`].
    #[must_use]
    pub fn get<A: AttributeType>(&self) -> Option<Result<A, ValueError>> {
        self.0
            .iter()
            .find(|(id, _)| id == &A::IDENTIFIER)
            .map(|(_, value)| A::from_attribute_data(value))
    }

    /// Removes the specified attribute type from the set.
    ///
    /// Returns `Ok(None)` if the attribute type was not present in the set.
    ///
    /// # Errors
    ///
    /// Returns an error if the attribute type could not be converted from its
    /// [`Value`].
    pub fn remove<A: AttributeType>(&mut self) -> Result<Option<A>, ValueError> {
        if let Some(pos) = self.0.iter().position(|(id, _)| id == &A::IDENTIFIER) {
            A::from_attribute_data(&self.0.remove(pos).1).map(Some)
        } else {
            Ok(None)
        }
    }
}

/// Asserts that the given slice contains no duplicate entries.
const fn assert_no_duplicates<T>(slice: &[(Identifier<'static>, T)]) {
    let mut i = 0;
    while i < slice.len() {
        let mut j = i + 1;
        while j < slice.len() {
            assert!(
                !slice[i].0.const_eq(&slice[j].0),
                "`BiomeAttributeSet` contains duplicate entries!"
            );
            j += 1;
        }
        i += 1;
    }
}

impl Deref for BiomeAttributeSet {
    type Target = [(Identifier<'static>, Value)];

    #[inline]
    fn deref(&self) -> &Self::Target { self.0.as_slice() }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all feature types.
pub trait AttributeType: Facet<'static> + Sized {
    /// The [`Identifier`] of this attribute type.
    const IDENTIFIER: Identifier<'static>;

    /// Try to convert [`Value`] into this type.
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails.
    #[inline]
    fn from_attribute_data(data: &Value) -> Result<Self, ValueError> {
        facet_value::from_value::<Self>(data.clone())
    }

    /// Convert this type into [`Value`].
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails.
    #[inline]
    fn to_attribute_data(&self) -> Result<Value, SerializeError<ToValueError>> {
        facet_value::to_value::<Self>(self)
    }
}
