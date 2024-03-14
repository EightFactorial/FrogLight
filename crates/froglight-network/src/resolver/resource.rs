use std::sync::Arc;

use async_std_resolver::{
    lookup::{Ipv4Lookup, Ipv6Lookup, SrvLookup},
    lookup_ip::LookupIp,
    AsyncStdResolver, ResolveError,
};
use bevy_ecs::{component::Component, system::Resource};
use bevy_tasks::{IoTaskPool, Task};

/// A DNS resolver
///
/// Lookups are performed asynchronously and functions return
/// tasks that can be polled to get the result.
#[derive(Debug, Resource)]
pub struct ResolverResource {
    resolver: Arc<AsyncStdResolver>,
}

impl ResolverResource {
    /// Creates a new [`ResolverResource`] from the given client.
    #[must_use]
    pub fn new(resolver: AsyncStdResolver) -> Self { Self { resolver: Arc::new(resolver) } }

    /// Performs an [`Ipv4Lookup`] for the given query.
    ///
    /// # Note
    /// This uses bevy's [`IoTaskPool`](bevy_tasks::IoTaskPool).
    #[must_use]
    pub fn ipv4_lookup(&self, query: &str) -> ResolverIpv4Task {
        let resolver = Arc::clone(&self.resolver);
        let query = query.to_string();

        ResolverIpv4Task {
            task: IoTaskPool::get().spawn(async move { resolver.ipv4_lookup(query).await }),
        }
    }

    /// Performs an [`Ipv6Lookup`] for the given query.
    ///
    /// # Note
    /// This uses bevy's [`IoTaskPool`](bevy_tasks::IoTaskPool).
    #[must_use]
    pub fn ipv6_lookup(&self, query: &str) -> ResolverIpv6Task {
        let resolver = Arc::clone(&self.resolver);
        let query = query.to_string();

        ResolverIpv6Task {
            task: IoTaskPool::get().spawn(async move { resolver.ipv6_lookup(query).await }),
        }
    }

    /// Performs an ip lookup for the given query.
    ///
    /// This can be either an IPv4 or IPv6 address.
    ///
    /// # Note
    /// This uses bevy's [`IoTaskPool`](bevy_tasks::IoTaskPool).
    #[must_use]
    pub fn lookup_ip(&self, query: &str) -> ResolverIpTask {
        let resolver = Arc::clone(&self.resolver);
        let query = query.to_string();

        ResolverIpTask {
            task: IoTaskPool::get().spawn(async move { resolver.lookup_ip(query).await }),
        }
    }

    /// Performs a [`SrvLookup`] for the given query.
    ///
    /// # Note
    /// This uses bevy's [`IoTaskPool`](bevy_tasks::IoTaskPool).
    #[must_use]
    pub fn srv_lookup(&self, query: &str) -> ResolverSrvTask {
        let resolver = Arc::clone(&self.resolver);
        let query = query.to_string();

        ResolverSrvTask {
            task: IoTaskPool::get().spawn(async move { resolver.srv_lookup(query).await }),
        }
    }
}

/// A macro that creates a task for a resolver lookup.
macro_rules! create_task {
    ($task:ident, $result:ty) => {
        /// An in-progress
        /// `
        #[doc = stringify!($result)]
        /// `
        /// task.
        #[derive(Debug, Component)]
        pub struct $task {
            task: Task<Result<$result, ResolveError>>,
        }

        impl $task {
            /// Blocks the current thread and polls the task once.
            ///
            /// # Note
            /// If the task returns `Some`, do not poll again and drop the task.
            pub fn poll_once(&mut self) -> Option<Result<$result, ResolveError>> {
                bevy_tasks::block_on(bevy_tasks::poll_once(&mut self.task))
            }
        }
    };
}

create_task!(ResolverIpv4Task, Ipv4Lookup);
create_task!(ResolverIpv6Task, Ipv6Lookup);
create_task!(ResolverIpTask, LookupIp);
create_task!(ResolverSrvTask, SrvLookup);
