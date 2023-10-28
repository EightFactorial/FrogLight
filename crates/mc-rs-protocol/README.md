# MC-RS Protocol

This crate contains the low-level code for communicating with Minecraft servers.

In particular, it contains the following:
  - A `Connection<Version, State>` type that handles the connection
  - Code for reading and writing to and from bytes
  - Version-specific packet definitions
