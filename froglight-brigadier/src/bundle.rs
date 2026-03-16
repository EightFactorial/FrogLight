//! TODO
#![allow(non_snake_case, reason = "Limited identifiers/patterns inside macro_rules")]
#![allow(unused_mut, unused_parens, reason = "Generated code inside macro_rules")]

use crate::argument::{ArgumentParseError, ArgumentParser};

/// A bundle of arguments that can be parsed from a string.
pub trait ArgumentBundle: Sized + 'static {
    /// Data required to parse the bundle from a string.
    type BundleData: Clone + Send + Sync + Sized + 'static;

    /// Create a new bundle from an input string and some parser data.
    ///
    /// # Errors
    ///
    /// Returns an error if the input string could not be parsed.
    fn bundle_from_string(input: &str, data: &Self::BundleData)
    -> Result<Self, ArgumentParseError>;
}

// -------------------------------------------------------------------------------------------------

impl ArgumentBundle for () {
    type BundleData = ();

    #[inline]
    fn bundle_from_string(_: &str, (): &()) -> Result<Self, ArgumentParseError> { Ok(()) }
}

impl<T: ArgumentParser> ArgumentBundle for T {
    type BundleData = T::Data;

    #[inline]
    fn bundle_from_string(
        input: &str,
        data: &Self::BundleData,
    ) -> Result<Self, ArgumentParseError> {
        let (t, _) = T::parse(input, data)?;
        Ok(t)
    }
}

// -------------------------------------------------------------------------------------------------

macro_rules! impl_argument_bundle {
    ($(($n:tt, $T:ident)),*) => {
        #[automatically_derived]
        impl<$($T: ArgumentParser),*> ArgumentBundle for ($($T),*) {
            type BundleData = ($(<$T as ArgumentParser>::Data),*);

            fn bundle_from_string(mut input: &str, data: &Self::BundleData) -> Result<Self, ArgumentParseError> {
                $(
                    let ($T, rest) = <$T as ArgumentParser>::parse(input, &data.$n)?;
                    if rest.is_empty() {
                        input = rest;
                    } else {
                        input = rest.strip_prefix(' ').ok_or(ArgumentParseError::InputMismatch)?;
                    }
                )*
                Ok(($($T),*))
            }
        }
    };
}

variadics_please::all_tuples_enumerated!(impl_argument_bundle, 2, 15, T);
