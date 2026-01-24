use async_trait::async_trait;
use reqwest::Client;

use crate::client::{GetOptions, GetResponse, HttpError, NetworkClient, PutOptions, PutResponse};

#[async_trait]
impl NetworkClient for Client {
    async fn get(&self, url: &str, _opts: GetOptions) -> Result<GetResponse, HttpError> {
        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_api::client::reqwest", "GET \"{url}\", OPTS {_opts:?}");

        let response = self.get(url).send().await.map_err(|err| HttpError::Other(Box::new(err)))?;
        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_api::client::reqwest", "Response from GET \"{url}\": {response:#?}");

        if response.status().is_success() {
            let headers: Vec<_> = response
                .headers()
                .iter()
                .map(|(h, v)| (h.to_string(), v.as_bytes().to_vec()))
                .collect();
            let status = response.status().as_u16();
            let data = response.bytes().await.map_err(|err| HttpError::Other(Box::new(err)))?;
            Ok(GetResponse { status, data: data.to_vec(), headers })
        } else {
            Err(HttpError::Http(response.status().as_u16()))
        }
    }

    async fn put(
        &self,
        url: &str,
        data: Vec<u8>,
        _opts: PutOptions,
    ) -> Result<PutResponse, HttpError> {
        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_api::client::reqwest", "PUT {url}, OPTS {_opts:?}");

        let response =
            self.put(url).body(data).send().await.map_err(|err| HttpError::Other(Box::new(err)))?;
        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_api::client::reqwest", "Response from PUT \"{url}\": {response:#?}");

        if response.status().is_success() {
            Ok(PutResponse { status: response.status().as_u16() })
        } else {
            Err(HttpError::Http(response.status().as_u16()))
        }
    }
}
