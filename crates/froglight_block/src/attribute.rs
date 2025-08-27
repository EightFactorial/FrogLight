//! [`BlockAttribute`] and [`BlockAttributes`] implementations.
//!
//! These traits allow getting and setting block states using attribute values
//! or string values.
//!
//! TODO: Example

use core::any::TypeId;

/// An single attribute that makes up a block's state.
pub trait BlockAttribute: Copy + Eq + Sized + 'static {
    /// All possible states of this attribute.
    const STATES: &'static [(&'static str, Self)];

    /// Get the name of the value of this attribute.
    fn attribute_name(&self) -> Option<&'static str> {
        Self::STATES.iter().find_map(|(val, attr)| if self == attr { Some(*val) } else { None })
    }
    /// Get the attribute value for a given state index.
    fn attribute_value(index: usize) -> Option<Self> {
        Self::STATES.get(index).map(|(_, attr)| *attr)
    }

    /// Get the index of this attribute in the states array.
    fn get_attribute_index(&self) -> Option<usize> {
        Self::STATES.iter().position(|(_, attr)| self == attr)
    }
    /// Set the state of this attribute using the given index.
    fn set_attribute_index(&mut self, index: usize) -> Option<usize> {
        if let Some((_, new)) = Self::STATES.get(index) {
            let previous = self.get_attribute_index();
            *self = *new;
            previous
        } else {
            None
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A block's set of attributes.
///
/// Made up of one or more [`BlockAttribute`]s.
pub trait BlockAttributes: Sized {
    /// The total number of states this set of attributes can make.
    const COUNT: usize;

    /// Create a set of attributes from a state index.
    fn from_state(state: usize) -> Option<Self>;
    /// Convert this set of attributes into a state index.
    fn into_index(self) -> Option<usize>;

    /// Get an attribute from this set of attributes.
    fn get_attr<A: BlockAttribute>(&self) -> Option<A>;
    /// Set an attribute in this set of attributes.
    fn set_attr<A: BlockAttribute>(&mut self, attr: A) -> Option<A>;

    /// Get the string attribute value for a given index.
    fn get_attr_str(&self, attr_index: usize) -> Option<&'static str>;
    /// Set the string attribute value for a given index.
    fn set_attr_str(&mut self, attr_index: usize, attr_value: &'static str)
    -> Option<&'static str>;
}

macro_rules! impl_attributes {
    (@from_state $state:expr, $attr:ident) => {
        let $attr = <$attr as BlockAttribute>::attribute_value($state % <$attr as BlockAttribute>::STATES.len());
        $state /= <$attr as BlockAttribute>::STATES.len();
    };
    (@from_state $state:expr, $attr:ident, $($rest:ident),*) => {
        impl_attributes!(@from_state $state, $($rest),*);
        let $attr = <$attr as BlockAttribute>::attribute_value($state % <$attr as BlockAttribute>::STATES.len());
        $state /= <$attr as BlockAttribute>::STATES.len();
    };

    (@into_index $attr:expr) => {
        $attr.get_attribute_index()?
    };
    (@into_index $attr:expr, $($rest:ident),*) => {
        impl_attributes!(@into_index $($rest),*) + $attr.get_attribute_index()? * ($(<$rest as BlockAttribute>::STATES.len() *)* 1)
    };

    (@get_attr_str $index:expr, $val:expr, $attr:ident) => {
        if $index == $val { return $attr.get_attr_str(0); }
    };
    (@get_attr_str $index:expr, $val:expr, $attr:ident, $($rest:ident),*) => {
        if $index == $val { return $attr.get_attr_str(0); }
        impl_attributes!(@get_attr_str $index, $val + 1, $($rest),*)
    };

    (@set_attr_str  $attr_str:ident, $index:expr, $val:expr, $attr:ident) => {
        if $index == $val { return $attr.set_attr_str(0, $attr_str); }
    };
    (@set_attr_str  $attr_str:ident, $index:expr, $val:expr, $attr:ident, $($rest:ident),*) => {
        if $index == $val { return $attr.set_attr_str(0, $attr_str); }
        impl_attributes!(@set_attr_str $attr_str, $index, $val + 1, $($rest),*)
    };


    ($(#[$meta:meta])* $($T:ident),*) => {
        #[expect(non_snake_case, reason = "Reusing generic type names as variable names")]
        impl<$($T: BlockAttribute),*> BlockAttributes for ($($T,)*) {
            const COUNT: usize = 0 $(+ <$T as BlockAttribute>::STATES.len())*;

            #[expect(unused_assignments, reason = "Yes it is")]
            fn from_state(mut state: usize) -> Option<Self> {
                if state >= Self::COUNT {
                    return None;
                }

                impl_attributes!(@from_state state, $($T),*);
                if let ($(Some($T)),*) = ($($T),*) {
                    Some(($($T),*))
                } else {
                    None
                }
            }
            fn into_index(self) -> Option<usize> {
                let ($($T),*) = self;
                Some(impl_attributes!(@into_index $($T),*))
            }

            fn get_attr<A: BlockAttribute>(&self) -> Option<A> { todo!() }
            fn set_attr<A: BlockAttribute>(&mut self, _attr: A) -> Option<A> { todo!() }

            fn get_attr_str(&self, attr_index: usize) -> Option<&'static str> {
                let ($($T),*) = self;
                impl_attributes!(@get_attr_str attr_index, 0, $($T),*);
                None
            }
            fn set_attr_str(&mut self, attr_index: usize, attr_value: &'static str) -> Option<&'static str> {
                let ($($T),*) = self;
                impl_attributes!(@set_attr_str attr_value, attr_index, 0, $($T),*);
                None
            }
        }
    };
}

#[cfg_attr(any(docsrs, docsrs_dep), doc(fake_variadic))]
impl<T0: BlockAttribute> BlockAttributes for (T0,) {
    const COUNT: usize = <T0 as BlockAttributes>::COUNT;

    #[inline]
    fn from_state(state: usize) -> Option<Self> {
        <T0 as BlockAttributes>::from_state(state).map(|attr| (attr,))
    }

    #[inline]
    fn into_index(self) -> Option<usize> { <T0 as BlockAttributes>::into_index(self.0) }

    #[inline]
    fn get_attr<A: BlockAttribute>(&self) -> Option<A> {
        <T0 as BlockAttributes>::get_attr::<A>(&self.0)
    }

    #[inline]
    fn set_attr<A: BlockAttribute>(&mut self, attr: A) -> Option<A> {
        <T0 as BlockAttributes>::set_attr::<A>(&mut self.0, attr)
    }

    #[inline]
    fn get_attr_str(&self, attr_index: usize) -> Option<&'static str> {
        <T0 as BlockAttributes>::get_attr_str(&self.0, attr_index)
    }

    #[inline]
    fn set_attr_str(
        &mut self,
        attr_index: usize,
        attr_value: &'static str,
    ) -> Option<&'static str> {
        <T0 as BlockAttributes>::set_attr_str(&mut self.0, attr_index, attr_value)
    }
}

variadics_please::all_tuples!(impl_attributes, 2, 10, T);

// -------------------------------------------------------------------------------------------------

impl BlockAttributes for () {
    const COUNT: usize = 0;

    #[inline]
    fn from_state(_: usize) -> Option<Self> { None }

    #[inline]
    fn into_index(self) -> Option<usize> { None }

    #[inline]
    fn get_attr<A: BlockAttribute>(&self) -> Option<A> { None }

    #[inline]
    fn set_attr<A: BlockAttribute>(&mut self, _: A) -> Option<A> { None }

    #[inline]
    fn get_attr_str(&self, _: usize) -> Option<&'static str> { None }

    #[inline]
    fn set_attr_str(&mut self, _: usize, _: &'static str) -> Option<&'static str> { None }
}

impl<T0: BlockAttribute> BlockAttributes for T0 {
    const COUNT: usize = <T0 as BlockAttribute>::STATES.len();

    #[inline]
    fn from_state(state: usize) -> Option<Self> { T0::attribute_value(state) }

    #[inline]
    fn into_index(self) -> Option<usize> { self.get_attribute_index() }

    fn get_attr<A: BlockAttribute>(&self) -> Option<A> {
        if TypeId::of::<A>() == TypeId::of::<Self>() {
            self.get_attribute_index().and_then(|index| A::attribute_value(index))
        } else {
            None
        }
    }

    fn set_attr<A: BlockAttribute>(&mut self, attr: A) -> Option<A> {
        if TypeId::of::<A>() == TypeId::of::<Self>() {
            let previous = self.get_attr::<A>()?;
            self.set_attribute_index(attr.get_attribute_index()?)?;
            Some(previous)
        } else {
            None
        }
    }

    #[inline]
    fn get_attr_str(&self, attr_index: usize) -> Option<&'static str> {
        if attr_index == 0 { self.attribute_name() } else { None }
    }

    fn set_attr_str(
        &mut self,
        attr_index: usize,
        attr_value: &'static str,
    ) -> Option<&'static str> {
        if attr_index == 0
            && let Some((_, attr)) =
                <T0 as BlockAttribute>::STATES.iter().find(|(name, _)| *name == attr_value)
        {
            let previous = self.attribute_name()?;
            *self = *attr;
            Some(previous)
        } else {
            None
        }
    }
}
