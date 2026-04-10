//! TODO

use alloc::vec::Vec;
use core::any::TypeId;

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use froglight_common::prelude::*;

/// An entity's set of [`EntityAttribute`]s.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Component))]
pub struct EntityAttributeSet(Vec<EntityAttribute>);

impl EntityAttributeSet {
    /// Create a new, empty [`EntityAttributeSet`].
    #[must_use]
    #[expect(clippy::new_without_default, reason = "WIP")]
    pub const fn new() -> Self { Self(Vec::new()) }

    /// Create a new, empty [`EntityAttributeSet`] with at least the specified
    /// capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity)) }

    /// Get all [`EntityAttribute`]s in this set as a slice.
    #[must_use]
    pub const fn as_slice(&self) -> &[EntityAttribute] { self.0.as_slice() }

    /// Insert the given [`EntityAttribute`] into this set,
    /// returning the old attribute of the same type if it was present.
    pub fn insert(&mut self, attribute: EntityAttribute) -> Option<EntityAttribute> {
        if let Some(index) = self.0.iter().position(|attr| attr.attr_ty == attribute.attr_ty) {
            Some(core::mem::replace(&mut self.0[index], attribute))
        } else {
            self.0.push(attribute);
            None
        }
    }

    /// Remove the attribute of the given type and version from this set,
    /// returning it if it was present.
    pub fn remove<T: EntityAttributeType<V>, V: Version>(&mut self) -> Option<EntityAttribute> {
        let (attr_ty, version_ty) = (TypeId::of::<T>(), TypeId::of::<V>());
        self.0
            .iter()
            .position(|attr| attr.attr_ty == attr_ty && attr.version_ty == version_ty)
            .map(|index| self.0.remove(index))
    }

    /// Remove the attribute of the given type from this set,
    /// returning it if it was present.
    pub fn remove_type<T: EntityAttributeType<V>, V: Version>(
        &mut self,
    ) -> Option<EntityAttribute> {
        let attr_ty = TypeId::of::<T>();
        self.0.iter().position(|attr| attr.attr_ty == attr_ty).map(|index| self.0.remove(index))
    }

    /// Remove the attribute with the given identifier from this set,
    /// returning it if it was present.
    pub fn remove_identifier(&mut self, ident: &str) -> Option<EntityAttribute> {
        self.0.iter().position(|attr| &attr.ident == ident).map(|index| self.0.remove(index))
    }

    /// Returns `true` if this set contains the attribute type and version.
    #[must_use]
    pub fn contains<T: EntityAttributeType<V>, V: Version>(&self) -> bool {
        let (attr_ty, version_ty) = (TypeId::of::<T>(), TypeId::of::<V>());
        self.as_slice().iter().any(|attr| attr.attr_ty == attr_ty && attr.version_ty == version_ty)
    }

    /// Returns `true` if this set contains the attribute type.
    #[must_use]
    pub fn contains_type<T: EntityAttributeType<V>, V: Version>(&self) -> bool {
        let attr_ty = TypeId::of::<T>();
        self.as_slice().iter().any(|attr| attr.attr_ty == attr_ty)
    }

    /// Returns `true` if this set contains an attribute with the given
    /// [`Identifier`].
    #[must_use]
    pub fn contains_identifier(&self, ident: &str) -> bool {
        self.as_slice().iter().any(|attr| &attr.ident == ident)
    }
}

// -------------------------------------------------------------------------------------------------

/// An attribute of an entity.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub struct EntityAttribute {
    ident: Identifier<'static>,
    value: EntityAttributeValue,
    modifiers: Vec<EntityAttributeModifier>,
    attr_ty: TypeId,
    version_ty: TypeId,
}

impl EntityAttribute {
    /// Create a new [`EntityAttribute`] from the given type.
    #[must_use]
    pub const fn new<T: EntityAttributeType<V> + ?Sized, V: Version>(
        value: EntityAttributeValue,
    ) -> Self {
        Self {
            value,
            ident: T::IDENTIFIER,
            modifiers: Vec::new(),
            attr_ty: TypeId::of::<T>(),
            version_ty: TypeId::of::<V>(),
        }
    }

    /// Get the [`Identifier`] of this attribute.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { &self.ident }

    /// Get the [`EntityAttributeValue`] of this attribute.
    ///
    /// If you want the numeric value of this attribute,
    /// use [`EntityAttribute::value`] instead.
    #[inline]
    #[must_use]
    pub const fn attribute_value(&self) -> &EntityAttributeValue { &self.value }

    /// Get the [`EntityAttributeModifier`]s of this attribute.
    #[inline]
    #[must_use]
    pub const fn modifiers(&self) -> &[EntityAttributeModifier] { self.modifiers.as_slice() }

    /// Insert the given [`EntityAttributeModifier`] into this attribute,
    /// returning the old modifier with the same identifier if present.
    pub fn insert_modifier(
        &mut self,
        modifier: EntityAttributeModifier,
    ) -> Option<EntityAttributeModifier> {
        if let Some(index) = self.modifiers.iter().position(|m| m.identifier == modifier.identifier)
        {
            Some(core::mem::replace(&mut self.modifiers[index], modifier))
        } else {
            self.modifiers.push(modifier);
            None
        }
    }

    /// Remove the modifier with the given identifier from this attribute,
    /// returning it if it was present.
    pub fn remove_modifier(&mut self, ident: &str) -> Option<EntityAttributeModifier> {
        self.modifiers
            .iter()
            .position(|m| &m.identifier == ident)
            .map(|index| self.modifiers.remove(index))
    }

    /// Get the total value of this attribute, including all modifiers.
    #[must_use]
    pub fn value(&self) -> f64 {
        let base = match self.value {
            EntityAttributeValue::Ranged(value) => value.base(),
        };

        self.modifiers().iter().fold(base, |total, modifier| match modifier.modifier_type {
            AttributeModifierType::AddValue => total + modifier.amount,
            AttributeModifierType::AddMultipliedBase => total + (modifier.amount * base),
            AttributeModifierType::AddMultipliedTotal => total * (modifier.amount + 1.0),
        })
    }

    /// Returns `true` if this attribute is of the given type.
    #[inline]
    #[must_use]
    pub fn is_type<T: EntityAttributeType<V>, V: Version>(&self) -> bool {
        self.is_type_id(TypeId::of::<T>())
    }

    /// Returns `true` if this attribute is of the given type.
    #[inline]
    #[must_use]
    pub fn is_type_id(&self, attr_ty: TypeId) -> bool { self.attr_ty == attr_ty }

    /// Returns `true` if this attribute is of the given version.
    #[inline]
    #[must_use]
    pub fn is_version<V: Version>(&self) -> bool { self.is_version_type(TypeId::of::<V>()) }

    /// Returns `true` if this attribute is of the given version.
    #[inline]
    #[must_use]
    pub fn is_version_type(&self, version_ty: TypeId) -> bool { self.version_ty == version_ty }
}

// -------------------------------------------------------------------------------------------------

/// The value of an [`EntityAttribute`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub enum EntityAttributeValue {
    /// A value with a minimum and maximum.
    Ranged(RangedAttributeValue),
}

/// An [`EntityAttribute`] with a minimum and maximum value.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub struct RangedAttributeValue {
    min: f64,
    max: f64,
    base: f64,
}

impl RangedAttributeValue {
    /// Create a new [`RangedAttributeValue`].
    #[inline]
    #[must_use]
    pub const fn new(min: f64, max: f64, base: f64) -> Self { Self { min, max, base } }

    /// Returns the minimum value of this attribute.
    #[inline]
    #[must_use]
    pub const fn min(&self) -> f64 { self.min }

    /// Returns the maximum value of this attribute.
    #[inline]
    #[must_use]
    pub const fn max(&self) -> f64 { self.max }

    /// Returns the base value of this attribute.
    #[inline]
    #[must_use]
    pub const fn base(&self) -> f64 { self.base }

    /// Set the base value of this attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is out of range.
    pub const fn set_base(&mut self, value: f64) -> Result<(), f64> {
        if value < self.min || value > self.max {
            Err(value)
        } else {
            self.base = value;
            Ok(())
        }
    }

    /// Set the base value of this attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is out of range.
    pub const fn with_base(mut self, value: f64) -> Result<Self, f64> {
        match self.set_base(value) {
            Ok(()) => Ok(self),
            Err(value) => Err(value),
        }
    }
}

impl From<RangedAttributeValue> for EntityAttributeValue {
    #[inline]
    fn from(value: RangedAttributeValue) -> Self { Self::Ranged(value) }
}

// -------------------------------------------------------------------------------------------------

/// A modifier to an [`EntityAttribute`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub struct EntityAttributeModifier {
    /// The identifier of this modifier.
    pub identifier: Identifier<'static>,
    /// The amount of this modifier.
    pub amount: f64,
    /// How to apply this modifier to the attribute's base value.
    pub modifier_type: AttributeModifierType,
}

/// How to apply an [`EntityAttributeModifier`] to an attribute's base value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
pub enum AttributeModifierType {
    /// Add the modifier's value to the base value of the attribute.
    AddValue,
    /// Multiply the base value of the attribute by the modifier's value.
    AddMultipliedBase,
    /// Multiply the total value of the attribute by the modifier's value plus
    /// 1.
    AddMultipliedTotal,
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all entity attribute types.
pub trait EntityAttributeType<V: Version>: 'static {
    /// The [`Identifier`] of this attribute type.
    const IDENTIFIER: Identifier<'static>;

    /// The type of value held by this attribute.
    type Value: Into<EntityAttributeValue>;
    /// The default value for this attribute type.
    const DEFAULT: Self::Value;

    /// Create a new [`EntityAttribute`] of this type with the given value.
    #[must_use]
    fn attribute(value: Self::Value) -> EntityAttribute {
        EntityAttribute::new::<Self, V>(value.into())
    }

    /// Create a new [`EntityAttribute`] of this type using the
    /// [`EntityAttributeType::DEFAULT`] value.
    #[inline]
    #[must_use]
    fn default() -> EntityAttribute { Self::attribute(Self::DEFAULT) }
}
