# FrogLight

[![Tests](https://github.com/EightFactorial/FrogLight/actions/workflows/test.yml/badge.svg)](https://github.com/EightFactorial/FrogLight/actions)

A **highly work-in-progress** Minecraft client written in Rust using **[Bevy](https://bevyengine.org/)**.

Currently *aiming* to support:
- 1.20.1

Froglight is written to support multiple protocol versions, however, only one is implemented to focus on the actual functionality.

Support for more versions will be added at some point in the future.

> [!Warning]
> While the goal is to emulate Minecraft as closely as possible, it will *never* be perfect.
> 
> Modded clients, including this one, **will get you banned** from servers with anticheat.
> 
> You have been warned!

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
