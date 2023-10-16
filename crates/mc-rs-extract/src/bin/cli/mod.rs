use clap::{Parser, Subcommand};
use mc_rs_extract::{extract::datasets::Datasets, types::Version};

/// The command line interface for the application
#[derive(Debug, Parser)]
pub(crate) struct Cli {
    /// Quiet mode
    ///
    /// This will suppress all output except for errors
    #[arg(short, long, value_name = "bool", default_value = "false")]
    pub quiet: bool,

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

/// The commands that can be run
#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Extract information about the game
    Extract {
        /// The datasets to extract
        ///
        /// Specify sets like `blocks,items,entities` to extract multiple
        ///
        /// If none are specified, all datasets will be extracted
        #[clap(short, long, value_name = "datasets", value_delimiter = ',')]
        datasets: Option<Vec<Datasets>>,
    },
    /// Search the game for a String
    Search {
        /// The String to search for
        query: String,
    },
    /// Print the contents of a class file
    Print {
        /// The path of the class file
        ///
        /// Using `*` will print all classes, but this may take a while
        class: String,
    },
}
