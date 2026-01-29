use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use dashmap::DashMap;
use reqwest::Client;

mod version;
pub use version::{Version, VersionStorage};

/// The root directory of the workspace.
pub const WORKSPACE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    while !dir.ends_with("FrogLight") {
        dir = dir.parent().expect("Failed to find workspace root directory");
    }
    dir.to_path_buf()
});
/// The cache directory for code generation.
pub const CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| WORKSPACE_DIR.join("target/codegen/"));

/// A thread-safe map storing version-specific data.
pub static DATA: LazyLock<DashMap<Version, VersionStorage>> = LazyLock::new(DashMap::new);
/// A shared HTTP client for making requests.
pub static REQWEST: LazyLock<Client> = LazyLock::new(Client::new);
