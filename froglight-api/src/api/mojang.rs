use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    api::{ApiError, NetworkApi},
    client::HttpClient,
};

/// The standard, default [`NetworkApi`].
///
/// Uses Mojang's official API endpoints.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mojang;

#[async_trait]
impl NetworkApi for Mojang {
    async fn query_uuid(&self, _: &str, _: &HttpClient) -> Result<Option<Uuid>, ApiError> {
        todo!()
    }

    async fn query_username(&self, _: Uuid, _: &HttpClient) -> Result<Option<String>, ApiError> {
        todo!()
    }
}
