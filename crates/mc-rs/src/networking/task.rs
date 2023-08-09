#![allow(clippy::type_complexity)]
#![allow(dead_code)]

use std::{
    fmt::Debug,
    marker::PhantomData,
    sync::{RwLock, RwLockReadGuard},
};

use bevy::{prelude::*, tasks::Task};
use flume::{Receiver, Sender};
use mc_rs_proto::{
    types::{enums::ConnectionIntent, GameProfile},
    versions::state::{Configuration, Handshake, Login, Play, Status},
    Connection, ConnectionError, State, Version,
};

use super::{
    handle::{ConnectionData, ConnectionSend, ConnectionState},
    request::{PingResponse, StatusResponse},
};

/// A task that is used to establish a connection with a server
#[derive(Debug, Deref, DerefMut, Component)]
pub struct ConnectionTask<V: Version>
where
    Handshake: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    #[deref]
    pub task: Task<Result<Connection<V, Handshake>, ConnectionError>>,
    pub hostname: String,
    pub intent: ConnectionIntent,
}

#[allow(dead_code)]
impl<V: Version> ConnectionTask<V>
where
    Handshake: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    /// Create a new connection task
    pub fn new(
        task: Task<Result<Connection<V, Handshake>, ConnectionError>>,
        hostname: String,
    ) -> Self {
        Self {
            task,
            hostname,
            intent: ConnectionIntent::Login,
        }
    }

    /// Create a new connection task with a connection intent
    pub fn new_with(
        task: Task<Result<Connection<V, Handshake>, ConnectionError>>,
        hostname: String,
        intent: ConnectionIntent,
    ) -> Self {
        Self {
            task,
            hostname,
            intent,
        }
    }

    /// Get the connection task
    pub fn task(&self) -> &Task<Result<Connection<V, Handshake>, ConnectionError>> { &self.task }

    /// Get the connection task mutably
    pub fn task_mut(&mut self) -> &mut Task<Result<Connection<V, Handshake>, ConnectionError>> {
        &mut self.task
    }
}

/// A task that is used to track the handshake state of a connection
#[derive(Debug, Deref, DerefMut, Component)]
pub struct ConnectionHandshakeTask<V: Version>
where
    Handshake: State<V>,
{
    #[deref]
    pub task: Task<Result<Connection<V, Handshake>, ConnectionError>>,
    pub intent: ConnectionIntent,
}

impl<V: Version> ConnectionHandshakeTask<V>
where
    Handshake: State<V>,
{
    pub fn new(
        task: Task<Result<Connection<V, Handshake>, ConnectionError>>,
        intent: ConnectionIntent,
    ) -> Self {
        Self { task, intent }
    }

    pub fn task(&self) -> &Task<Result<Connection<V, Handshake>, ConnectionError>> { &self.task }

    pub fn task_mut(&mut self) -> &mut Task<Result<Connection<V, Handshake>, ConnectionError>> {
        &mut self.task
    }
}

/// A task that is used to track the status state of a connection
#[derive(Debug, Deref, DerefMut, Component)]
pub struct ConnectionStatusTask<V: Version>
where
    Status: State<V>,
{
    #[deref]
    pub task: Task<Result<(StatusResponse, PingResponse), ConnectionError>>,
    _version: PhantomData<V>,
}

impl<V: Version> ConnectionStatusTask<V>
where
    Status: State<V>,
{
    pub fn new(task: Task<Result<(StatusResponse, PingResponse), ConnectionError>>) -> Self {
        Self {
            task,
            _version: PhantomData,
        }
    }

    pub fn task(&self) -> &Task<Result<(StatusResponse, PingResponse), ConnectionError>> {
        &self.task
    }

    pub fn task_mut(
        &mut self,
    ) -> &mut Task<Result<(StatusResponse, PingResponse), ConnectionError>> {
        &mut self.task
    }
}

/// A task that is used to track the login state of a connection
#[derive(Debug, Deref, DerefMut, Component)]
pub struct ConnectionLoginTask<V: Version>(
    pub Task<Result<(Connection<V, Login>, GameProfile), ConnectionError>>,
)
where
    Login: State<V>;

impl<V: Version> ConnectionLoginTask<V>
where
    Login: State<V>,
{
    pub fn new(task: Task<Result<(Connection<V, Login>, GameProfile), ConnectionError>>) -> Self {
        Self(task)
    }

    pub fn task(&self) -> &Task<Result<(Connection<V, Login>, GameProfile), ConnectionError>> {
        &self.0
    }

    pub fn task_mut(
        &mut self,
    ) -> &mut Task<Result<(Connection<V, Login>, GameProfile), ConnectionError>> {
        &mut self.0
    }
}

/// A task that is used to track the configuration state of a connection
#[derive(Debug, Deref, DerefMut, Component)]
pub struct ConnectionConfigurationTask<V: Version>(
    pub Task<Result<Connection<V, Configuration>, ConnectionError>>,
)
where
    Configuration: State<V>;

impl<V: Version> ConnectionConfigurationTask<V>
where
    Configuration: State<V>,
{
    pub fn new(task: Task<Result<Connection<V, Configuration>, ConnectionError>>) -> Self {
        Self(task)
    }

    pub fn task(&self) -> &Task<Result<Connection<V, Configuration>, ConnectionError>> { &self.0 }

    pub fn task_mut(&mut self) -> &mut Task<Result<Connection<V, Configuration>, ConnectionError>> {
        &mut self.0
    }
}

/// A task that is used to track the play state of a connection
#[derive(Debug, Deref, DerefMut, Component)]
pub struct ConnectionPlayTask<V: Version>
where
    Configuration: State<V>,
    Play: State<V>,
{
    #[deref]
    pub rx: Receiver<Result<ConnectionData<V>, ConnectionError>>,
    pub tx: Sender<ConnectionSend<V>>,
    pub state: RwLock<ConnectionState>,
    task: Task<()>,
    _version: PhantomData<V>,
}

impl<V: Version> ConnectionPlayTask<V>
where
    Configuration: State<V>,
    Play: State<V>,
{
    pub fn new(
        rx: Receiver<Result<ConnectionData<V>, ConnectionError>>,
        tx: Sender<ConnectionSend<V>>,
        task: Task<()>,
    ) -> Self {
        Self {
            rx,
            tx,
            task,
            state: RwLock::new(ConnectionState::Play),
            _version: PhantomData,
        }
    }

    pub fn state(&self) -> RwLockReadGuard<ConnectionState> { self.state.read().unwrap() }

    pub fn task(&self) -> &Task<()> { &self.task }

    pub fn task_mut(&mut self) -> &mut Task<()> { &mut self.task }
}
