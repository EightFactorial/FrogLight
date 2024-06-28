# Froglight Protocol

A barebones implementation of reading and writing types and packets in the Minecraft format.

This crate contains no networking logic, only logic for reading and writing bytes and data structures!

Most likely you are looking for the [Froglight Network](../froglight-network/) crate, which implements a basic [`Connection`](../froglight-network/src/connection/mod.rs) using this protocol.

## Version Support

Currently supported protocols:
 - 1.21.0

Support for new versions can be added by updating the `generator.toml` file and running the `generate` tool:
```bash
# Note: All of the required fields are prepopulated
# To generate packets, run
just tools generate -vv -m Packets

# For more information, run
just tools generate --help
```

All preparation will be done automatically, and the tool will attempt to generate new code.

> [!Warning]
> This code will not be pretty, and packets will need to be manually cleaned up and double checked.
> 
> **Do not generate code and complain it doesn't work!**

> [!Note]
> This tool uses packet `CODEC`s, which are fairly recent additions.
>
> This means packet generation only works for versions starting around `1.20.5` and onward.
>
> For unsupported versions, consider using a compatibility layer such as [ViaProxy](`https://github.com/ViaVersion/ViaProxy`).
