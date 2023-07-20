use clap::{Parser, Subcommand};
use mc_rs_ext::types::Version;

#[derive(Debug, Parser)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Extract information from a version of Minecraft
    Extract {
        /// Refetch the version manifest
        ///
        /// Use this if a new version has been released
        #[arg(short, value_name = "bool", default_value = "false")]
        refresh: bool,

        /// Allow extracting from unstable versions
        ///
        /// This will allow extracting from snapshots, pre-releases, etc.
        #[arg(short, long, value_name = "bool", default_value = "false")]
        unstable: bool,

        /// The version to extract
        ///
        /// If not specified, the latest version will be used
        #[arg(short, long)]
        version: Option<Version>,
    },
    /// Search a Minecraft jar for a String
    Search {
        /// The version to search
        ///
        /// If not specified, the latest version will be used
        #[arg(short, long)]
        version: Option<Version>,

        /// The string to search for
        query: String,
    },
    /// Print the contents of a class file
    Print {
        /// The version to print from
        ///
        /// If not specified, the latest version will be used
        #[arg(short, long)]
        version: Option<Version>,

        /// The path of the class file
        path: String,
    },
}

impl Commands {
    pub fn version(&self) -> &Option<Version> {
        match &self {
            Commands::Extract { version, .. }
            | Commands::Search { version, .. }
            | Commands::Print { version, .. } => version,
        }
    }
}
