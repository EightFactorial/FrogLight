use std::{marker::PhantomData, net::SocketAddr};

use async_net::AsyncToSocketAddrs;
use bevy::{prelude::*, tasks::Task, utils::HashMap};
use futures_lite::future::{block_on, poll_once};
use mc_rs_proto::{ConnectionError, Version};
use uuid::Uuid;

/// Add status request systems to the app
pub(super) fn setup(app: &mut App) {
    app.add_event::<StatusResponse>();

    app.add_systems(Update, poll_status_requests.run_if(any_requests));
}

/// An event that requests the status of a server
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct StatusRequest<V: Version> {
    pub addr: SocketAddr,
    _version: PhantomData<V>,
}

#[allow(dead_code)]
impl<V: Version> StatusRequest<V> {
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

/// Check if there are any status requests
fn any_requests(tasks: Res<StatusRequests>) -> bool { !tasks.is_empty() }

/// Poll all running status requests
fn poll_status_requests(
    mut tasks: ResMut<StatusRequests>,
    mut status_events: EventWriter<StatusResponse>,
) {
    // Poll all tasks
    let mut finished: Vec<usize> = Vec::new();
    for (index, task) in tasks.iter_mut().enumerate() {
        if let Some(result) = block_on(poll_once(task)) {
            match result {
                Ok(status) => {
                    status_events.send(status);
                }
                Err(e) => {
                    error!("Error polling status request: {}", e);
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

/// A response to a status request
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct StatusResponse {
    pub description: String,
    pub favicon: Option<String>,
    pub player_max: i32,
    pub player_online: i32,
    pub sample_players: HashMap<String, Uuid>,
    pub version: String,
    pub protocol: i32,
}

/// A resource that contains a list of ongoing status requests
#[derive(Debug, Deref, DerefMut, Resource)]
pub struct StatusRequests(pub Vec<Task<Result<StatusResponse, ConnectionError>>>);
