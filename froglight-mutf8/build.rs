//! TODO

use std::process::Command;

fn main() {
    // Search `rustc` for the host's native CPU type.
    let mut target_cpu = String::new();
    if let Ok(out) = Command::new("rustc").args(["--print", "target-cpus"]).output() {
        let out = String::from_utf8_lossy(&out.stdout);

        // Look for the host's CPU type
        if let Some(current) = out.lines().find(|l| l.contains("native"))
            && let Some((_, cpu)) = current.split_once("(currently ")
        {
            target_cpu = cpu.trim_end_matches(").").to_string();
        }
    }

    // Allow the `slow_bmi2` cfg to be used.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-check-cfg=cfg(slow_bmi2)");

    // Disable optimizations for AMD Zen 1 and 2 CPUs.
    if matches!(target_cpu.as_str(), "znver1" | "znver2") {
        #[cfg(feature = "simd")]
        println!("cargo:warning=Disabling optimizations for current CPU!");
        println!("cargo:rustc-cfg=slow_bmi2");
    }
}
