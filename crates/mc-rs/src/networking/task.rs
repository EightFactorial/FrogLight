#![allow(dead_code)]

use std::{fmt::Debug, marker::PhantomData};

use bevy::{prelude::*, tasks::Task};
use mc_rs_proto::{
    types::enums::ConnectionIntent,
    versions::state::{Configuration, Handshake, Login, Play, Status},
    Connection, ConnectionError, State, Version,
};

use super::request::{PingResponse, StatusResponse};

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
    pub fn new(task: Task<Result<Connection<V, Handshake>, ConnectionError>>) -> Self {
        Self {
            task,
            intent: ConnectionIntent::Login,
        }
    }

    /// Create a new connection task with a connection intent
    pub fn new_with(
        task: Task<Result<Connection<V, Handshake>, ConnectionError>>,
        intent: ConnectionIntent,
    ) -> Self {
        Self { task, intent }
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
pub struct ConnectionLoginTask<V: Version>
where
    Login: State<V>,
{
    #[deref]
    pub task: Task<Result<Connection<V, Login>, ConnectionError>>,
}

impl<V: Version> ConnectionLoginTask<V>
where
    Login: State<V>,
{
    pub fn new(task: Task<Result<Connection<V, Login>, ConnectionError>>) -> Self { Self { task } }

    pub fn task(&self) -> &Task<Result<Connection<V, Login>, ConnectionError>> { &self.task }

    pub fn task_mut(&mut self) -> &mut Task<Result<Connection<V, Login>, ConnectionError>> {
        &mut self.task
    }
}

/// A task that is used to track the configuration state of a connection
#[derive(Debug, Deref, DerefMut, Component)]
pub struct ConnectionConfigurationTask<V: Version>
where
    Configuration: State<V>,
{
    #[deref]
    pub task: Task<Result<Connection<V, Configuration>, ConnectionError>>,
}

impl<V: Version> ConnectionConfigurationTask<V>
where
    Configuration: State<V>,
{
    pub fn new(task: Task<Result<Connection<V, Configuration>, ConnectionError>>) -> Self {
        Self { task }
    }

    pub fn task(&self) -> &Task<Result<Connection<V, Configuration>, ConnectionError>> {
        &self.task
    }

    pub fn task_mut(&mut self) -> &mut Task<Result<Connection<V, Configuration>, ConnectionError>> {
        &mut self.task
    }
}
