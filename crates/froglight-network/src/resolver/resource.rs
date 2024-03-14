use std::sync::Arc;

use async_std_resolver::{lookup::SrvLookup, proto::rr::IntoName, AsyncStdResolver, ResolveError};
use bevy_ecs::{component::Component, system::Resource};
use bevy_tasks::{IoTaskPool, Task};

/// A DNS resolver
#[derive(Debug, Resource)]
pub struct ResolverResource {
    client: Arc<AsyncStdResolver>,
}

impl ResolverResource {
    /// Creates a new resolver resource from the given client.
    #[must_use]
    pub fn new(client: AsyncStdResolver) -> Self { Self { client: Arc::new(client) } }

    /// Looks up an SRV record for the given query.
    ///
    /// # Note
    /// This uses bevy's [`IoTaskPool`](bevy_tasks::IoTaskPool).
    #[must_use]
    pub fn srv_lookup<N: IntoName>(&self, query: &str) -> ResolverSrvTask {
        let client = Arc::clone(&self.client);
        let query = query.to_string();

        ResolverSrvTask {
            task: IoTaskPool::get().spawn(async move { client.srv_lookup(query).await }),
        }
    }
}

/// An in-progress [`SrvLookup`].
#[derive(Component)]
pub struct ResolverSrvTask {
    task: Task<Result<SrvLookup, ResolveError>>,
}

impl ResolverSrvTask {
    /// Blocks the current thread and polls the task once.
    ///
    /// # Note
    /// If the task returns `Some`, do not poll again and drop the task.
    pub fn poll_once(&mut self) -> Option<Result<SrvLookup, ResolveError>> {
        bevy_tasks::block_on(bevy_tasks::poll_once(&mut self.task))
    }
}
