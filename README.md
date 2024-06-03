<h1 align="center">FrogLight</h1>
<p align="center">A Minecraft client/server library written in Rust using <a href="https://bevyengine.org/">Bevy</a></p>

<p align="center">
  <a href="https://github.com/EightFactorial/FrogLight"><img alt="Documentation" src="https://img.shields.io/badge/docs-main-green.svg"></a>  
  <img alt="License" src="https://img.shields.io/badge/license-MIT/Apache--2.0---?color=blue">
  <a href="https://github.com/EightFactorial/FrogLight/actions"><img alt="Tests" src="https://github.com/EightFactorial/FrogLight/actions/workflows/testing.yml/badge.svg"></a>
</p>

## About

FrogLight is currently in a **very early alpha state**. Not everything is functional yet!

> [!Caution]
> Some servers do not allow modified clients!
> 
> **I am not responsible if your account gets banned!**

## Version Support

FrogLight supports multiple different versions, including:
- 1.20.0 - 1.20.1 (**TEMP**)

> [!Note]
> Support for other versions will be added after most development is done.

## Usage

FrogLight can added as a dependency in your `cargo.toml` file:

```toml
[dependencies]
bevy = "0.13.0"
froglight = { git = "https://github.com/EightFactorial/FrogLight.git" }
# etc...
```

> [!Important]
> If you are getting poor performance, try building your project in `--release` mode!

## Helpful Resources

For more information, see the following links:
  * [Bevy Quick Start Guide](https://bevyengine.org/learn/quick-start/getting-started/setup/)
  * [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
  * [Bevy Examples](https://github.com/bevyengine/bevy/tree/latest/examples)
  * [FrogLight Examples](examples)

For documentation, see:
  * [Bevy Docs](https://docs.rs/bevy/latest/bevy/)
  * [FrogLight Docs](https://github.com/EightFactorial/FrogLight/)

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
