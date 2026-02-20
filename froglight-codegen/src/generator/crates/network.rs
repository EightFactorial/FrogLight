use miette::Result;

use crate::{common::WORKSPACE_DIR, config::ConfigBundle, helper::ModuleBuilder};

/// Generate placeholder network implementations for all [`Version`]s.
pub async fn generate_global(config: &ConfigBundle) -> Result<()> {
    event_versions(config).await?;
    network_versions(config).await
}

async fn event_versions(config: &ConfigBundle) -> Result<()> {
    let path = WORKSPACE_DIR.join("froglight-network/src/event/mod.rs");
    let mut module = ModuleBuilder::new_after_marker(path.clone()).await?;

    for version in &config.versions {
        let version_feature = version.base.as_feature();
        let version_ident = version_feature.to_ascii_uppercase();

        module
            .with_submodule(&version_feature, async |builder, mut settings| {
                settings = settings.with_feature(version_feature.clone());

                let mut path = path.clone();
                path.set_file_name(&version_feature);
                path.set_extension("rs");

                if path.exists() {
                    // Skip generating the file if it already exists.
                    settings.build = false;
                } else {
                    // Otherwise, generate a placeholder implementation.
                    let docs = format!("[`EventVersion`] implementation for [`{version_ident}`].");

                    let content = format!(
                        "
use froglight_common::version::{version_ident};
use froglight_packet::version::{{Clientbound, Serverbound, VersionPacket}};

use crate::{{connection::ConnectionError, event::EventVersion, prelude::*}};

impl EventVersion for {version_ident} {{
    fn client_event_to_packet(
        _event: ClientboundEventEnum,
    ) -> Result<Option<VersionPacket<Self, Clientbound>>, ConnectionError> {{
        todo!();
    }}

    fn client_packet_to_event(
        _packet: VersionPacket<Self, Clientbound>,
    ) -> Result<Option<ClientboundEventEnum>, ConnectionError> {{
        todo!();
    }}

    fn server_event_to_packet(
        _event: ServerboundEventEnum,
    ) -> Result<Option<VersionPacket<Self, Serverbound>>, ConnectionError> {{
        todo!();
    }}

    fn server_packet_to_event(
        _packet: VersionPacket<Self, Serverbound>,
    ) -> Result<Option<ServerboundEventEnum>, ConnectionError> {{
        todo!();
    }}
}}
"
                    );

                    builder.with_docs(&docs).with_content(&content);
                }

                Ok(settings)
            })
            .await?;
    }

    module.build().await
}

async fn network_versions(config: &ConfigBundle) -> Result<()> {
    let path = WORKSPACE_DIR.join("froglight-network/src/bevy/version/mod.rs");
    let mut module = ModuleBuilder::new_after_marker(path.clone()).await?;

    for version in &config.versions {
        let version_feature = version.base.as_feature();
        let version_ident = version_feature.to_ascii_uppercase();

        module
            .with_submodule(&version_feature, async |builder, mut settings| {
                settings = settings.with_feature(version_feature.clone());

                let mut path = path.clone();
                path.set_file_name(&version_feature);
                path.set_extension("rs");

                if path.exists() {
                    // Skip generating the file if it already exists.
                    settings.build = false;
                } else {
                    // Otherwise, generate a placeholder implementation.
                    let docs =
                        format!("[`NetworkVersion`] implementation for [`{version_ident}`].");

                    let content = format!(
                        "
use bevy_ecs::world::EntityRef;
use froglight_common::version::{version_ident};
use froglight_packet::version::{{Clientbound, Serverbound, VersionPacket}};

use super::ConnectionUpdate;
use crate::{{bevy::NetworkVersion, connection::ConnectionError, event::EventVersion, prelude::*}};

impl NetworkVersion for {version_ident} {{
    fn update_connection_details(
        _packet: &VersionPacket<Self, Clientbound>,
    ) -> Option<ConnectionUpdate> {{
        todo!()
    }}

    #[inline]
    fn event_to_packet(
        event: ServerboundEventEnum,
        _: EntityRef<'_>,
    ) -> Result<Option<VersionPacket<Self, Serverbound>>, ConnectionError> {{
        <{version_ident} as EventVersion>::server_event_to_packet(event)
    }}

    #[inline]
    fn packet_to_event(
        packet: VersionPacket<Self, Clientbound>,
        _: EntityRef<'_>,
    ) -> Result<Option<ClientboundEventEnum>, ConnectionError> {{
        <{version_ident} as EventVersion>::client_packet_to_event(packet)
    }}
}}
"
                    );

                    builder.with_docs(&docs).with_content(&content);
                }

                Ok(settings)
            })
            .await?;
    }

    module.build().await
}
