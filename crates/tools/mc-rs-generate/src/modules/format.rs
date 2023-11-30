use std::{future::Future, pin::Pin};

use git2::Repository;
use mc_rs_extract::ModuleData;
use tokio::process::Command;
use tracing::{error, info};

use super::ModuleExt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FormatModule;

impl ModuleExt for FormatModule {
    fn run(&self, _data: &ModuleData, _repo: &Repository) -> Pin<Box<dyn Future<Output = ()>>> {
        Box::pin(async {
            let mut command = Command::new("cargo");
            command.arg("fmt").arg("--all");

            let mut child = match command.spawn() {
                Ok(child) => child,
                Err(err) => {
                    error!("Failed to spawn Command `cargo fmt --all`: {err}");
                    return;
                }
            };

            let status = match child.wait().await {
                Ok(status) => status,
                Err(err) => {
                    error!("Failed to wait for Command `cargo fmt --all`: {err}");
                    return;
                }
            };

            if status.success() {
                info!("Project formatted successfully");
            } else {
                error!("Command `cargo fmt --all` failed with: {status}");
            }
        })
    }
}
