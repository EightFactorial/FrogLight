use crate::prelude::*;

impl<V: ValidState<Handshake>, D: Direction<V, Handshake>> Connection<V, Handshake, D> {
    /// Enter the [`Status`] state.
    #[inline]
    #[must_use]
    pub fn into_status(self) -> Connection<V, Status, D>
    where
        V: ValidState<Status>,
        D: Direction<V, Status>,
    {
        self.into_state_unchecked()
    }

    /// Enter the [`Login`] state.
    #[inline]
    #[must_use]
    pub fn into_login(self) -> Connection<V, Login, D>
    where
        V: ValidState<Login>,
        D: Direction<V, Login>,
    {
        self.into_state_unchecked()
    }
}

impl<V: ValidState<Login>, D: Direction<V, Login>> Connection<V, Login, D> {
    /// Enter the [`Config`] state.
    #[inline]
    #[must_use]
    pub fn into_config(self) -> Connection<V, Config, D>
    where
        V: ValidState<Config>,
        D: Direction<V, Config>,
    {
        self.into_state_unchecked()
    }
}

impl<V: ValidState<Config>, D: Direction<V, Config>> Connection<V, Config, D> {
    /// Enter the [`Play`] state.
    #[inline]
    #[must_use]
    pub fn into_play(self) -> Connection<V, Play, D>
    where
        V: ValidState<Play>,
        D: Direction<V, Play>,
    {
        self.into_state_unchecked()
    }
}

impl<V: ValidState<Play>, D: Direction<V, Play>> Connection<V, Play, D> {
    /// Enter the [`Config`] state.
    #[inline]
    #[must_use]
    pub fn into_config(self) -> Connection<V, Config, D>
    where
        V: ValidState<Config>,
        D: Direction<V, Config>,
    {
        self.into_state_unchecked()
    }
}
