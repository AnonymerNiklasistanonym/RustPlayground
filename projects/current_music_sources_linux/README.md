## Setup

```sh
cargo new current_music_sources_linux
cd current_music_sources_linux
# interact with bus
cargo add zbus
# serialize data
cargo add serde --features derive
# serialize data to JSON
cargo add serde_json
# async execution
cargo add tokio --features macros,rt-multi-thread
cargo add futures
# parse BASE64 encoded image strings to image files
cargo add base64
# create hash of strings
cargo add xxhash-rust
# create JSON schema of serialized output
cargo add schemars
# easy command line argument parsing
cargo add clap
```

## Idea

### Media Player Remote Interfacing Specification (MPRIS) D-Bus Interface Specification

> The Media Player Remote Interfacing Specification is a standard D-Bus interface which aims to provide a common programmatic API for controlling media players.
> It provides a mechanism for discovery, querying and basic playback control of compliant media players, as well as a tracklist interface which is used to add context to the active media item.
>
> https://specifications.freedesktop.org/mpris/latest/

Meaning reading and parsing this information should give all current media players.

### D-Bus

> D-Bus is a system for interprocess communication (IPC)
> [...]
> D-Bus is designed for two specific cases:
> 1. Communication between desktop applications in the same desktop session; to allow integration of the desktop session as a whole, and address issues of process lifecycle (when do desktop components start and stop running).
> 2. Communication between the desktop session and the operating system, where the operating system would typically include the kernel and any system daemons or processes.
>
> https://dbus.freedesktop.org/doc/dbus-tutorial.html
