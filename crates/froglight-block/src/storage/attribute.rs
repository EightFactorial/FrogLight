use core::any::TypeId;

/// A block attribute.
pub trait Attribute: Into<usize> + Copy + Eq + Sized + 'static {
    /// All possible states of the [`Attribute`].
    const STATES: &'static [Self];
    /// The string values of the [`Attribute`].
    const VALUES: &'static [&'static str];
}

/// A collection of zero or more [`Attribute`]s.
pub trait BlockAttributes: Sized + 'static {
    /// The types of [`Attribute`]s that make this [`BlockAttributes`].
    const TYPES: &'static [TypeId];
    /// The total number of combinations of [`Attribute`]s.
    const COUNT: usize;

    /// Return the [`BlockAttributes`] for the given index.
    ///
    /// # Panics
    /// This function will panic if the index is out of bounds.
    fn from_index(index: usize) -> Self;
    /// Return the index of the [`BlockAttributes`].
    fn into_index(self) -> usize;

    /// Get the [`Attribute`] of the given type.
    ///
    /// Returns `None` if the [`Attribute`] is not present.
    fn get_attr<T: Attribute>(&self) -> Option<T>;

    /// Get the string value of an [`Attribute`].
    ///
    /// # Panics
    /// This function will panic if the attribute index is out of bounds.
    fn get_attr_str(&self, attr_index: usize) -> &'static str;
    /// Set the string value of an [`Attribute`].
    ///
    /// Returns `None` if the value is invalid.
    ///
    /// # Panics
    /// This function will panic if the attribute index is out of bounds.
    fn set_attr_str(&mut self, attr_index: usize, value: &'static str) -> Option<&'static str>;
}

// Implement for zero attributes
impl BlockAttributes for () {
    const COUNT: usize = 1;
    const TYPES: &'static [TypeId] = &[TypeId::of::<()>()];

    #[inline]
    fn from_index(index: usize) -> Self {
        debug_assert_eq!(index, 0, "Invalid BlockAttributes index!");
    }

    #[inline]
    fn into_index(self) -> usize { 0 }

    #[inline]
    fn get_attr<T: Attribute>(&self) -> Option<T> {
        TypeId::of::<Self>().eq(&TypeId::of::<T>()).then(|| T::STATES[0])
    }

    #[inline]
    fn get_attr_str(&self, attr_index: usize) -> &'static str {
        debug_assert_eq!(attr_index, 0, "Invalid BlockAttributes index!");
        ""
    }

    #[inline]
    fn set_attr_str(&mut self, attr_index: usize, attr_str: &'static str) -> Option<&'static str> {
        debug_assert_eq!(attr_index, 0, "Invalid BlockAttributes index!");
        attr_str.is_empty().then_some("")
    }
}

// Implement for one attribute
impl<A: Attribute> BlockAttributes for A {
    const COUNT: usize = A::STATES.len();
    const TYPES: &'static [TypeId] = &[TypeId::of::<A>()];

    #[inline]
    fn from_index(index: usize) -> Self { A::STATES[index] }

    #[inline]
    fn into_index(self) -> usize { self.into() }

    #[inline]
    fn get_attr<T: Attribute>(&self) -> Option<T> {
        TypeId::of::<Self>().eq(&TypeId::of::<T>()).then(|| T::STATES[Into::<usize>::into(*self)])
    }

    #[inline]
    fn get_attr_str(&self, attr_index: usize) -> &'static str {
        debug_assert_eq!(attr_index, 0, "Invalid BlockAttributes index!");
        A::VALUES[Into::<usize>::into(*self)]
    }

    #[inline]
    fn set_attr_str(&mut self, attr_index: usize, attr_str: &'static str) -> Option<&'static str> {
        debug_assert_eq!(attr_index, 0, "Invalid BlockAttributes index!");
        A::VALUES
            .iter()
            .position(|&val| val == attr_str)
            .map(|new_index| core::mem::replace(self, A::STATES[new_index]).get_attr_str(0))
    }
}
impl<A: Attribute> BlockAttributes for (A,) {
    const COUNT: usize = A::STATES.len();
    const TYPES: &'static [TypeId] = &[TypeId::of::<A>()];

    #[inline]
    fn from_index(index: usize) -> Self { (A::STATES[index],) }

    #[inline]
    fn into_index(self) -> usize { self.0.into() }

    #[inline]
    fn get_attr<T: Attribute>(&self) -> Option<T> {
        TypeId::of::<Self>().eq(&TypeId::of::<T>()).then(|| T::STATES[Into::<usize>::into(self.0)])
    }

    #[inline]
    fn get_attr_str(&self, attr_index: usize) -> &'static str {
        debug_assert_eq!(attr_index, 0, "Invalid BlockAttributes index!");
        A::VALUES[Into::<usize>::into(self.0)]
    }

    #[inline]
    fn set_attr_str(&mut self, attr_index: usize, attr_str: &'static str) -> Option<&'static str> {
        debug_assert_eq!(attr_index, 0, "Invalid BlockAttributes index!");
        A::VALUES
            .iter()
            .position(|&val| val == attr_str)
            .map(|new_index| core::mem::replace(&mut self.0, A::STATES[new_index]).get_attr_str(0))
    }
}

macro_rules! impl_attributes {
    (@from_index $index:tt, $attr:ident) => {
        let $attr = $attr::from_index($index % <$attr>::COUNT);
        $index /= <$attr>::COUNT;
    };
    (@from_index $index:tt, $attr:ident, $($rest:ident),*) => {
        impl_attributes!(@from_index $index, $($rest),*);
        let $attr = $attr::from_index($index % <$attr>::COUNT);
        $index /= <$attr>::COUNT;
    };

    (@to_index $attr:expr) => {
        Into::<usize>::into($attr)
    };
    (@to_index $attr:tt, $($rest:ident),*) => {
        impl_attributes!(@to_index $($rest),*) + Into::<usize>::into($attr) * ($(<$rest>::COUNT *)* 1)
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


    ($($attr:ident),*) => {
        #[allow(non_snake_case, unused_assignments)]
        impl<$($attr: Attribute),*> BlockAttributes for ($($attr),*)
        {
            const TYPES: &'static [TypeId] = &[$(TypeId::of::<$attr>()),*];
            const COUNT: usize = $(<$attr>::COUNT*)* 1;

            fn from_index(mut index: usize) -> Self {
                debug_assert!(index < Self::COUNT, "Invalid BlockAttributes index!");

                impl_attributes!(@from_index index, $($attr),*);
                ($($attr),*)
            }
            fn into_index(self) -> usize {
                let ($($attr),*) = self;
                impl_attributes!(@to_index $($attr),*)
            }

            fn get_attr<T: Attribute>(&self) -> Option<T> {
                let ($($attr),*) = self;
                match TypeId::of::<T>() {
                    $(id if id == TypeId::of::<$attr>() => Some(T::STATES[Into::<usize>::into(*$attr)])),*,
                    _ => None,
                }
            }
            fn get_attr_str(&self, attr_index: usize) -> &'static str {
                let ($($attr),*) = self;
                impl_attributes!(@get_attr_str attr_index, 0, $($attr),*);
                panic!("Invalid BlockAttributes index!");
            }
            fn set_attr_str(&mut self, attr_index: usize, attr_str: &'static str) -> Option<&'static str> {
                let ($($attr),*) = self;
                impl_attributes!(@set_attr_str attr_str, attr_index, 0, $($attr),*);
                panic!("Invalid BlockAttributes index!");
            }
        }
    };
}

// Implement for two to nine attributes
impl_attributes!(A, B);
impl_attributes!(A, B, C);
impl_attributes!(A, B, C, D);
impl_attributes!(A, B, C, D, E);
impl_attributes!(A, B, C, D, E, F);
impl_attributes!(A, B, C, D, E, F, G);
impl_attributes!(A, B, C, D, E, F, G, H);
impl_attributes!(A, B, C, D, E, F, G, H, I);
