use clap::{Parser, Subcommand};
use mc_rs_ext::types::Version;

#[derive(Debug, Parser)]
pub(crate) struct Cli {
    /// Refetch the version manifest
    ///
    /// Use this if a new version has been released
    #[arg(short, value_name = "bool", default_value = "false")]
    pub refresh: bool,

    /// Allow extracting from unstable versions
    ///
    /// This will allow extracting from snapshots, pre-releases, etc.
    #[arg(short, long, value_name = "bool", default_value = "false")]
    pub unstable: bool,

    /// The version to search
    ///
    /// If not specified, the latest version will be used
    #[arg(short, long)]
    pub version: Option<Version>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Extract information from a version of Minecraft
    Extract {
        /// The path to output the extracted data to
        ///
        /// If not specified, it will be output to the console
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Search a Minecraft jar for a String
    Search {
        /// The string to search for
        query: String,
    },
    /// Print the contents of a class file
    Print {
        /// The path of the output file
        ///
        /// If not specified, it will be output to the console
        #[arg(short, long)]
        output: Option<String>,

        /// The path of the class file
        class: String,
    },
}
