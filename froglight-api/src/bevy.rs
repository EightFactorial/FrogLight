//! TODO

use bevy_app::{App, Plugin};

use crate::{agent::HttpAgent, resolver::DnsResolver};

/// A [`Plugin`] that adds DNS and HTTP capabilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApiPlugin;

impl Plugin for ApiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HttpAgent>().register_type::<DnsResolver>();

        #[cfg(feature = "resolver")]
        app.init_resource::<DnsResolver>();

        #[cfg(feature = "ureq")]
        if !app.world().contains_resource::<HttpAgent>()
            && let Some(resolver) = app.world().get_resource::<DnsResolver>().cloned()
        {
            use ureq::{config::Config, unversioned::transport::DefaultConnector};

            app.world_mut().insert_resource(HttpAgent::new(ureq::Agent::with_parts(
                Config::default(),
                DefaultConnector::default(),
                resolver,
            )));
        }
    }
}
