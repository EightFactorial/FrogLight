use std::any::TypeId;

/// A block attribute.
pub trait Attribute: Into<usize> + Copy + Eq + Sized + 'static {
    /// All possible states of the [`Attribute`].
    const STATES: &'static [Self];
}

/// A collection of zero or more [`Attribute`]s.
pub trait BlockAttributes: Sized + 'static {
    /// The types of the [`Attributes`].
    const TYPES: &'static [TypeId];
    /// The total number of combinations of [`Attributes`].
    const COUNT: usize;

    /// Return the [`BlockAttributes`] for the given index.
    ///
    /// # Panics
    /// This function will panic if the index is out of bounds.
    fn from_index(index: usize) -> Self;
    /// Return the index of the [`BlockAttributes`].
    fn into_index(self) -> usize;
}

// Implement for zero attributes
impl BlockAttributes for () {
    const TYPES: &'static [TypeId] = &[];
    const COUNT: usize = 1;
    fn from_index(index: usize) -> Self {
        assert_eq!(index, 0, "Invalid BlockAttributes index!");
    }
    fn into_index(self) -> usize { 0 }
}

// Implement for one attribute
impl<A: Attribute> BlockAttributes for A {
    const TYPES: &'static [TypeId] = &[TypeId::of::<A>()];
    const COUNT: usize = A::STATES.len();
    fn from_index(index: usize) -> Self { A::STATES[index] }
    fn into_index(self) -> usize { self.into() }
}
impl<A: Attribute> BlockAttributes for (A,) {
    const TYPES: &'static [TypeId] = &[TypeId::of::<A>()];
    const COUNT: usize = A::STATES.len();
    fn from_index(index: usize) -> Self { (A::STATES[index],) }
    fn into_index(self) -> usize { self.0.into() }
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
