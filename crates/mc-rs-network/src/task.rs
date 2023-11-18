#![allow(clippy::type_complexity)]

use std::{fmt::Debug, marker::PhantomData};

use bevy::{prelude::*, tasks::Task};
use compact_str::CompactString;
use flume::{Receiver, Sender};
use mc_rs_core::{PingResponse, StatusResponse};
use mc_rs_protocol::{
    types::{enums::ConnectionIntent, GameProfile},
    versions::state::{Configuration, Handshake, Login, Play, Status},
    Connection, ConnectionError, State, Version,
};

use super::handle::{ConnectionData, ConnectionSend, ConnectionState};

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
    pub hostname: CompactString,
    pub intent: ConnectionIntent,
}

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
        hostname: CompactString,
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
        hostname: CompactString,
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

/// A resource that communicates with the connection task
#[derive(Debug, Deref, DerefMut, Resource)]
pub struct ConnectionChannel<V: Version>
where
    Configuration: State<V>,
    Play: State<V>,
{
    #[deref]
    pub rx: Receiver<Result<ConnectionData<V>, ConnectionError>>,
    pub tx: Sender<ConnectionSend<V>>,
    pub state: ConnectionState,
    task: Task<()>,
    _version: PhantomData<V>,
}

impl<V: Version> ConnectionChannel<V>
where
    Configuration: State<V>,
    Play: State<V>,
{
    pub fn new(
        rx: Receiver<Result<ConnectionData<V>, ConnectionError>>,
        tx: Sender<ConnectionSend<V>>,
        state: ConnectionState,
        task: Task<()>,
    ) -> Self {
        Self {
            rx,
            tx,
            task,
            state,
            _version: PhantomData,
        }
    }

    pub fn new_config(
        rx: Receiver<Result<ConnectionData<V>, ConnectionError>>,
        tx: Sender<ConnectionSend<V>>,
        task: Task<()>,
    ) -> Self {
        Self {
            rx,
            tx,
            task,
            state: ConnectionState::Configuration,
            _version: PhantomData,
        }
    }

    pub fn new_play(
        rx: Receiver<Result<ConnectionData<V>, ConnectionError>>,
        tx: Sender<ConnectionSend<V>>,
        task: Task<()>,
    ) -> Self {
        Self {
            rx,
            tx,
            task,
            state: ConnectionState::Play,
            _version: PhantomData,
        }
    }

    pub fn task(&self) -> &Task<()> { &self.task }

    pub fn task_mut(&mut self) -> &mut Task<()> { &mut self.task }

    pub fn send_config(&mut self, packet: impl Into<<Configuration as State<V>>::Serverbound>) {
        if let Err(err) = self.tx.send(ConnectionSend::Configuration(packet.into())) {
            error!("Failed to send configuration packet: {err}");
        }
    }

    pub fn send_play(&mut self, packet: impl Into<<Play as State<V>>::Serverbound>) {
        if let Err(err) = self.tx.send(ConnectionSend::Play(packet.into())) {
            error!("Failed to send play packet: {err}");
        }
    }
}
