//! Resolve a tuple of block attributes into an index and vice versa.
//!
//! For example, given the following attributes:
//!   - `BooleanAttribute` with states `true` and `false`
//!   - `EnumAttribute` with states `A`, `B`, and `C`
//!
//! There are the following combinations:
//!  - `(true, A)`
//!  - `(true, B)`
//!  - `(true, C)`
//!  - `(false, A)`
//!  - `(false, B)`
//!  - `(false, C)`
#![allow(dead_code, non_snake_case, unused_assignments)]

use std::any::TypeId;

use super::BlockAttribute;

#[cfg(test)]
mod test;

/// A set of attributes that can be resolved into an index and vice versa.
pub(crate) trait ResolvableAttributes: Sized {
    const TYPES: &'static [TypeId];
    const STATE_COUNT: usize;

    /// Index into all possible combinations of attributes.
    fn from_index(index: usize) -> Option<Self>;
    /// Convert a tuple of attributes into an index.
    fn to_index(&self) -> usize;
}

// Implement for unit, single attributes, and tuples of 1
impl ResolvableAttributes for () {
    const TYPES: &'static [TypeId] = &[];
    const STATE_COUNT: usize = 1;
    #[inline]
    fn from_index(index: usize) -> Option<Self> {
        if index == 0 {
            Some(())
        } else {
            None
        }
    }
    #[inline]
    fn to_index(&self) -> usize { 0 }
}
impl<A: BlockAttribute> ResolvableAttributes for A {
    const TYPES: &'static [TypeId] = &[TypeId::of::<A>()];
    const STATE_COUNT: usize = A::STATE_COUNT;
    #[inline]
    fn from_index(index: usize) -> Option<Self> { A::STATES.get(index).copied() }
    #[inline]
    fn to_index(&self) -> usize { Into::<usize>::into(*self) }
}
impl<A: BlockAttribute> ResolvableAttributes for (A,) {
    const TYPES: &'static [TypeId] = &[TypeId::of::<A>()];
    const STATE_COUNT: usize = A::STATE_COUNT;
    #[inline]
    fn from_index(index: usize) -> Option<Self> { A::STATES.get(index).map(|attr| (*attr,)) }
    #[inline]
    fn to_index(&self) -> usize { Into::<usize>::into(self.0) }
}

/// A macro for automatically implementing
/// `ResolvableAttributes` for tuples of attributes.
macro_rules! impl_resolvable {
    (@from_index $index:tt, $attr:ident) => {
        let $attr = $attr::from_index($index % <$attr>::STATE_COUNT)?;
        $index /= <$attr>::STATE_COUNT;
    };
    (@from_index $index:tt, $attr:ident, $($rest:ident),*) => {
        impl_resolvable!(@from_index $index, $($rest),*);
        let $attr = $attr::from_index($index % <$attr>::STATE_COUNT)?;
        $index /= <$attr>::STATE_COUNT;
    };

    (@to_index $attr:expr) => {
        Into::<usize>::into(*$attr)
    };
    (@to_index $attr:tt, $($rest:ident),*) => {
        impl_resolvable!(@to_index $($rest),*) + Into::<usize>::into(*$attr) * ($(<$rest>::STATE_COUNT *)* 1)
    };

    ($($attr:ident),*) => {
        impl<$($attr: BlockAttribute),*> ResolvableAttributes for ($($attr),*)
        {
            const TYPES: &'static [TypeId] = &[$(TypeId::of::<$attr>()),*];
            const STATE_COUNT: usize = $(<$attr>::STATE_COUNT*)* 1;

            fn from_index(mut index: usize) -> Option<Self> {
                if index >= Self::STATE_COUNT {
                    return None;
                }

                impl_resolvable!(@from_index index, $($attr),*);
                Some(($($attr),*))
            }

            fn to_index(&self) -> usize {
                let ($($attr),*) = self;
                impl_resolvable!(@to_index $($attr),*)
            }
        }
    };
}

// Implement for tuples with 2 to 8 attributes
impl_resolvable!(A, B);
impl_resolvable!(A, B, C);
impl_resolvable!(A, B, C, D);
impl_resolvable!(A, B, C, D, E);
impl_resolvable!(A, B, C, D, E, F);
impl_resolvable!(A, B, C, D, E, F, G);
impl_resolvable!(A, B, C, D, E, F, G, H);
