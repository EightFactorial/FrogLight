//! TODO

use alloc::{borrow::Cow, boxed::Box, string::String};

use bevy_ecs::prelude::*;
use bevy_reflect::{
    func::{ArgList, ArgValue},
    prelude::*,
};

// use crate::prelude::*;

/// A command sent by an [`Entity`].
#[derive(Debug, Reflect, EntityEvent)]
#[reflect(Debug, Event)]
pub struct GameCommand {
    /// The [`Entity`] that sent the command.
    entity: Entity,
    /// The command that was sent, without the leading slash.
    command: Cow<'static, str>,
    /// The list of parsed arguments.
    #[reflect(ignore)]
    arguments: Option<ArgList<'static>>,
}

impl GameCommand {
    /// Create a new [`GameCommand`] with the given [`Entity`] and command.
    ///
    /// # Note
    ///
    /// The command should not include the leading slash.
    #[must_use]
    pub fn new<T: Into<Cow<'static, str>>>(entity: Entity, command: String) -> Self {
        Self::new_with(entity, command, None)
    }

    /// Create a new [`GameCommand`] with the given [`Entity`], command, and
    /// list of parsed arguments.
    ///
    /// # Note
    ///
    /// The command should not include the leading slash.
    #[must_use]
    pub fn new_with<T: Into<Cow<'static, str>>>(
        entity: Entity,
        command: T,
        arguments: Option<ArgList<'static>>,
    ) -> Self {
        Self { entity, command: command.into(), arguments }
    }

    /// Get the [`Entity`] that sent the command.
    #[inline]
    #[must_use]
    pub const fn entity(&self) -> Entity { self.entity }

    /// Get the command that was sent, without the leading slash.
    #[inline]
    #[must_use]
    pub const fn command(&self) -> &str {
        match &self.command {
            Cow::Borrowed(s) => s,
            Cow::Owned(s) => s.as_str(),
        }
    }

    /// Get a reference to the inner list of parsed arguments.
    #[inline]
    #[must_use]
    pub const fn arguments(&self) -> Option<&ArgList<'static>> { self.arguments.as_ref() }

    /// Get a mutable reference to the inner list of parsed arguments.
    #[inline]
    #[must_use]
    pub const fn arguments_mut(&mut self) -> Option<&mut ArgList<'static>> {
        self.arguments.as_mut()
    }

    /// Adds an argument to the command.
    #[inline]
    #[must_use]
    pub fn with_argument<T: PartialReflect>(mut self, argument: T) -> Self {
        self.push_argument::<T>(argument);
        self
    }

    /// Adds an argument to the command.
    #[inline]
    pub fn push_argument<T: PartialReflect>(&mut self, argument: T) -> &mut Self {
        self.push_value(ArgValue::Owned(Box::new(argument)))
    }

    /// Adds a [`Box<dyn PartialReflect>`] argument to the command.
    #[inline]
    pub fn push_reflect(&mut self, argument: Box<dyn PartialReflect>) -> &mut Self {
        self.push_value(ArgValue::Owned(argument))
    }

    /// Adds an [`ArgValue`] to the command.
    #[expect(clippy::missing_panics_doc, reason = "Cannot panic")]
    pub fn push_value(&mut self, argument: ArgValue<'static>) -> &mut Self {
        if self.arguments().is_none() {
            self.arguments = Some(ArgList::new());
        }
        self.arguments.as_mut().unwrap().push_arg(argument);
        self
    }

    /// Takes the inner list of parsed arguments, leaving `None` in its place.
    ///
    /// Returns `None` if there was no argument list to take.
    #[inline]
    #[must_use]
    pub fn take_arguments(&mut self) -> Option<ArgList<'static>> { self.arguments.take() }

    /// Returns the inner list of parsed arguments.
    #[inline]
    #[must_use]
    pub fn into_arguments(self) -> Option<ArgList<'static>> { self.arguments }

    /// Returns the command and the inner list of parsed arguments.
    #[inline]
    #[must_use]
    pub fn into_parts(self) -> (Cow<'static, str>, Option<ArgList<'static>>) {
        (self.command, self.arguments)
    }
}
