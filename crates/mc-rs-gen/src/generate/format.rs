use std::process::Command;

use git2::Repository;
use json::JsonValue;
use log::error;
use mc_rs_ext::{extract::datasets::Datasets, types::Version};

use super::Generator;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Format;

impl Generator for Format {
    fn deps(&self) -> &'static [Datasets] { &[] }

    fn parse(&self, _version: &Version, _data: &JsonValue, repo: &Repository) {
        let path = repo.path().parent().unwrap();

        let cmd = Command::new("cargo").arg("fmt").current_dir(path).spawn();

        match cmd {
            Ok(mut child) => {
                let status = child.wait().unwrap();
                if !status.success() {
                    error!("Failed to format code!");
                }
            }
            Err(err) => {
                error!("Failed to run cargo fmt: {}", err);
            }
        }
    }
}
