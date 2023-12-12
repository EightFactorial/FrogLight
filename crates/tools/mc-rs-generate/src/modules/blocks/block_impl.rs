use std::{future::Future, path::Path, pin::Pin};

use git2::Repository;
use mc_rs_extract::{modules::ExtractModule, ModuleData};
use tracing::{error, info};

use crate::modules::ModuleExt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BlockVersionModule;

impl ModuleExt for BlockVersionModule {
    fn deps(&self) -> &'static [ExtractModule] { &[ExtractModule::BlockAttributes] }

    fn run<'a>(
        &self,
        data: &'a mut ModuleData,
        repo: &Repository,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        let mut impl_path = repo.path().parent().unwrap().to_path_buf();
        impl_path.push("crates/mc-rs-world/src/blocks/version");
        impl_path.push(format!("{}.rs", data.version));

        Box::pin(async move {
            info!("Generating block impls");
            if let Err(err) = Self::generate_impls(&impl_path, data).await {
                error!("Failed to generate block impls: {err}");
            }
        })
    }
}

impl BlockVersionModule {
    async fn generate_impls(_path: &Path, _data: &ModuleData) -> Result<(), &'static str> { Ok(()) }
}
