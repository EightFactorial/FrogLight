# MC-RS Extractor

This crate downloads and extracts data from the Minecraft jar file.

Heavily inspired by similar projects like [Burger](https://github.com/Pokechu22/Burger) ❤️.

Generated data is not always accurate, check with other sources like [`Burger`](https://github.com/Pokechu22/Burger), [`wiki.vg`](https://wiki.vg/), or [`minecraft.wiki`](https://minecraft.wiki/).

## Usage

While the main usage of this crate is as a library, it does include a small CLI tool to extract data as JSON.

Logs are printed to stderr, so you can pipe the output to a file or text editor.

```sh
# Run the selected modules on the specified version
# {  Run Compiler  } {         Package         }   --  { Minecraft Version } {     List of Modules     }
cargo run --package mc-rs-extract --features=binary -- --version <VERSION> --module info --module debug

# Run the selected modules on the latest release
# {  Run Compiler  } {         Package         }   --  { List of Modules }
cargo run --package mc-rs-extract --features=binary -- -m info

# Run all modules on the latest release and save to `./latest.json`
# {  Run Compiler  } {         Package         }   --  {      Output File      }
cargo run --package mc-rs-extract --features=binary -- --output latest.json

```
