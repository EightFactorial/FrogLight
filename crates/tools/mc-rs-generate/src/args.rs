use clap::{ArgAction, Parser};
use mc_rs_extract::Version;

use crate::modules::GenerateModule;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub(crate) struct Args {
    /// Whether to enable logging
    ///
    /// By default, logging is enabled.
    #[arg(short, long, default_value = "true", action = ArgAction::SetFalse)]
    pub(crate) quiet: bool,

    /// Refresh the [`VersionManifest`](mc_rs_extract::manifest::VersionManifest)
    ///
    /// This is needed to get information about versions that have been released
    /// since the last time the [`VersionManifest`](mc_rs_extract::manifest::VersionManifest) was
    /// downloaded.
    #[arg(short, help = REFRESH_HELP, long, long_help = REFRESH_LONG_HELP)]
    pub(crate) refresh: bool,

    /// The [`Version`] to extract information from
    ///
    /// If not specified, the latest release version, as determined by the
    /// [`VersionManifest`](mc_rs_extract::manifest::VersionManifest), will be used.
    #[arg(short, help = VERSION_HELP, long, long_help = VERSION_LONG_HELP)]
    pub(crate) version: Option<Version>,

    /// The list of [`Module`s](crate::modules::Module) to run
    ///
    /// If none are specified, all modules will be run.
    #[arg(short, help = MODULES_HELP, long = "module", long_help = MODULES_LONG_HELP)]
    pub(crate) modules: Vec<GenerateModule>,
}

const REFRESH_HELP: &str = "Refresh the VersionManifest before extracting information.";

const REFRESH_LONG_HELP: &str = r"Refresh the VersionManifest before extracting information.

This is needed to get information about versions that have been
released since the last time the manifest was downloaded.";

const VERSION_HELP: &str = "The version to extract information from.";

const VERSION_LONG_HELP: &str = r"The version to extract information from.

If not specified, the latest release version, as determined by the VersionManifest, will be used.";

const MODULES_HELP: &str = "The list of modules to run.";

const MODULES_LONG_HELP: &str = r"The list of modules to run.

If none are specified, all modules will be run.";
