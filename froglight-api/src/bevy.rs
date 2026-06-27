//! TODO

use bevy_app::{App, Plugin};

use crate::{api::ClientApi, client::HttpClient, resolver::DnsResolver};

/// A [`Plugin`] that adds DNS and HTTP capabilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApiPlugin;

impl Plugin for ApiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ClientApi>()
            .register_type::<HttpClient>()
            .register_type::<DnsResolver>();
    }

    #[allow(unused_variables, reason = "Used if features are enabled")]
    fn finish(&self, app: &mut App) {
        #[cfg(feature = "resolver")]
        {
            #[cfg(feature = "tracing")]
            tracing::debug!(target: "froglight-api", "Initializing the default Hickory-based resolver");

            app.insert_resource(DnsResolver::hickory());
        }

        #[cfg(feature = "ureq")]
        if !app.world().contains_resource::<HttpClient>()
            && let Some(resolver) = app.world().get_resource::<DnsResolver>().cloned()
        {
            use ureq::{Agent, config::Config, unversioned::transport::DefaultConnector};

            #[cfg(feature = "tracing")]
            tracing::debug!(target: "froglight-api", "Initializing the default ureq-based HTTP client");

            let agent = Agent::with_parts(Config::default(), DefaultConnector::default(), resolver);
            app.world_mut().insert_resource(HttpClient::new(agent));
        }
    }
}
