<h1 align="center">FrogLight</h1>
<p align="center">A Minecraft client written in Rust using <a href="https://bevyengine.org/">Bevy</a></p>

<p align="center">
  <a href="https://github.com/EightFactorial/FrogLight"><img alt="Documentation" src="https://img.shields.io/badge/docs-master-blue.svg"></a>  
  <img alt="License" src="https://img.shields.io/badge/license-MIT/Apache--2.0---?color=blue">
  <a href="https://github.com/EightFactorial/FrogLight/actions"><img alt="Tests" src="https://github.com/EightFactorial/FrogLight/actions/workflows/nextest.yml/badge.svg"</a>
</p>

## About

FrogLight is currently in a **very early alpha state**. Not everything is functional yet!

> [!Warning]
> Modded clients, including this one, will get you **banned** from servers with anticheat!
> 
> You have been warned!

## Supported Versions

FrogLight is multiprotocol and supports release versions 1.20.0+, including:
- 1.20.0 and 1.20.1 (763)
- ~1.20.2~ (764)
- ~1.20.3 and 1.20.4~ (765)

> [!Note]
> Version support will be added after core client functions
> 
> Multi-protocol is already built in, but packets have not been created!

## Configuration

All configuration files and resource packs can be found in the config folder

| OS | Default Folder |
| --- | --- |
| Windows | `%APPDATA%/FrogLight` |
| Linux | `$XDG_CONFIG_HOME/FrogLight` |
| MacOS | `~/Library/Application Support/FrogLight` |

## Building

Like most Rust projects, compilation is done using the `cargo` command.

```sh
# Clone and enter the repo
git clone --depth 1 https://github.com/EightFactorial/FrogLight
cd FrogLight

# Build the client, or
cargo build --release
# Build and run the client
cargo run --release
```

This will take some time and a lot of storage space

The built client will be located at `{PROJECT_DIR}/target/release/froglight`

## Plugins

FrogLight was written with customization in mind.

For example, you can replace the loading screen art with your own!

```rust
use bevy::{asset::embedded_asset, prelude::*};
use froglight_client::{plugins::LoadingPlugin, prelude::*};

fn main() {
  // Create a new bevy App
  let mut app = App::new();

  // Place the image next to the main.rs file
  // and embed it inside the application
  embedded_asset!(app, "my_new_icon.png");

  // Create the AppPlugins group
  let mut plugins = AppPlugins::build();

  // Set the new image path
  // If not using this repository, replace `froglight` with the name of your crate
  plugins = plugins.set(LoadingPlugin::new("embedded://froglight/my_new_icon.png"));

  // Add the plugins and run!
  app.add_plugins(plugins).run();
}
```
This is just a basic example, everything is replacable! Don't like the GUI, import your own!

See the [Bevy Book](https://bevyengine.org/learn/book/getting-started/plugins/) and [Froglight Docs](https://github.com/EightFactorial/FrogLight/) for more information.

## Contributing

Too much of the internal structure is undecided, crates ~may~ *will* be rewritten!

In it's current state, FrogLight is not accepting PRs *except* for:
- ~New versions~ (Soon™)
- Packet fixes

## New Versions

Support for new versions may be automatically generated using the included `just` recipes

There is no guarentee new versions will work out of the box, please test *extensively* before making a PR!

```sh
# Install just if needed
cargo install just

# Run the generator tool
# Manual cleanup is required!
just tools generate {VERSION}
```
