use async_zip::base::read::{WithEntry, ZipEntryReader};
use bevy::reflect::Reflect;
use futures_lite::{io::Cursor, AsyncReadExt};

use crate::{loader::ResourcePackLoaderError, settings::ResourcePackLoaderSettings};

/// The audio settings for a resource pack.
///
/// This is different from the client's audio settings, and is used to define
/// which sounds play in different biomes, and how often they play.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct ResourcePackAudioSettings {}

impl ResourcePackAudioSettings {
    /// Parses the `sounds.json` file for a given resource pack.
    ///
    /// # Errors
    /// If the file is not valid JSON
    /// If the file is not a valid `sounds.json` file.
    #[allow(clippy::no_effect_underscore_binding)]
    pub(crate) async fn parse(
        entry: &mut ZipEntryReader<'_, Cursor<&[u8]>, WithEntry<'_>>,
        settings: &ResourcePackLoaderSettings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<(), ResourcePackLoaderError> {
        let manager = settings.0.as_ref().unwrap();

        // If the audio settings already exist, skip parsing the file.
        if manager.audio_settings.read().is_some() {
            #[cfg(feature = "logging")]
            bevy::log::trace!(
                "Skipping `sounds.json` from `{}` as it already exists",
                _load_context.path().display()
            );
            return Ok(());
        }

        // Read the file into memory.
        let mut contents = String::new();
        entry.read_to_string(&mut contents).await?;

        // TODO: Parse the `sounds.json` file.
        let settings = ResourcePackAudioSettings {};
        manager.audio_settings.write().replace(settings);

        Ok(())
    }
}
