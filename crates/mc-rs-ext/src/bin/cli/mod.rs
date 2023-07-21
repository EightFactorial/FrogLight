use clap::{Parser, Subcommand};
use mc_rs_ext::types::Version;

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

    /// The path to output data to
    ///
    /// If not specified, it will be output to the console
    #[arg(short, long, value_name = "path")]
    pub output: Option<String>,

    /// The version to extract from
    ///
    /// If not specified, the latest version will be used
    #[arg(short, long, value_name = "version")]
    pub version: Option<Version>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Extract information about the game
    Extract,
    /// Search the game for a String
    Search {
        /// The String to search for
        query: String,
    },
    /// Print the contents of a class file
    Print {
        /// The path of the class file
        class: String,
    },
}
