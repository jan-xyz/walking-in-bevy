# Walking in Bevy

A 3D local-multiplayer game built with [Bevy](https://bevyengine.org/). Two players share a keyboard, run around as donuts (or cubes), jump, and swap models on the fly — all with split-screen cameras and physics-based movement.

## Features

- **Two-player split-screen** — each player gets their own camera with automatic viewport layout
- **Physics-based movement & jumping** — powered by [Avian3d](https://github.com/Jondolf/avian) + [bevy-tnua](https://github.com/idanarye/bevy-tnua) character controller
- **Swappable player models** — toggle between a Blender-exported donut (glTF) and a procedural cube at runtime
- **Input mapping** — clean action-based input via [leafwing-input-manager](https://github.com/leafwing-studios/leafwing-input-manager)

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/)
- [Git LFS](https://git-lfs.com/) (for binary assets)

### Run

```sh
git lfs pull   # fetch binary assets if not already present
cargo run
```

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
