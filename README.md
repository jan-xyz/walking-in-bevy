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

A server is deployed at `imperius.janxyz.de:5000` — running `cargo run --bin client` connects there
by default. Override with the `SERVER_ADDR` environment variable to point at a local or different
server instead:

```sh
git lfs pull   # fetch binary assets if not already present

# connect to the deployed server
cargo run --bin client

# or run everything locally
# Terminal 1: start the server
cargo run --bin server

# Terminal 2: start a client pointed at localhost
SERVER_ADDR=127.0.0.1:5000 cargo run --bin client
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

## Deployment

The server can be built as a container image and deployed to Kubernetes. The image is packaged from
a prebuilt binary (no compilation happens inside Docker), so `make docker-build` only produces a
runnable image on a Linux x86_64 host — do this in CI or on a Linux x86_64 machine:

```sh
make docker-build   # builds walking-in-bevy-server:latest
make docker-run      # runs it locally with UDP 5000 published
make k8s-render      # renders k8s/ manifests via kustomize (no cluster required)
make k8s-deploy       # kubectl apply -k k8s/ against your current kube context
```

`.github/workflows/server-build-and-deploy.yml` builds the server binary natively on the GitHub
Actions runner (`ubuntu-latest`, linux/x86_64 — matching the cluster architecture, so no
cross-compilation is needed) using `Swatinem/rust-cache` for fast incremental builds, packages it
into the container image, pushes it to GitHub Container Registry, and deploys to Kubernetes
automatically on every push to `main` that touches server-related files. It requires a `KUBECONFIG`
repository secret (base64-encoded kubeconfig) to be able to deploy.

## Tutorials & References

- [Blender 4 Donut Tutorial](https://www.youtube.com/watch?v=4haAdmHqGOw)
- [Blender Character Design Playlist](https://www.youtube.com/watch?v=UAami_DhnTA&list=PLC7nmYI-cbT1gLvOzU-pcIZKbPezbRSyz)
- [Chris Biscardi's Bevy Channel](https://www.youtube.com/@chrisbiscardi)
