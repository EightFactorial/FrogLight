use std::env;

fn main() {
    // Check if building the FrogLight binary
    if Ok("froglight-bin") == env::var("CARGO_PKG_NAME").as_deref() {
        froglight_scripts();
    }
}

/// Scripts for building FrogLight
fn froglight_scripts() {
    // Get the target platform
    let target_platform =
        env::var("TARGET").expect("The \"TARGET\" environment variable is not set!");
    println!("cargo:warning=Building FrogLight for: \"{target_platform}\"");

    if target_platform.contains("windows") {
        // Embed the executable icon
        windows_exec_icon();
    }

    // Get the target cpu
    let target_cpu = if let Ok(flags) = env::var("CARGO_ENCODED_RUSTFLAGS") {
        // Split the flags by the flag separator
        let mut flags = flags.split('\u{1f}');

        // Find the 'target-cpu' flag
        if let Some(flag) = flags.find(|flag| flag.starts_with("-Ctarget-cpu=")) {
            flag.split('=').nth(1).unwrap_or("generic").to_string()
        } else {
            String::from("generic")
        }
    } else {
        String::from("generic")
    };
    println!("cargo::rustc-env=FROGLIGHT_TARGET_CPU={target_cpu}");

    // Get the rust compiler version
    match rustc_version::version_meta() {
        Ok(version) => {
            let channel = format!("{:?}", version.channel).to_ascii_lowercase();
            println!("cargo::rustc-env=FROGLIGHT_RUSTC_CHANNEL={channel}");

            let version = format!(
                "{}.{}.{}",
                version.semver.major, version.semver.minor, version.semver.patch
            );
            println!("cargo::rustc-env=FROGLIGHT_RUSTC_VERSION={version}");
        }
        Err(err) => {
            println!("cargo:warning=Failed to get rustc metadata: {err}");
        }
    }
}

/// Embeds the icon into the Windows executable
fn windows_exec_icon() {
    let profile = env::var("PROFILE").expect("The \"PROFILE\" environment variable is not set!");
    println!("cargo:warning=Setting the icon for the Windows executable...");

    // Set the icon based on the build profile
    let resource_file = if profile.contains("release") {
        // Use the "Verdant" icon for release builds
        println!("cargo:warning=Build in \"{profile}\" mode, using \"Verdant\" icon.");
        println!("cargo:rerun-if-changed=src/assets/verdant.rc");
        println!("cargo:rerun-if-changed=src/assets/verdant.ico");

        "src/assets/verdant.rc"
    } else {
        // Use the "Pearlescent" icon for other builds
        println!("cargo:warning=Build in \"{profile}\" mode, using \"Pearlescent\" icon.");
        println!("cargo:rerun-if-changed=src/assets/pearlescent.rc");
        println!("cargo:rerun-if-changed=src/assets/pearlescent.ico");

        "src/assets/pearlescent.rc"
    };

    // Compile the resource file
    embed_resource::compile(resource_file, embed_resource::NONE);
}
