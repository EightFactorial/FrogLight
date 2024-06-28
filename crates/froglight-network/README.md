# Froglight Network

A basic networking implementation built on [Froglight Protocol](../froglight-protocol/).

Supports both `Client -> Server` and `Server -> Client` connections.

## Version Support

Basic networking is automatically implemented for all versions supported by [Froglight Protocol](../froglight-protocol/).

## Bevy Plugins

Optionally included are a few [plugins](src/plugin.rs).

These add the ability to ping servers, connect to servers, and send and receive packets.

See [`server-status`](examples/server-status/main.rs) and [`server-login`](examples/server-login/main.rs) for basic examples on how to use these plugins.

> [!Note]
> These plugins add networking *support*, but do not add any networking systems.
>
> How you manage sending and receiving packets is up to you!

## Plugin Support

To add plugin support for a version, all thats needed are the following basic traits:
- [HandshakeState](src/network/networking/handshake/mod.rs)
- [StatusState](src/network/networking/status/mod.rs)
- [LoginState](src/network/networking/login/mod.rs)
- [ConfigurationState](src/network/networking/configuration/mod.rs)
- [PlayState](src/network/networking/play/mod.rs)
