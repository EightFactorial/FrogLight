use std::sync::RwLock;

use async_trait::async_trait;
use foldhash::fast::FixedState;
use froglight_player::{profile::PlayerProfile, username::Username};
use indexmap::IndexMap;
use uuid::Uuid;

use crate::{
    api::{ApiError, NetworkApi},
    client::HttpClient,
};

/// An offline mock API that serves provided [`PlayerProfile`]s.
///
/// Does not make any network requests.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Offline;

impl Offline {
    /// The [`Offline`] API endpoint for querying player profiles.
    const PROFILE_ENDPOINT: &'static RwLock<IndexMap<Uuid, PlayerProfile, FixedState>> = {
        static STATIC: RwLock<IndexMap<Uuid, PlayerProfile, FixedState>> =
            RwLock::new(IndexMap::with_hasher(FixedState::with_seed(3)));

        &STATIC
    };
    /// The [`Offline`] API endpoint for querying usernames by UUID.
    const USERNAME_ENDPOINT: &'static RwLock<IndexMap<Uuid, Username, FixedState>> = {
        static STATIC: RwLock<IndexMap<Uuid, Username, FixedState>> =
            RwLock::new(IndexMap::with_hasher(FixedState::with_seed(4)));

        &STATIC
    };
    /// The [`Offline`] API endpoint for querying UUIDs by username.
    const UUID_ENDPOINT: &'static RwLock<IndexMap<Username, Uuid, FixedState>> = {
        static STATIC: RwLock<IndexMap<Username, Uuid, FixedState>> =
            RwLock::new(IndexMap::with_hasher(FixedState::with_seed(5)));

        &STATIC
    };

    /// Insert a [`PlayerProfile`] into the [`Offline`] API's internal storage.
    ///
    /// # Errors
    ///
    /// Returns the given profile if one with a matching UUID or username
    /// already exists.
    #[expect(clippy::missing_panics_doc, reason = "This should never panic")]
    pub fn insert_profile(profile: PlayerProfile) -> Result<(), PlayerProfile> {
        let uuid = *profile.uuid();
        let username = profile.username();

        let uuid_endpoint = Self::UUID_ENDPOINT.read().unwrap();
        let username_endpoint = Self::USERNAME_ENDPOINT.read().unwrap();
        let profile_endpoint = Self::PROFILE_ENDPOINT.read().unwrap();

        if uuid_endpoint.contains_key(username)
            || username_endpoint.contains_key(&uuid)
            || profile_endpoint.contains_key(&uuid)
        {
            Err(profile)
        } else {
            let mut uuid_endpoint = Self::UUID_ENDPOINT.write().unwrap();
            uuid_endpoint.insert(username.clone(), uuid);

            let mut username_endpoint = Self::USERNAME_ENDPOINT.write().unwrap();
            username_endpoint.insert(uuid, username.clone());

            let mut profile_endpoint = Self::PROFILE_ENDPOINT.write().unwrap();
            profile_endpoint.insert(uuid, profile);

            Ok(())
        }
    }

    /// Get a [`PlayerProfile`] from the [`Offline`] API's internal storage.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "This should never panic")]
    pub fn get_uuid(uuid: Uuid) -> Option<PlayerProfile> {
        Self::PROFILE_ENDPOINT.read().unwrap().get(&uuid).cloned()
    }

    /// Get a [`PlayerProfile`] from the [`Offline`] API's internal storage.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "This should never panic")]
    pub fn get_username(username: &str) -> Option<PlayerProfile> {
        let uuid = *Self::UUID_ENDPOINT.read().unwrap().get(username)?;
        Self::PROFILE_ENDPOINT.read().unwrap().get(&uuid).cloned()
    }

    /// Remove a [`PlayerProfile`] from the [`Offline`] API's internal storage.
    ///
    /// Returns the removed [`PlayerProfile`] if it existed.
    #[expect(clippy::missing_panics_doc, reason = "This should never panic")]
    pub fn remove_uuid(uuid: Uuid) -> Option<PlayerProfile> {
        let profile = Self::PROFILE_ENDPOINT.write().unwrap().shift_remove(&uuid)?;
        Self::USERNAME_ENDPOINT.write().unwrap().shift_remove(&uuid);
        Self::UUID_ENDPOINT.write().unwrap().shift_remove(profile.username());
        Some(profile)
    }

    /// Remove a [`PlayerProfile`] from the [`Offline`] API's internal storage.
    ///
    /// Returns the removed [`PlayerProfile`] if it existed.
    #[expect(clippy::missing_panics_doc, reason = "This should never panic")]
    pub fn remove_username(username: &str) -> Option<PlayerProfile> {
        let uuid = *Self::UUID_ENDPOINT.read().unwrap().get(username)?;
        let profile = Self::PROFILE_ENDPOINT.write().unwrap().shift_remove(&uuid)?;
        Self::USERNAME_ENDPOINT.write().unwrap().shift_remove(&uuid);
        Some(profile)
    }
}

#[async_trait]
impl NetworkApi for Offline {
    async fn query_uuid(
        &self,
        username: &str,
        _client: &HttpClient,
    ) -> Result<Option<Uuid>, ApiError> {
        Ok(Self::get_username(username).map(|profile| *profile.uuid()))
    }

    async fn query_username(
        &self,
        uuid: Uuid,
        _client: &HttpClient,
    ) -> Result<Option<Username>, ApiError> {
        Ok(Self::get_uuid(uuid).map(|profile| profile.username().clone()))
    }

    async fn query_profile(
        &self,
        uuid: Uuid,
        _client: &HttpClient,
    ) -> Result<Option<PlayerProfile>, ApiError> {
        Ok(Self::get_uuid(uuid))
    }
}
