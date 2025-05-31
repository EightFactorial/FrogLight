//! [`RegistryStorage`] and [`AppRegistryStorage`]

mod generated;

mod storage;
pub use storage::{AppRegistryStorage, GlobalRegistryId, RegistryStorage};

mod traits;
pub use traits::RegistryTrait;
