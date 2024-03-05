//! This module is solely for converting between different types of
//! [`Connection`]s.

use bevy_log::debug;
use froglight_protocol::{
    states::{Configuration, Handshaking, Login, Play, Status},
    traits::{State, Version},
};

use super::{Connection, Direction};

impl<V: Version, S: State<V>, D: Direction<V, S>> Connection<V, S, D> {
    /// Forcefully sets a [`Connection`]'s state.
    ///
    /// WARNING: This can cause the [`Connection`] to close if packets
    /// are sent or received that are not valid for the new [`State`].
    #[must_use]
    pub fn set_state<S2: State<V>>(self) -> Connection<V, S2, D>
    where
        D: Direction<V, S2>,
    {
        todo!()
    }
}

impl<V: Version, D: Direction<V, Handshaking>> Connection<V, Handshaking, D>
where
    Handshaking: State<V>,
{
    /// Set the [`Connection`]'s state to [`Status`].
    #[must_use]
    #[inline]
    pub fn status(self) -> Connection<V, Status, D>
    where
        Status: State<V>,
        D: Direction<V, Status>,
    {
        debug!("Setting Connection state to `Status`");
        self.set_state()
    }

    /// Set the [`Connection`]'s state to [`Login`].
    #[must_use]
    #[inline]
    pub fn login(self) -> Connection<V, Login, D>
    where
        Login: State<V>,
        D: Direction<V, Login>,
    {
        debug!("Setting Connection state to `Login`");
        self.set_state()
    }
}

impl<V: Version, D: Direction<V, Login>> Connection<V, Login, D>
where
    Login: State<V>,
{
    /// Set the [`Connection`]'s state to [`Configuration`].
    #[must_use]
    #[inline]
    pub fn configuration(self) -> Connection<V, Configuration, D>
    where
        Configuration: State<V>,
        D: Direction<V, Configuration>,
    {
        debug!("Setting Connection state to `Configuration`");
        self.set_state()
    }

    /// Set the [`Connection`]'s state to [`Play`].
    #[must_use]
    #[inline]
    pub fn play(self) -> Connection<V, Play, D>
    where
        Play: State<V>,
        D: Direction<V, Play>,
    {
        debug!("Setting Connection state to `Play`");
        self.set_state()
    }
}

impl<V: Version, D: Direction<V, Configuration>> Connection<V, Configuration, D>
where
    Configuration: State<V>,
{
    /// Set the [`Connection`]'s state to [`Play`].
    #[must_use]
    #[inline]
    pub fn play(self) -> Connection<V, Play, D>
    where
        Play: State<V>,
        D: Direction<V, Play>,
    {
        debug!("Setting Connection state to `Play`");
        self.set_state()
    }
}
