use core::any::TypeId;

/// A block attribute.
pub trait BlockAttribute: Copy + Eq + Sized + 'static {
    /// The possible states for this attribute, in order.
    const STATES: &'static [(&'static str, Self)];

    /// Get the name of this attribute state.
    fn to_name(&self) -> &'static str {
        if let Some(name) =
            Self::STATES.iter().find_map(|(n, attr)| if attr == self { Some(*n) } else { None })
        {
            name
        } else {
            unreachable!("BlockAttr state is missing from STATES!");
        }
    }

    /// Get the attribute state from its name.
    ///
    /// Returns `None` if no state with the given name exists.
    fn from_name(name: &str) -> Option<Self> {
        Self::STATES.iter().find_map(|(n, attr)| if *n == name { Some(*attr) } else { None })
    }

    /// Get the index of this attribute state.
    fn to_index(&self) -> usize {
        if let Some(index) = Self::STATES.iter().position(|(_, attr)| attr == self) {
            index
        } else {
            unreachable!("BlockAttr state is missing from STATES!")
        }
    }

    /// Get the attribute state from its index.
    ///
    /// Returns `None` if no state with the given index exists.
    fn from_index(index: usize) -> Option<Self> { Self::STATES.get(index).map(|(_, attr)| *attr) }
}

/// A set of block attributes.
pub trait BlockAttributes: Sized + 'static {
    /// The number of attributes in this set.
    const SIZE: usize;
    /// The total number of possible states.
    const TOTAL: u16;

    /// Get the index of this attribute set.
    fn to_set_index(&self) -> usize;
    /// Get the attribute set from its index.
    fn from_set_index(index: usize) -> Option<Self>;

    /// Get the value of a specific attribute.
    fn get_attr<A: BlockAttribute>(&self) -> Option<A>;
    /// Set the value of a specific attribute.
    fn set_attr<A: BlockAttribute>(&mut self, attr: A) -> Option<A>;

    /// Get the name of an attribute by its index in the set.
    fn get_attr_str(&self, index: usize) -> Option<&'static str>;
    /// Set the value of an attribute by its index in the set.
    fn set_attr_str(&mut self, index: usize, value: &str) -> Option<&'static str>;
}

// -------------------------------------------------------------------------------------------------

impl BlockAttributes for () {
    const SIZE: usize = 1;
    const TOTAL: u16 = 1;

    fn to_set_index(&self) -> usize { 0 }

    fn from_set_index(index: usize) -> Option<Self> { (index == 0).then_some(()) }

    fn get_attr<A: BlockAttribute>(&self) -> Option<A> {
        if TypeId::of::<A>() == TypeId::of::<()>() { A::from_index(0) } else { None }
    }

    fn set_attr<A: BlockAttribute>(&mut self, _: A) -> Option<A> { self.get_attr::<A>() }

    fn get_attr_str(&self, _: usize) -> Option<&'static str> { None }

    fn set_attr_str(&mut self, _: usize, _: &str) -> Option<&'static str> { None }
}

impl<T0: BlockAttribute> BlockAttributes for T0 {
    const SIZE: usize = 1;
    #[allow(
        clippy::cast_possible_truncation,
        reason = "There should never be enough states to overflow a u16"
    )]
    const TOTAL: u16 = T0::STATES.len() as u16;

    fn to_set_index(&self) -> usize { self.to_index() }

    fn from_set_index(index: usize) -> Option<Self> { Self::from_index(index) }

    fn get_attr<A: BlockAttribute>(&self) -> Option<A> {
        if TypeId::of::<Self>() == TypeId::of::<A>() {
            A::from_index(self.to_index())
        } else {
            None
        }
    }

    fn set_attr<A: BlockAttribute>(&mut self, attr: A) -> Option<A> {
        if TypeId::of::<Self>() == TypeId::of::<A>()
            && let Some(new) = Self::from_index(attr.to_index())
            && let Some(old) = A::from_index(self.to_index())
        {
            *self = new;
            Some(old)
        } else {
            None
        }
    }

    fn get_attr_str(&self, index: usize) -> Option<&'static str> {
        (index == 0).then(|| self.to_name())
    }

    fn set_attr_str(&mut self, index: usize, name: &str) -> Option<&'static str> {
        if index == 0
            && let Some(new) = Self::from_name(name)
        {
            let old = self.to_name();
            *self = new;
            Some(old)
        } else {
            None
        }
    }
}

impl<T0: BlockAttributes> BlockAttributes for (T0,) {
    const SIZE: usize = 1;
    const TOTAL: u16 = T0::TOTAL;

    #[inline]
    fn to_set_index(&self) -> usize { self.0.to_set_index() }

    #[inline]
    fn from_set_index(index: usize) -> Option<Self> { T0::from_set_index(index).map(|t0| (t0,)) }

    #[inline]
    fn get_attr<A: BlockAttribute>(&self) -> Option<A> { self.0.get_attr() }

    #[inline]
    fn set_attr<A: BlockAttribute>(&mut self, attr: A) -> Option<A> { self.0.set_attr::<A>(attr) }

    #[inline]
    fn get_attr_str(&self, index: usize) -> Option<&'static str> { self.0.get_attr_str(index) }

    #[inline]
    fn set_attr_str(&mut self, index: usize, value: &str) -> Option<&'static str> {
        self.0.set_attr_str(index, value)
    }
}

// -------------------------------------------------------------------------------------------------

macro_rules! implement {
    (@count $token:tt, $($rest:tt)*) => {
        1 + implement!(@count $($rest)*)
    };
    (@count $token:tt) => {
        1
    };

    ($($T:ident),*) => {
        #[allow(non_snake_case, unused_assignments, reason = "Macro expansion")]
        impl<$($T: BlockAttribute),*> BlockAttributes for ($($T,)*) {
            const SIZE: usize = 0 + implement!(@count $($T),*) ;
            #[allow(clippy::cast_possible_truncation, reason = "There should never be enough states to overflow a u16")]
            const TOTAL: u16 = 1 $( * $T::STATES.len() as u16 )* ;

            fn to_set_index(&self) -> usize {
                let ($($T),*) = self;
                let mut index = 0;
                let mut multiplier = 1;
                $(
                    index += $T.to_index() * multiplier;
                    multiplier *= $T::STATES.len();
                )*
                index
            }
            fn from_set_index(index: usize) -> Option<Self> {
                if index >= usize::from(Self::TOTAL) {
                     None
                } else {
                    let mut rem = index;

                    $(
                        let $T = {
                            let len = $T::STATES.len();
                            let attr_index = rem % len;
                            rem /= len;
                            $T::from_index(attr_index)?
                        };
                    )*

                    Some( ($($T,)*) )
                }
            }

            fn get_attr<A: BlockAttribute>(&self) -> Option<A> {
                let ($($T),*) = self;
                $(
                    if TypeId::of::<A>() == TypeId::of::<$T>() {
                        return A::from_index($T.to_index());
                    }
                )*
                None
            }
            fn set_attr<A: BlockAttribute>(&mut self, attr: A) -> Option<A> {
                let ($($T),*) = self;
                $(
                    if TypeId::of::<A>() == TypeId::of::<$T>() {
                        let old = A::from_index($T.to_index())?;
                        let new = $T::from_index(attr.to_index())?;
                        *$T = new;
                        return Some(old);
                    }
                )*
                None
            }

            fn get_attr_str(&self, mut index: usize) -> Option<&'static str> {
                let ($($T),*) = self;
                $(
                    if index == 0 {
                        return Some($T.to_name());
                    }
                    index = index.saturating_sub(1);
                )*
                None
            }

            fn set_attr_str(&mut self, mut index: usize, value: &str) -> Option<&'static str> {
                let ($($T),*) = self;
                $(
                    if index == 0 {
                        let old_name = $T.to_name();
                        let new_attr = $T::from_name(value)?;
                        *$T = new_attr;
                        return Some(old_name);
                    }
                    index = index.saturating_sub(1);
                )*
                None
            }
        }
    };
}

variadics_please::all_tuples!(implement, 2, 10, T);
