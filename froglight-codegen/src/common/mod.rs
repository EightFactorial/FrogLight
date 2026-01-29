use std::sync::LazyLock;

use dashmap::DashMap;
use reqwest::Client;

mod version;
pub use version::{Version, VersionData};

/// The root directory of the workspace.
pub const WORKSPACE_DIR: &str = env!("CARGO_MANIFEST_DIR");
/// The cache directory for code generation.
pub const CACHE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/target/codegen");

/// A thread-safe map storing version-specific data.
pub static DATA: LazyLock<DashMap<Version, VersionData>> = LazyLock::new(DashMap::new);
/// A shared HTTP client for making requests.
pub static REQWEST: LazyLock<Client> = LazyLock::new(Client::new);
