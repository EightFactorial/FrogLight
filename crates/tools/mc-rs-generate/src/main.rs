use clap::Parser;
use git2::Repository;
use mc_rs_extract::{modules::ExtractModule, ModuleData, Version};
use tracing::{debug, error, info, warn};
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

mod args;
use args::Args;

mod modules;

/// The minimum supported version supported by this tool.
const MIN_SUPPORTED_VERSION: Version = Version::new_release(1, 20, 0);

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() {
    // Parse arguments
    let args = Args::parse();

    // Initialize tracing
    if args.verbose {
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
            )
            .add_directive(
                #[cfg(debug_assertions)]
                {
                    "mc_rs_generate=debug".parse().unwrap()
                },
                #[cfg(not(debug_assertions))]
                {
                    "mc_rs_generate=info".parse().unwrap()
                },
            );

        builder.with_env_filter(filter).init();
    }
    debug!("{args:?}");

    // Exit if there are no modules to run
    if args.modules.is_empty() {
        error!("No modules specified, exiting...");
        return;
    }

    // Get the local Repository
    // This is used for local file paths
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(err) => {
            error!("Could not get the local repository: {err}");
            return;
        }
    };

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

    // Get the ExtractModules to run
    let mut extract_modules: Vec<ExtractModule> = Vec::new();
    for gen_mod in &args.modules {
        for dep in gen_mod.deps() {
            if !extract_modules.contains(dep) {
                debug!("Adding dependency `{dep}` for `{gen_mod}`");
                extract_modules.push(*dep);
            }
        }
    }

    if !extract_modules.is_empty() {
        // Add ExtractModule dependencies, recursively
        let mut i = 0;
        while i < extract_modules.len() {
            let ex_mod = extract_modules[i];
            for dep in ex_mod.deps() {
                if !extract_modules.contains(dep) {
                    debug!("Adding dependency `{dep}` for `{ex_mod}`");
                    extract_modules.push(*dep);
                }
            }

            i += 1;
        }

        // Sort and deduplicate the ExtractModules
        extract_modules.dedup();
        extract_modules.sort();
        debug!("Running Extractors: {extract_modules:?}");

        info!("Extracting data...");
        info!("");

        // Run the ExtractModules
        for m in &extract_modules {
            info!("Running {m}");
            m.run(&mut data);
            info!("");
        }
    }

    // Sort the deduplicate the GenerateModules
    let mut generate_modules = args.modules;
    generate_modules.dedup();
    generate_modules.sort();
    debug!("Running Generators: {generate_modules:?}");

    // Run the GenerateModules
    if extract_modules.is_empty() {
        info!("Running generators...");
    } else {
        info!("Data extraction complete, running generators...");
    }
    info!("");
    for m in generate_modules {
        info!("Running {m}");
        m.run(&mut data, &repo).await;
        info!("");
    }
    info!("Done!");

    warn!("");
    warn!("Please test generated code before committing it!");
    warn!("");
}
