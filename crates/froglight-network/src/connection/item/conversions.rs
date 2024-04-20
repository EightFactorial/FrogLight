use std::{collections::VecDeque, marker::PhantomData};

use bevy_log::debug;
use froglight_protocol::{
    states::{Configuration, Handshaking, Login, Play, Status},
    traits::{State, Version},
};

use super::{Connection, NetworkDirection};

impl<V, S, D> Connection<V, S, D>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    /// Sets the state of the connection.
    ///
    /// # Warning
    /// This will cause connection errors if the state unexpectedly changes!
    #[must_use]
    #[inline]
    pub fn set_state<S2: State<V>>(self) -> Connection<V, S2, D>
    where
        D: NetworkDirection<V, S2>,
    {
        Connection {
            stream: self.stream,
            buffer: self.buffer,
            bundle: VecDeque::with_capacity(16),
            compression: self.compression,
            info: self.info,
            account: self.account,
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
        self.set_state()
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
        self.set_state()
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
        self.set_state()
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
        self.set_state()
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
        self.set_state()
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
        self.set_state()
    }
}
