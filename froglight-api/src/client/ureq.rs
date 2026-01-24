use async_trait::async_trait;
use ureq::Agent;

use crate::client::{GetOptions, GetResponse, HttpError, NetworkClient, PutOptions, PutResponse};

#[async_trait]
impl NetworkClient for Agent {
    async fn get(&self, url: &str, _opts: GetOptions) -> Result<GetResponse, HttpError> {
        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_api::client::ureq", "GET \"{url}\", OPTS {_opts:?}");

        let mut response = self.get(url).call().map_err(|err| HttpError::Other(Box::new(err)))?;
        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_api::client::ureq", "Response from GET \"{url}\": {response:#?}");

        if response.status().is_success() {
            let headers: Vec<_> = response
                .headers()
                .iter()
                .map(|(h, v)| (h.to_string(), v.as_bytes().to_vec()))
                .collect();
            let status = response.status().as_u16();
            let data =
                response.body_mut().read_to_vec().map_err(|err| HttpError::Other(Box::new(err)))?;
            Ok(GetResponse { status, data, headers })
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
        tracing::trace!(target: "froglight_api::client::ureq", "PUT {url}, OPTS {_opts:?}");

        let response = self.put(url).send(data).map_err(|err| HttpError::Other(Box::new(err)))?;
        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_api::client::ureq", "Response from PUT \"{url}\": {response:#?}");

        if response.status().is_success() {
            Ok(PutResponse { status: response.status().as_u16() })
        } else {
            Err(HttpError::Http(response.status().as_u16()))
        }
    }
}
