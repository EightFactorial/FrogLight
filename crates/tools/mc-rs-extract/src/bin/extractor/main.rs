use clap::Parser;
use mc_rs_extract::{modules::ExtractModule, ModuleData, Version};
use strum::IntoEnumIterator;
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::{debug, error, info, warn};
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

mod args;
use args::Args;

/// The minimum supported version supported by this tool.
const MIN_SUPPORTED_VERSION: Version = Version::new_release(1, 20, 0);

#[tokio::main]
async fn main() {
    // Parse arguments
    let args = Args::parse();

    // Initialize tracing
    if args.quiet {
        let builder = SubscriberBuilder::default().without_time().compact();

        let filter = EnvFilter::from_default_env()
            .add_directive("reqwest=warn".parse().unwrap())
            .add_directive("hyper=warn".parse().unwrap())
            .add_directive(
                #[cfg(debug_assertions)]
                {
                    "mc_rs_extract=debug".parse().unwrap()
                },
                #[cfg(not(debug_assertions))]
                {
                    "mc_rs_extract=info".parse().unwrap()
                },
            );

        builder.with_env_filter(filter).init();
    }
    debug!("{args:?}");

    // Get the ModuleData
    let mut data = match ModuleData::new(args.version, args.refresh).await {
        Ok(data) => data,
        Err(err) => {
            error!("Error while getting ModuleData: {}", err);
            return;
        }
    };

    // Output warnings if extracting from an unsupported Version
    if let Version::Release { .. } = &data.version {
        if let Some(true) = data.version.lt(&MIN_SUPPORTED_VERSION, &data.manifest) {
            warn!("");
            warn!(
                "Version {} is older than the minimum supported version ({MIN_SUPPORTED_VERSION})",
                data.version
            );
            warn!("Some things might not work correctly!");
            warn!("");
        }
    } else {
        warn!("");
        warn!(
            "Version {} is not a Release version. Some things might not work correctly!",
            data.version
        );
        warn!("");
    }

    // Get the list of Modules to run
    let mut modules = if args.modules.is_empty() {
        debug!("No Modules specified, running all of them");
        ExtractModule::iter().collect::<Vec<ExtractModule>>()
    } else {
        args.modules
    };

    // Add module dependencies, recursively
    let mut i = 0;
    while i < modules.len() {
        let module = modules[i];
        for dep in module.deps() {
            if !modules.contains(dep) {
                debug!("Adding dependency `{dep}` for `{module}`");
                modules.push(*dep);
            }
        }

        i += 1;
    }

    // Sort and deduplicate the modules
    modules.dedup();
    modules.sort();

    debug!("Running: {modules:?}");
    info!("Extracting data for {}", data.version);
    info!("Starting...");
    info!("");

    // Run the Modules
    for m in modules {
        info!("Running {m}");
        m.run(&mut data);
        info!("");
    }
    info!("Done!");

    // Output the data
    let output = json::stringify_pretty(data.output, 4);
    if let Some(output_path) = args.output {
        info!("Saving JSON to `{}`", output_path.display());
        let mut file = match File::create(&output_path).await {
            Ok(file) => file,
            Err(err) => {
                error!("Error creating `{}`: {err}", output_path.display());
                return;
            }
        };

        if let Err(err) = file.write_all(output.as_bytes()).await {
            error!("Error writing to `{}`: {err}", output_path.display());
        }
    } else {
        debug!("Writing JSON to stdout");
        println!("{output}");
    }
}
