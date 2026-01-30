use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use papaya::HashMap;
use reqwest::Client;

mod version;
use tokio::sync::RwLock;
pub use version::{Version, VersionStorage};

/// The root directory of the workspace.
pub static WORKSPACE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    while !dir.ends_with("FrogLight") {
        dir = dir.parent().expect("Failed to find workspace root directory");
    }
    dir.to_path_buf()
});
/// The cache directory for code generation.
pub static CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| WORKSPACE_DIR.join("target/codegen/"));

/// A thread-safe map storing version-specific data.
pub static DATA: LazyLock<HashMap<Version, RwLock<VersionStorage>>> = LazyLock::new(HashMap::new);
/// A shared HTTP client for making requests.
pub static REQWEST: LazyLock<Client> = LazyLock::new(Client::new);
