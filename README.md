<h1 align="center">FrogLight</h1>
<p align="center">A Minecraft client written in Rust using <a href="https://bevyengine.org/">Bevy</a></p>

<p align="center">
  <a href="https://github.com/EightFactorial/FrogLight"><img alt="Documentation" src="https://img.shields.io/badge/docs-main-green.svg"></a>  
  <img alt="License" src="https://img.shields.io/badge/license-MIT/Apache--2.0---?color=blue">
  <a href="https://github.com/EightFactorial/FrogLight/actions"><img alt="Tests" src="https://github.com/EightFactorial/FrogLight/actions/workflows/nextest.yml/badge.svg"></a>
</p>

## About

FrogLight is currently in a **very early alpha state**. Not everything is functional yet!

Because FrogLight does not come with any assets, you will need to provide your own resource packs!



> [!Warning]
> Modded clients, including this one, will get you **banned** from servers with anticheat!
> 
> You have been warned!

## Version Support

FrogLight is multiprotocol and supports release versions 1.20.0+, including:
- 1.20.0 and 1.20.1 (763)
- ~1.20.2~ (764)
- ~1.20.3 and 1.20.4~ (765)

> [!Note]
> Version support will be added after core client functions
> 
> Multi-protocol is already built in, but packets have not been created!

---

Support for new versions may be automatically generated using the included `just` recipes

There is no guarentee new versions will work out of the box, please test *extensively* before making a PR!

```sh
# Install just if needed
cargo install just

# Run the generator tool
# Manual cleanup is required!
just tools generate {VERSION}
```

## Configuration Files

All configuration files and resource packs are stored in the following directories:

| OS      | Default Folder                            |
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
cargo build --release
# Build and run the client
cargo run --release
```

This may take a while and requires a couple of gigabytes of space!

The built client will be located at `./target/release/froglight`

> [!Important]
> Make sure you are building in `release` mode, as `debug` mode will be very slow!

## Plugins

FrogLight was written with customization in mind.

For example, here is how to embed an image and use it on the loading screen:

```rust
use bevy::{asset::embedded_asset, prelude::*};
use froglight_client::{plugins::LoadingPlugin, prelude::*};

fn main() {
  // Create a new bevy App
  let mut app = App::new();

  // Create the AppPlugins group
  let mut plugins = AppPlugins::build();

  // Place the image next to the main.rs file
  // and embed it inside the application
  embedded_asset!(app, "my_new_icon.png");

  // Set the newly embedded image's path
  // If not using this repository, replace `froglight` with the name of your crate
  plugins = plugins.set(LoadingPlugin::new("embedded://froglight/my_new_icon.png"));

  // Add the plugins and run!
  app.add_plugins(plugins).run();
}
```
This is just a basic example, almost everything is modular and can be replaced.

> [!Tip]
> See the [Bevy Book](https://bevyengine.org/learn/book/getting-started/plugins/) and [Froglight Docs](https://github.com/EightFactorial/FrogLight/) for more information.

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
