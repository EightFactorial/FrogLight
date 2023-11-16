use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Add environment variables at build time.
    EmitBuilder::builder()
        .all_build()
        .all_cargo()
        .all_git()
        .emit()?;

    Ok(())
}
