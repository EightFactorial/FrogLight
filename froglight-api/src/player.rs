//! API extensions for [`froglight_player`].

use facet::Facet;
use froglight_player::{prelude::*, profile::ProfilePropertyItem};
use uuid::Uuid;

use crate::{
    api::{ApiError, ClientApi},
    prelude::HttpClient,
};

/// An extension trait for [`Username`].
pub trait UsernameExt {
    /// Fetch the player's [`Uuid`] from the API.
    ///
    /// ## Warning
    ///
    /// This method makes a web request, which is *very* slow.
    ///
    /// If this isn't a new player they likely already have a [`Uuid`] inside
    /// their [`PlayerProfile`].
    fn uuid(
        &self,
        api: &ClientApi,
        client: &HttpClient,
    ) -> impl Future<Output = Result<Option<Uuid>, ApiError>> + Send;
}

impl UsernameExt for Username {
    #[inline]
    fn uuid(
        &self,
        api: &ClientApi,
        client: &HttpClient,
    ) -> impl Future<Output = Result<Option<Uuid>, ApiError>> + Send {
        api.query_uuid(self, client)
    }
}

// -------------------------------------------------------------------------------------------------

/// An extension trait for [`PlayerProfile`].
pub trait PlayerProfileExt {}

impl PlayerProfileExt for PlayerProfile {}

// ------------------------------------------------

/// A profile property containing the player's skin and cape textures.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Facet)]
pub struct PlayerTextureProperty {
    /// The timestamp of when the textures were last updated.
    pub timestamp: u64,
    /// The player's [`Uuid`].
    #[facet(rename = "profileId")]
    pub profile_id: Uuid,
    /// The player's [`Username`].
    #[facet(rename = "profileName")]
    pub profile_name: Username,
    /// Whether the player has a signature for their textures.
    #[facet(default, rename = "signatureRequired")]
    pub signature_required: bool,
    /// The player's skin and cape textures.
    #[facet(default)]
    pub textures: PlayerTextureURLs,
}
/// A player's skin and cape texture URLs.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Facet)]
pub struct PlayerTextureURLs {
    /// The URL of the player's skin texture.
    #[facet(default, rename = "SKIN")]
    pub skin: Option<PlayerTextureURL>,
    /// The URL of the player's cape texture.
    #[facet(default, rename = "CAPE")]
    pub cape: Option<PlayerTextureURL>,
}
/// A URL container
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Facet)]
pub struct PlayerTextureURL {
    /// The URL of the texture.
    pub url: String,
}

impl ProfilePropertyItem for PlayerTextureProperty {
    const PROPERTY_KEY: &'static str = "textures";
}

// -------------------------------------------------------------------------------------------------
