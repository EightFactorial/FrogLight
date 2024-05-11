use std::env;

fn main() {
    // Check if building the FrogLight binary
    if let Ok(package_name) = env::var("CARGO_PKG_NAME") {
        if package_name == "froglight" {
            froglight_scripts();
        }
    }
}

/// Scripts for building FrogLight.
fn froglight_scripts() {
    // Get the target platform
    let target = env::var("TARGET").expect("The \"TARGET\" environment variable is not set!");
    println!("cargo:warning=Building FrogLight for: \"{target}\"");

    if target.contains("windows") {
        // Add an executable icon for Windows
        windows_exec_icon();
    }
}

/// Embeds the icon into the Windows executable.
fn windows_exec_icon() {
    let profile = env::var("PROFILE").expect("The \"PROFILE\" environment variable is not set!");
    println!("cargo:warning=Setting the icon for the Windows executable...");

    let resource_file = if profile.contains("release") {
        println!("cargo:warning=Build in \"{profile}\" mode, using \"Verdant\" icon.");
        println!("cargo:rerun-if-changed=src/assets/verdant.rc");
        println!("cargo:rerun-if-changed=src/assets/verdant.ico");
        "src/assets/verdant.rc"
    } else {
        println!("cargo:warning=Build in \"{profile}\" mode, using \"Pearlescent\" icon.");
        println!("cargo:rerun-if-changed=src/assets/pearlescent.rc");
        println!("cargo:rerun-if-changed=src/assets/pearlescent.ico");
        "src/assets/pearlescent.rc"
    };
    embed_resource::compile(resource_file, embed_resource::NONE);
}
