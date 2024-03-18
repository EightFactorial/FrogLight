//! Conversions between different types of [`Connection`]s.

use std::{collections::VecDeque, marker::PhantomData};

use bevy_log::debug;
use froglight_protocol::{
    states::{Configuration, Handshaking, Login, Play, Status},
    traits::{State, Version},
};

use super::{Connection, NetworkDirection};

impl<V: Version, S: State<V>, D: NetworkDirection<V, S>> Connection<V, S, D> {
    /// Forcefully sets a [`Connection`]'s state.
    ///
    /// WARNING: This can cause the [`Connection`] to close if packets
    /// are sent or received that are not valid for the new [`State`].
    #[must_use]
    pub fn into_state<S2: State<V>>(self) -> Connection<V, S2, D>
    where
        D: NetworkDirection<V, S2>,
    {
        Connection {
            stream: self.stream,
            buffer: self.buffer,
            compression: self.compression,
            info: self.info,
            bundle: VecDeque::with_capacity(8),
            _version: PhantomData,
            _state: PhantomData,
            _direction: PhantomData,
        }
    }
}

impl<V: Version, D: NetworkDirection<V, Handshaking>> Connection<V, Handshaking, D>
where
    Handshaking: State<V>,
{
    /// Set the [`Connection`]'s state to [`Status`].
    #[must_use]
    #[inline]
    pub fn status(self) -> Connection<V, Status, D>
    where
        Status: State<V>,
        D: NetworkDirection<V, Status>,
    {
        debug!("Setting `Handshaking` Connection to `Status`");
        self.into_state()
    }

    /// Set the [`Connection`]'s state to [`Login`].
    #[must_use]
    #[inline]
    pub fn login(self) -> Connection<V, Login, D>
    where
        Login: State<V>,
        D: NetworkDirection<V, Login>,
    {
        debug!("Setting `Handshaking` Connection to `Login`");
        self.into_state()
    }
}

impl<V: Version, D: NetworkDirection<V, Login>> Connection<V, Login, D>
where
    Login: State<V>,
{
    /// Set the [`Connection`]'s state to [`Configuration`].
    #[must_use]
    #[inline]
    pub fn configuration(self) -> Connection<V, Configuration, D>
    where
        Configuration: State<V>,
        D: NetworkDirection<V, Configuration>,
    {
        debug!("Setting `Login` Connection to `Configuration`");
        self.into_state()
    }

    /// Set the [`Connection`]'s state to [`Play`].
    #[must_use]
    #[inline]
    pub fn play(self) -> Connection<V, Play, D>
    where
        Play: State<V>,
        D: NetworkDirection<V, Play>,
    {
        debug!("Setting `Login` Connection to `Play`");
        self.into_state()
    }
}

impl<V: Version, D: NetworkDirection<V, Configuration>> Connection<V, Configuration, D>
where
    Configuration: State<V>,
{
    /// Set the [`Connection`]'s state to [`Play`].
    #[must_use]
    #[inline]
    pub fn play(self) -> Connection<V, Play, D>
    where
        Play: State<V>,
        D: NetworkDirection<V, Play>,
    {
        debug!("Setting `Configuration` Connection to `Play`");
        self.into_state()
    }
}

impl<V: Version, D: NetworkDirection<V, Play>> Connection<V, Play, D>
where
    Play: State<V>,
{
    /// Set the [`Connection`]'s state to [`Configuration`].
    #[must_use]
    #[inline]
    pub fn configuration(self) -> Connection<V, Configuration, D>
    where
        Configuration: State<V>,
        D: NetworkDirection<V, Configuration>,
    {
        debug!("Setting `Play` Connection to `Configuration`");
        self.into_state()
    }
}
