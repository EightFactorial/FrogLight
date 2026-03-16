//! TODO
#![allow(non_snake_case, reason = "Limited identifiers inside macro_rules")]

use alloc::boxed::Box;

use bevy_reflect::{
    PartialReflect,
    func::{ArgError, ArgList, args::FromArg},
};

use crate::argument::{ArgumentParseError, ArgumentParser, ArgumentParserObjectExt};

/// A bundle of command arguments.
pub trait ArgumentBundle: Send + Sync {
    /// Convert this bundle into an [`ArgList`].
    fn into_args(self) -> ArgList<'static>;
}

/// A bundle of command argument parsers.
pub trait ArgumentParserBundle: Send + Sync + 'static {
    /// Try to create an [`ArgList`] from the given string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is invalid for this bundle.
    fn try_from_string(&self, input: &str) -> Result<ArgList<'static>, ArgumentParseError>;

    /// Clone this bundle into a boxed trait object.
    fn dyn_clone(&self) -> Box<dyn ArgumentParserBundle>;
}

/// An extension trait for [`ArgumentParserBundle`].
pub trait ArgumentParserBundleExt: ArgumentParserBundle {
    /// The type of arguments produced by this bundle.
    type Arguments: 'static;

    /// Try to create an [`ArgumentBundle`] from the given [`ArgList`].
    ///
    /// # Errors
    ///
    /// Returns an error if the list of arguments is invalid for this bundle.
    fn try_from_args(args: ArgList<'static>) -> Result<Self::Arguments, ArgError>;
}

// -------------------------------------------------------------------------------------------------

impl ArgumentBundle for () {
    fn into_args(self) -> ArgList<'static> { ArgList::new() }
}

impl ArgumentParserBundle for () {
    fn try_from_string(&self, _: &str) -> Result<ArgList<'static>, ArgumentParseError> {
        Ok(ArgList::new())
    }

    fn dyn_clone(&self) -> Box<dyn ArgumentParserBundle> { Box::new(()) }
}

impl ArgumentParserBundleExt for () {
    type Arguments = ();

    fn try_from_args(_: ArgList<'static>) -> Result<Self::Arguments, ArgError> { Ok(()) }
}

// -------------------------------------------------------------------------------------------------

impl<T: PartialReflect + ArgumentParser> ArgumentBundle for T {
    fn into_args(self) -> ArgList<'static> {
        let mut args = ArgList::new();
        args.push_owned(self);
        args
    }
}

impl<T: Clone + PartialReflect + ArgumentParser + 'static> ArgumentParserBundle for T {
    fn try_from_string(&self, input: &str) -> Result<ArgList<'static>, ArgumentParseError> {
        T::parse(input).map(|(arg, _)| arg.into_args())
    }

    fn dyn_clone(&self) -> Box<dyn ArgumentParserBundle> { Box::new(self.clone()) }
}

impl<T: Clone + PartialReflect + ArgumentParser + 'static> ArgumentParserBundleExt for T
where
    T: FromArg<This<'static> = T>,
{
    type Arguments = T;

    fn try_from_args(mut args: ArgList<'static>) -> Result<Self::Arguments, ArgError> {
        args.take::<T>()
    }
}

// -------------------------------------------------------------------------------------------------

macro_rules! impl_bundle {
    ($(#[$meta:meta])* $(($n:tt, $T:ident)),*) => {
        $(#[$meta])*
        #[automatically_derived]
        #[allow(unused_mut, reason = "Macro")]
        impl<$($T: PartialReflect + ArgumentParser),*> ArgumentBundle for ($($T,)*) {
            fn into_args(self) -> ArgList<'static> {
                let mut args = ArgList::new();
                $(
                    args.push_owned(self.$n);
                )*
                args
            }
        }

        $(#[$meta])*
        #[automatically_derived]
        impl<$($T: Clone + ArgumentParserObjectExt),*> ArgumentParserBundle for ($($T,)*) {
            #[allow(unused_mut, reason = "Macro")]
            fn try_from_string(&self, mut input: &str) -> Result<ArgList<'static>, ArgumentParseError> {
                let mut args = ArgList::new();
                $(
                    let (arg, rest) = self.$n.parse_dyn(input)?;
                    args.push_arg(arg);
                    input = rest;
                )*
                Ok(args)
            }

            fn dyn_clone(&self) -> Box<dyn ArgumentParserBundle> {
                Box::new(self.clone())
            }
        }

        $(#[$meta])*
        #[automatically_derived]
        impl<$($T: Clone + ArgumentParserObjectExt),*> ArgumentParserBundleExt for ($($T,)*)
            where
                $($T::Output: FromArg<This<'static> = $T::Output>),*
        {
            type Arguments = ($($T::Output,)*);

            #[allow(unused_mut, reason = "Macro")]
            fn try_from_args(mut args: ArgList<'static>) -> Result<Self::Arguments, ArgError> {
                $(
                    let $T = args.take::<$T::Output>()?;
                )*
                Ok(($($T,)*))
            }
        }
    };
}

variadics_please::all_tuples_enumerated!(impl_bundle, 2, 15, T);
