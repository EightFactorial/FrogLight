<h1 align="center">FrogLight</h1>
<p align="center">A collection of Minecraft libraries written in Rust</p>

<p align="center">
  <a href="https://github.com/EightFactorial/FrogLight"><img alt="Documentation" src="https://img.shields.io/badge/docs-main-green.svg"></a>  
  <img alt="License" src="https://img.shields.io/badge/license-MIT/Apache--2.0---?color=blue">
  <a href="https://github.com/EightFactorial/FrogLight/actions"><img alt="Tests" src="https://github.com/EightFactorial/FrogLight/actions/workflows/testing.yml/badge.svg"></a>
</p>

## About

FrogLight is a set of libraries for creating Minecraft clients. Each crate is designed to be used independently, but plugins are provided for the [Bevy](https://github.com/bevyengine/bevy/) engine.

It is currently in a **very early alpha state**. Most things do not work yet!

> [!Caution]
> Some servers do not allow modified clients!
> 
> ### !! I am not responsible for banned accounts !!

## Version Support

FrogLight has support for:
- v1.21.0 - v1.21.1

## Usage

FrogLight can added as a dependency in your `cargo.toml` file:

```toml
[dependencies]
bevy = "0.15"
froglight = { git = "https://github.com/EightFactorial/FrogLight.git" }
# etc...
```

See the [examples](examples) directory for how to use the library.

## Client

See [FrogLight Client](https://github.com/EightFactorial/FrogLight/) for more information.

## Helpful Resources

For more information, see the following links:
  * [Bevy Quick Start Guide](https://bevyengine.org/learn/quick-start/getting-started/setup/)
  * [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
  * [Bevy Examples](https://github.com/bevyengine/bevy/tree/latest/examples)
  * [FrogLight Examples](examples)

For documentation, see:
  * [Bevy Docs](https://docs.rs/bevy/latest/bevy/)
  * ~~[FrogLight Docs](https://github.com/EightFactorial/FrogLight/)~~

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
