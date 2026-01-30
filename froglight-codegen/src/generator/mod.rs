use miette::Result;

use crate::config::VersionPair;

pub async fn generate(VersionPair { base: _, real: _ }: VersionPair) -> Result<()> { Ok(()) }
