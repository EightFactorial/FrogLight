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
    let target = env::var("TARGET").expect("The \"TARGET\" environment variable is not set!");
    println!("cargo:warning=Building FrogLight for: \"{target}\"");

    if target.contains("windows") {
        // Embed the executable icon
        windows_exec_icon();
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
