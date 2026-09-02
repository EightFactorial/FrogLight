//! TODO
#![allow(non_snake_case, reason = "Limited identifiers/patterns inside macro_rules")]
#![allow(unused_mut, unused_parens, reason = "Generated code inside macro_rules")]

use alloc::{borrow::Cow, sync::Arc};

use bevy_ecs::{entity::Entity, system::SystemId, world::World};

use crate::{
    argument::{ArgumentParseError, ArgumentParser},
    commandset::CommandExecuteError,
    context::GameCommandCtx,
};

/// A bundle of arguments that can be parsed from a string.
pub trait ArgumentBundle: Sized + 'static {
    /// Data required to parse the bundle from a string.
    type BundleData: Send + Sync + 'static;

    /// Create a new bundle from an input string and some parser data.
    ///
    /// # Errors
    ///
    /// Returns an error if the input string could not be parsed.
    fn bundle_from_string<'a>(
        arguments: &'a str,
        settings: &Self::BundleData,
    ) -> Result<Self, ArgumentParseError<'a>>;
}

// -------------------------------------------------------------------------------------------------

impl ArgumentBundle for () {
    type BundleData = ();

    #[inline]
    fn bundle_from_string<'a>(input: &'a str, (): &()) -> Result<Self, ArgumentParseError<'a>> {
        if input.is_empty() {
            Ok(())
        } else {
            Err(ArgumentParseError::ExtraInput(Cow::Borrowed(input)))
        }
    }
}

impl<T: ArgumentParser> ArgumentBundle for T {
    type BundleData = T::Data;

    #[inline]
    fn bundle_from_string<'a>(
        input: &'a str,
        settings: &Self::BundleData,
    ) -> Result<Self, ArgumentParseError<'a>> {
        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_brigadier", "Parsing Argument 0: {input:?}");

        let (t, rem) = T::parse(input, settings)?;
        if rem.is_empty() { Ok(t) } else { Err(ArgumentParseError::ExtraInput(Cow::Borrowed(rem))) }
    }
}

// -------------------------------------------------------------------------------------------------

macro_rules! impl_argument_bundle {
    ($(($n:tt, $T:ident)),*) => {
        #[automatically_derived]
        impl<$($T: ArgumentParser),*> ArgumentBundle for ($($T),*) {
            type BundleData = ($(<$T as ArgumentParser>::Data),*);

            fn bundle_from_string<'a >(mut input: &'a str, data: &Self::BundleData) -> Result<Self, ArgumentParseError<'a>> {
                $(
                    #[cfg(feature = "tracing")]
                    tracing::trace!(target: "froglight_brigadier", "Parsing Argument {}: {input:?}", $n);

                    let ($T, rest) = <$T as ArgumentParser>::parse(input, &data.$n)?;
                    if rest.is_empty() {
                        input = rest;
                    } else {
                        input = rest.strip_prefix(' ').ok_or(ArgumentParseError::InputMismatch)?;
                    }
                )*

                if input.is_empty() {
                    Ok(($($T),*))
                } else {
                    Err(ArgumentParseError::ExtraInput(Cow::Borrowed(input)))
                }
            }
        }
    };
}

variadics_please::all_tuples_enumerated!(impl_argument_bundle, 2, 15, T);

// -------------------------------------------------------------------------------------------------

/// A dyn-compatible trait for parsing [`ArgumentBundle`]s and calling
/// [`System`](bevy_ecs::system::System)s.
pub(super) trait ExecutableParser: Send + Sync + 'static {
    fn system_runner<'a>(
        &self,
        caller: Entity,
        arguments: &'a str,
        system_id: Entity,
        world: &mut World,
    ) -> Result<(), CommandExecuteError<'a>>;
}

/// A transparent wrapper over an [`ArgumentBundle`].
#[repr(transparent)]
pub(super) struct BundleWrapper<B: ArgumentBundle> {
    settings: B::BundleData,
}

impl<B: ArgumentBundle> BundleWrapper<B> {
    /// Create a new, dynamic [`ExecutableParser`].
    #[inline]
    #[must_use]
    pub(super) fn new_executor(settings: B::BundleData) -> Arc<dyn ExecutableParser> {
        Arc::new(Self { settings })
    }
}

impl<B: ArgumentBundle> ExecutableParser for BundleWrapper<B> {
    #[inline]
    fn system_runner<'a>(
        &self,
        caller: Entity,
        arguments: &'a str,
        system_id: Entity,
        world: &mut World,
    ) -> Result<(), CommandExecuteError<'a>> {
        // Parse the `ArgumentBundle` from the command.
        let input: B =
            B::bundle_from_string(arguments, &self.settings).map_err(CommandExecuteError::Parse)?;

        // Run the system with the caller and argument bundle.
        let system_id = SystemId::<GameCommandCtx<B>, ()>::from_entity(system_id);
        world.run_system_with(system_id, (caller, input)).map_err(CommandExecuteError::execute)
    }
}
