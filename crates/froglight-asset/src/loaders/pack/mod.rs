mod folder;
pub use folder::{ResourcePackFolderError, ResourcePackFolderLoader};

mod zip;
pub use zip::{ResourcePackZipError, ResourcePackZipLoader};

/// The type of an entry in a resource pack.
enum EntryType {
    Texture,
    Sound,
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

        // Read the directory after the namespace.
        match path.split('/').nth(2)? {
            "textures" => Some(Self::Texture),
            "sounds" => Some(Self::Sound),
            "sounds.json" => Some(Self::SoundMap),
            #[cfg(debug_assertions)]
            unk => {
                bevy_log::warn!("Unknown entry type: {unk}");
                None
            }
            #[cfg(not(debug_assertions))]
            _ => None,
        }
    }
}
