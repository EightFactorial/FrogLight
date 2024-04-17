<h1 align="center">FrogLight</h1>
<p align="center">A Minecraft client written in Rust using <a href="https://bevyengine.org/">Bevy</a></p>

<p align="center">
  <a href="https://github.com/EightFactorial/FrogLight"><img alt="Documentation" src="https://img.shields.io/badge/docs-main-green.svg"></a>  
  <img alt="License" src="https://img.shields.io/badge/license-MIT/Apache--2.0---?color=blue">
  <a href="https://github.com/EightFactorial/FrogLight/actions"><img alt="Tests" src="https://github.com/EightFactorial/FrogLight/actions/workflows/testing.yml/badge.svg"></a>
</p>

## About

FrogLight is currently in a **very early alpha state**. Not everything is functional yet!

This project does not include any assets, all assets must be loaded from resource packs!

> [!Warning]
> Modded clients will get you **banned** from servers with anticheat!
> 
> You have been warned!

## Version Support

FrogLight is multiprotocol and supports release versions 1.20.0+, including:
- 1.20.0 and 1.20.1 (763)
- ~1.20.2~ (764)
- ~1.20.3 and 1.20.4~ (765)

> [!Note]
> Support for more versions will be added after core features and functionality.
> 
> Everything is built with support for multiple versions in mind!

## Configuration Files

All configuration files and resource packs are stored in the following directories:

| OS      | Config Folder                             |
| ------- | ----------------------------------------- |
| Windows | `%APPDATA%/FrogLight`                     |
| Linux   | `$XDG_CONFIG_HOME/FrogLight`              |
| MacOS   | `~/Library/Application Support/FrogLight` |

## Building

Like most Rust projects, you can build the project using `cargo`

```sh
# Clone and enter the repository
git clone --depth 1 https://github.com/EightFactorial/FrogLight
cd FrogLight

# Build the client, or
cargo build --release --bin froglight
# Build and run the client
cargo run --release
```

This will take a while the first time and requires a couple of gigabytes of space!

After building, FrogLight can be found at `./target/release/froglight[.exe]`

> [!Important]
> Make sure you are building in `--release` mode!

## Plugins

Using the [Bevy Engine](https://github.com/bevyengine/bevy/), plugins are very easy to create.

```rust
use bevy::{asset::embedded_asset, prelude::*};
use froglight::plugins::{AppPlugins, LoadingPlugin};

fn main() {
    // Create a new bevy App
    let mut app = App::new();

    // Add the FrogLight app plugins
    app.add_plugins(AppPlugins);

    // Add your own custom plugins here!
    // app.add_plugins(MyCustomPlugin);

    // Run the client!
    app.run();
}
```

> [!Tip]
> For more information, see the following links:
>  * [Bevy Quick Start Guide](https://bevyengine.org/learn/quick-start/getting-started/setup/)
>  * [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
>  * [Bevy Examples](https://github.com/bevyengine/bevy/tree/latest/examples)
>  * [FrogLight Examples](examples)
> 
> For documentation, see:
>  * [Bevy Docs](https://docs.rs/bevy/latest/bevy/)
>  * [FrogLight Docs](https://github.com/EightFactorial/FrogLight/)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
