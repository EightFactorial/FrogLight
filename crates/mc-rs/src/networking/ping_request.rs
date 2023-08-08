use std::{marker::PhantomData, net::SocketAddr};

use async_net::AsyncToSocketAddrs;
use bevy::{
    prelude::*,
    tasks::{IoTaskPool, Task},
};
use futures_lite::future::{block_on, poll_once};
use mc_rs_proto::{ConnectionError, Version};

use super::status_request::StatusResponse;

/// Add Ping request systems to the app
pub(super) fn setup(app: &mut App) {
    app.add_event::<PingResponse>();

    app.add_systems(Update, poll_ping_requests.run_if(any_requests));
}

/// An event that requests the Ping of a server
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct PingRequest<V: Version> {
    pub addr: SocketAddr,
    pub host: String,
    _version: PhantomData<V>,
}

#[allow(dead_code)]
impl<V: Version> PingRequest<V> {
    pub async fn new_task(address: String) -> Task<anyhow::Result<Self>> {
        IoTaskPool::get().spawn(Self::new(address))
    }

    pub async fn new(host: String) -> anyhow::Result<Self> {
        let address = host.clone();
        let mut address = address.as_str();
        if address.starts_with("http://") {
            address = &address[7..];
        } else if address.starts_with("https://") {
            address = &address[8..];
        } else if address.starts_with("tcp://") {
            address = &address[6..];
        }

        if let Some(colon) = address.find(':') {
            let (address, port) = address.split_at(colon);
            let port: u16 = port[1..].parse()?;

            Self::new_from(host, (address, port)).await
        } else {
            Self::new_from(host, (address, 25565)).await
        }
    }

    pub async fn new_from(host: String, addr: impl AsyncToSocketAddrs) -> anyhow::Result<Self> {
        let addr = addr
            .to_socket_addrs()
            .await?
            .next()
            .ok_or_else(|| anyhow::anyhow!("No addresses found"))?;

        Ok(Self::from_sock(host, addr))
    }

    pub fn from_sock(host: String, addr: SocketAddr) -> Self {
        Self {
            addr,
            host,
            _version: PhantomData,
        }
    }
}

/// Check if there are any ping requests
fn any_requests(tasks: Res<PingRequests>) -> bool { !tasks.is_empty() }

/// Poll all running ping requests
fn poll_ping_requests(
    mut tasks: ResMut<PingRequests>,
    mut status_events: EventWriter<StatusResponse>,
    mut ping_events: EventWriter<PingResponse>,
) {
    // Poll all tasks
    let mut finished: Vec<usize> = Vec::new();
    for (index, task) in tasks.iter_mut().enumerate() {
        if let Some(result) = block_on(poll_once(task)) {
            match result {
                Ok((status, ping)) => {
                    status_events.send(status);
                    ping_events.send(ping);
                }
                Err(e) => {
                    error!("Error polling ping request: {}", e);
                }
            }

            finished.push(index);
        }
    }

    // Remove finished tasks
    for index in finished {
        tasks.remove(index).detach();
    }
}

/// A response to a Ping request
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct PingResponse {
    pub host: String,
    pub time: u64,
}

/// A resource that contains a list of ongoing ping requests
#[derive(Debug, Deref, DerefMut, Resource)]
pub struct PingRequests(pub Vec<Task<Result<(StatusResponse, PingResponse), ConnectionError>>>);
