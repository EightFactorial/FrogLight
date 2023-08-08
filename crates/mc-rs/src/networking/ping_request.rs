use std::{marker::PhantomData, net::SocketAddr};

use async_net::AsyncToSocketAddrs;
use bevy::{prelude::*, tasks::Task};
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
    _version: PhantomData<V>,
}

#[allow(dead_code)]
impl<V: Version> PingRequest<V> {
    pub async fn new(mut address: &str) -> anyhow::Result<Self> {
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

            Self::new_from((address, port)).await
        } else {
            Self::new_from((address, 25565)).await
        }
    }

    pub async fn new_from(sock: impl AsyncToSocketAddrs) -> anyhow::Result<Self> {
        let addr = sock
            .to_socket_addrs()
            .await?
            .next()
            .ok_or_else(|| anyhow::anyhow!("No addresses found"))?;

        Ok(Self::from_sock(addr))
    }

    pub fn from_sock(addr: SocketAddr) -> Self {
        Self {
            addr,
            _version: PhantomData,
        }
    }
}

/// Check if there are any Ping requests
fn any_requests(tasks: Res<PingRequests>) -> bool { !tasks.is_empty() }

/// Poll all running Ping requests
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
                    error!("Error polling Ping request: {}", e);
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
pub struct PingResponse(pub u64);

/// A resource that contains a list of ongoing ping requests
#[derive(Debug, Deref, DerefMut, Resource)]
pub struct PingRequests(pub Vec<Task<Result<(StatusResponse, PingResponse), ConnectionError>>>);
