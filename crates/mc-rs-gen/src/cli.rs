use clap::Parser;
use mc_rs_ext::types::Version;

use crate::generate::Generators;

/// The command line interface for the application
#[derive(Debug, Parser)]
pub(crate) struct Cli {
    /// Refetch the version manifest
    ///
    /// Use this if a new version has been released but the manifest hasn't been updated yet
    #[arg(short, long, value_name = "bool", default_value = "false")]
    pub refresh: bool,

    /// Allow selecting unstable versions
    ///
    /// This will allow selecting snapshots, pre-releases, etc.
    #[arg(short, long, value_name = "bool", default_value = "false")]
    pub unstable: bool,

    /// The version to generate code for
    #[arg(short, long, value_name = "version")]
    pub version: Version,

    /// The code generators to run
    ///
    /// By default, only the `format` generator is run
    #[arg(short, long, value_name = "generator", default_value = "format")]
    pub generators: Option<Vec<Generators>>,
}
