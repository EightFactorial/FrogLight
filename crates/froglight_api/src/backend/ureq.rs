use alloc::{boxed::Box, vec::Vec};

use ureq::Agent;

use crate::backend::{BackendError, ClientBackend};

#[async_trait::async_trait]
impl ClientBackend for Agent {
    async fn get(
        &self,
        url: &str,
        headers: Option<&[(&str, &str)]>,
    ) -> Result<Vec<u8>, BackendError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("GET: \"{url}\"");

        let mut request = self.get(url);

        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(*key, *value);
            }
        }

        match request.call() {
            Ok(mut res) => Ok(res.body_mut().read_to_vec()?),
            Err(err) => Err(BackendError::from(err)),
        }
    }

    async fn post(
        &self,
        url: &str,
        headers: Option<&[(&str, &str)]>,
        body: &[u8],
    ) -> Result<Vec<u8>, BackendError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("POST: \"{url}\"");

        let mut request = self.post(url);

        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(*key, *value);
            }
        }

        match request.send(body) {
            Ok(mut res) => Ok(res.body_mut().read_to_vec()?),
            Err(err) => Err(BackendError::from(err)),
        }
    }

    async fn put(
        &self,
        url: &str,
        headers: Option<&[(&str, &str)]>,
        body: &[u8],
    ) -> Result<Vec<u8>, BackendError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("PUT: \"{url}\"");

        let mut request = self.put(url);

        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(*key, *value);
            }
        }

        match request.send(body) {
            Ok(mut res) => Ok(res.body_mut().read_to_vec()?),
            Err(err) => Err(BackendError::from(err)),
        }
    }

    async fn delete(
        &self,
        url: &str,
        headers: Option<&[(&str, &str)]>,
        body: Option<&[u8]>,
    ) -> Result<Vec<u8>, BackendError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("DELETE: \"{url}\"");

        let mut request = self.delete(url);

        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(*key, *value);
            }
        }

        match match body {
            Some(body) => request.force_send_body().send(body),
            None => request.call(),
        } {
            Ok(mut res) => Ok(res.body_mut().read_to_vec()?),
            Err(err) => Err(BackendError::from(err)),
        }
    }
}

impl From<ureq::Error> for BackendError {
    fn from(err: ureq::Error) -> Self {
        match err {
            ureq::Error::StatusCode(code) => BackendError::StatusCode(code),
            ureq::Error::Io(error) => BackendError::Io(error),
            ureq::Error::HostNotFound => BackendError::HostNotFound,
            ureq::Error::Other(error) => BackendError::Other(error),
            other => BackendError::Other(Box::new(other)),
        }
    }
}
