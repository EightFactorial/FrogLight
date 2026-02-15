<h1 align="center">FrogLight</h1>
<p align="center">A collection of Minecraft libraries written in Rust</p>

<p align="center">
  <a href="https://github.com/EightFactorial/FrogLight"><img alt="Documentation" src="https://img.shields.io/badge/docs-main-green.svg"></a>
  <img alt="License" src="https://img.shields.io/badge/license-MIT/Apache--2.0---?color=blue">
  <a href="https://github.com/EightFactorial/FrogLight/actions"><img alt="Tests" src="https://github.com/EightFactorial/FrogLight/actions/workflows/testing.yml/badge.svg"></a>
</p>

## About

FrogLight is a set of libraries for writing Minecraft clients and servers.

Almost every library is written to be used independently, though most provide additional features when used together.

Additionally, some libraries provide plugins for the [Bevy](https://github.com/bevyengine/bevy/) engine.

> [!Note]
> This project is very early in development and far from being stable!
>
> If you have any issues or questions feel free to open an issue or discussion!

> [!Caution]
> Many servers do not allow modified clients!
>
> ### I am not responsible for banned accounts

## Version Support

Support for Minecraft versions is enabled using features.

| Version | Feature |
| ------- | ------- |
| 26.1.x  | `v26_1` |

If you are writing a library on top of FrogLight, consider putting version-specific behavior behind features.

This allows users to cut down on compile times by only enabling the versions they need.

## Usage

First, create a new Rust project and add the following to your `Cargo.toml`:

```toml
[dependencies]
bevy = { version = "0.18" }
froglight = { git = "https://github.com/EightFactorial/FrogLight.git", features = ["bevy", "v26_1"] }
```

Then create a new Bevy app and add your plugins:

```rust
use bevy::prelude::*;
use froglight::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrogLightPlugins)
        .add_plugins(MyCustomPlugin)
        .run()
}

struct MyCustomPlugin;

impl Plugin for MyCustomPlugin {
    fn build(&self, _app: &mut App) {
        // Do stuff here
    }
}
```

See [Bevy](https://github.com/bevyengine/bevy) for more information on how to write a Bevy app.

## FAQ

### - What about Minecraft 1.xx?

This project generates a lot of code using Minecraft's deobfuscated jar files, which have only been available starting with Minecraft 26.1.0.

If you want to join a server running on an unsupported version, you can try [ViaProxy](https://github.com/ViaVersion/ViaProxy).

---

### - Can I use this to write a bot?

Yes! See the [FrogBot](examples/frogbot) example for a simple bot that can be used as a template for your own.

> [!Caution]
> I am not responsible for any accounts banned for botting!

---

### - Can I use this to write a client?

Yes! See the [FrogClient](https://github.com/EightFactorial/FrogClient) repository.

> [!Note]
> While FrogLight is Apache/MIT licensed, FrogClient is GPLv3 licensed.

---

### - Can I use this to write a server?

Maybe! A lot of the basic components are written, but there are likely a lot of features missing that you would want in a server.

Writing a server is a far more complex task than writing a client, so good luck!

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
