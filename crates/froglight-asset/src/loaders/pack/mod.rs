mod folder;
pub use folder::{ResourcePackFolderError, ResourcePackFolderLoader};

mod zip;
pub use zip::{ResourcePackZipError, ResourcePackZipLoader};

/// The type of an entry in a resource pack.
enum EntryType {
    Texture,
    Sound,
    Language,
    TextureAtlas,
    ResourcePack,
    SoundMap,
    PackMeta,
    PackPng,
}

impl EntryType {
    /// Gets the type of an entry from its path.
    #[must_use]
    fn from_path(path: &str) -> Option<Self> {
        // Check for `pack.mcmeta` and `pack.png`.
        match path {
            "pack.mcmeta" => return Some(Self::PackMeta),
            "pack.png" => return Some(Self::PackPng),
            _ => {}
        }

        // Skip .mcmeta files.
        if path.ends_with(".mcmeta") {
            return None;
        }
        // Skip icons.
        if path.starts_with("assets/icons") {
            return None;
        }

        // Get the folder and file extension
        let folder = path.split('/').nth(2)?;
        let extension = path.split('.').last()?;

        // Match the folder and expected extension.
        match (folder, extension) {
            ("textures", "png") => Some(Self::Texture),
            ("sounds", "ogg") => Some(Self::Sound),
            ("lang", "json") => Some(Self::Language),
            ("atlases", "json") => Some(Self::TextureAtlas),
            ("resourcepacks", "zip") => Some(Self::ResourcePack),
            ("sounds.json", "json") => Some(Self::SoundMap),

            // Suppress warnings for known unsupported assets.
            #[cfg(debug_assertions)]
            ("blockstates" | "font" | "models" | "particles" | "shaders" | "texts", _) => None,

            // Suppress warnings for known but unused assets.
            #[cfg(debug_assertions)]
            ("gpu_warnlist.json" | "regional_compliancies.json", _) => None,

            // Warn about unknown assets in debug mode.
            #[cfg(debug_assertions)]
            _ => {
                bevy_log::warn!("Unknown asset: \"{path}\"");
                None
            }
            // Ignore unknown assets in release mode.
            #[cfg(not(debug_assertions))]
            _ => None,
        }
    }
}
