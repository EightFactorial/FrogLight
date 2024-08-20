//! [`SingleLanguageMap`] and related types.

use bevy_app::App;
use bevy_asset::{Assets, Handle};
use bevy_ecs::{reflect::ReflectResource, system::Resource};
use bevy_reflect::Reflect;
use froglight_common::ResourceKey;

use super::SingleLanguageMap;
use crate::AssetCatalog;

/// The `CurrentLanguage` type is only registered here.
///
/// It is up to the client to create a [`CurrentLanguage`] resource on startup.
#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<CurrentLanguage>(); }

/// The current language.
///
/// Used to determine the language to use for strings.
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect(Resource)]
pub struct CurrentLanguage {
    language: ResourceKey,
    handle: Handle<SingleLanguageMap>,
}

impl CurrentLanguage {
    /// Creates a new [`CurrentLanguage`] [`Resource`].
    ///
    /// # Errors
    /// Errors if there is no language with the given key in the
    /// [`AssetCatalog`].
    pub fn new(
        language: ResourceKey,
        catalog: &AssetCatalog,
    ) -> Result<Self, CurrentLanguageError> {
        if let Some(handle) = catalog.get(&language) {
            Ok(Self { language, handle })
        } else {
            Err(CurrentLanguageError::UnknownLanguage)
        }
    }

    /// Sets the current language.
    ///
    /// # Errors
    /// Errors if there is no [`SingleLanguageMap`] with the given key in the
    /// [`AssetCatalog`].
    pub fn set_language(
        &mut self,
        language: ResourceKey,
        catalog: &AssetCatalog,
    ) -> Result<(), CurrentLanguageError> {
        if let Some(handle) = catalog.get(&language) {
            self.language = language;
            self.handle = handle;
            Ok(())
        } else {
            Err(CurrentLanguageError::UnknownLanguage)
        }
    }

    /// Gets the [`ResourceKey`] of the current language.
    #[must_use]
    pub fn language(&self) -> &ResourceKey { &self.language }

    /// Gets the handle to the current [`SingleLanguageMap`].
    #[must_use]
    pub fn handle(&self) -> &Handle<SingleLanguageMap> { &self.handle }

    /// Gets the current language.
    ///
    /// Returns `None` if the language does not exist.
    ///
    /// If only one or a few strings are needed, consider using
    /// [`CurrentLanguage::get_string`] instead.
    #[must_use]
    pub fn get_language<'a>(
        &self,
        assets: &'a Assets<SingleLanguageMap>,
    ) -> Option<&'a SingleLanguageMap> {
        assets.get(&self.handle)
    }

    /// Gets a string from the current language.
    ///
    /// Returns `None` if the language does not exist
    /// or the language does not contain the string.
    ///
    /// If many strings are needed, consider using
    /// [`CurrentLanguage::get_language`] instead.
    #[must_use]
    pub fn get_string<'a>(
        &self,
        string: &str,
        assets: &'a Assets<SingleLanguageMap>,
    ) -> Option<&'a str> {
        self.get_language(assets)?.get(string).map(String::as_str)
    }

    /// Gets the current language mutably.
    ///
    /// Returns `None` if the language does not exist.
    ///
    /// If only one or a few strings are needed, consider using
    /// [`CurrentLanguage::get_string_mut`] instead.
    #[must_use]
    pub fn get_language_mut<'a>(
        &self,
        assets: &'a mut Assets<SingleLanguageMap>,
    ) -> Option<&'a mut SingleLanguageMap> {
        assets.get_mut(&self.handle)
    }

    /// Gets a string from the current language mutably.
    ///
    /// Returns `None` if the language does not exist
    /// or the language does not contain the string.
    ///
    /// If many strings are needed, consider using
    /// [`CurrentLanguage::get_language_mut`] instead.
    #[must_use]
    pub fn get_string_mut<'a>(
        &self,
        string: &str,
        assets: &'a mut Assets<SingleLanguageMap>,
    ) -> Option<&'a mut String> {
        self.get_language_mut(assets)?.get_mut(string)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CurrentLanguageError {
    /// The language was not found in the [`AssetCatalog`].
    #[error("Language not found in the AssetCatalog")]
    UnknownLanguage,
}
