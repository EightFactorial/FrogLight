# MC-RS Extractor

This crate downloads and extracts data from the Minecraft jar file.

Heavily inspired by similar projects like [Burger](https://github.com/Pokechu22/Burger) ❤️.

Generated data is not always accurate, check with another source like [Burger](https://github.com/Pokechu22/Burger) or [wiki.vg](https://wiki.vg/)

## Usage

While the main usage of this crate is as a library, it does include a small CLI tool to extract data as json.

Warnings and errors are printed to stderr, so you can pipe the output to a file or text editor.

```sh
# Extract json to console 
# { Run compiler } { Package } -- { Minecraft Version } { Command } {  List of dataset modules  }
cargo run --package mc-rs-ext -- --version <VERSION> extract --datasets diagnostics,info,etc...
```

Also included in the CLI tool are other commands for debugging and writing more dataset modules.

```sh
# Print a class to console
# { Run compiler } { Package } -- { Minecraft Version } { Command } { Class name }
cargo run --package mc-rs-ext -- --version <VERSION> print <CLASS>

# Print any classes containing a string to console
# { Run compiler } { Package } -- { Minecraft Version } { Command } { String }
cargo run --package mc-rs-ext -- --version <VERSION> search <STRING>
```