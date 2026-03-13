# Walking in Bevy

A 3D multiplayer game built with [Bevy](https://bevyengine.org/). Players run around as donuts (or cubes), jump, and swap models on the fly — with physics-based movement and networked multiplayer.

## Features

- **Networked multiplayer** — client/server architecture powered by [lightyear](https://github.com/cBournhonesque/lightyear) with client-side prediction, server reconciliation, and visual interpolation
- **Local split-screen** — two players share a keyboard with automatic viewport layout
- **Physics-based movement & jumping** — powered by [Avian3d](https://github.com/Jondolf/avian) + [bevy-tnua](https://github.com/idanarye/bevy-tnua) character controller
- **Swappable player models** — toggle between a Blender-exported donut (glTF) and a procedural cube at runtime
- **Input mapping** — clean action-based input via [leafwing-input-manager](https://github.com/leafwing-studios/leafwing-input-manager)

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/)
- [Git LFS](https://git-lfs.com/) (for binary assets)

### Run (Networked Multiplayer)

Start the server, then connect one or more clients:

```sh
git lfs pull   # fetch binary assets if not already present

# Terminal 1: start the server
cargo run --bin server

# Terminal 2: start a client
cargo run --bin client
```

Each client controls one player. The server spawns players at different positions with different colors. Your own player uses client-side prediction; other players are smoothly interpolated from the server state.

### Run (Local Split-Screen)

```sh
cargo run
```

Two players share a keyboard with split-screen viewports.

### Test

```sh
cargo test
```

### Lint / Format

```sh
cargo fmt --check
cargo clippy -- -D warnings
```

## Tutorials & References

- [Blender 4 Donut Tutorial](https://www.youtube.com/watch?v=4haAdmHqGOw)
- [Blender Character Design Playlist](https://www.youtube.com/watch?v=UAami_DhnTA&list=PLC7nmYI-cbT1gLvOzU-pcIZKbPezbRSyz)
- [Chris Biscardi's Bevy Channel](https://www.youtube.com/@chrisbiscardi)
