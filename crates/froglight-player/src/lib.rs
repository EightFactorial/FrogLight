#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod chat;
pub mod profile;
pub mod username;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        chat::{signature::MessageSignatureCtx, text::Text},
        profile::PlayerProfile,
        username::PlayerUsername,
    };
}

/// Retry a request up to N times if it fails.
///
/// # Warning
/// This function will block until the request is complete!
#[cfg(feature = "online")]
fn retry_request<T: serde::de::DeserializeOwned, const N: usize>(
    uri: &str,
    agent: &ureq::Agent,
) -> Result<T, ureq::Error> {
    let mut response = handle_request::<T>(uri, agent);

    // Retry up to N times if the request fails.
    let mut attempts = 0;
    while response.is_err() && attempts < N {
        response = handle_request::<T>(uri, agent);
        attempts += 1;
    }

    response
}

/// Get information from the given API endpoint and deserialize it.
///
/// # Warning
/// This function will block until the request is complete!
#[cfg(feature = "online")]
fn handle_request<T: serde::de::DeserializeOwned>(
    uri: &str,
    agent: &ureq::Agent,
) -> Result<T, ureq::Error> {
    agent.get(uri).call()?.into_body().read_json()
}
