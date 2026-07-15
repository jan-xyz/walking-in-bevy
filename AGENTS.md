# AGENTS.md

Guidance for AI agents working in this repository.

## Project Overview

A 3D multiplayer game built with the [Bevy](https://bevyengine.org/) game engine (Rust). Players
move around as a swappable Blender-exported donut (glTF) or procedural cube, with physics-based
movement/jumping and networked multiplayer via [lightyear](https://github.com/cBournhonesque/lightyear).

Three binaries share one library crate (`walking_in_bevy`, `src/lib.rs`):

- `src/main.rs` (bin `main`, default) — local split-screen mode, two players on one keyboard, no networking.
- `src/server.rs` (bin `server`) — headless authoritative server (`MinimalPlugins`, no rendering).
- `src/client.rs` (bin `client`) — networked client that connects to `server` over UDP (netcode) and renders.

Run `cargo run --bin server` then `cargo run --bin client` (one or more) for networked play, or
plain `cargo run` for local split-screen. By default `client` connects to the deployed server at
`imperius.janxyz.de:5000` (DNS-resolved at startup, see `src/client.rs::resolve_server_addr`);
override with the `SERVER_ADDR` env var (e.g. `SERVER_ADDR=127.0.0.1:5000`) to point at a local
`server` instance instead. The server itself always listens on `0.0.0.0:5000` (hardcoded in
`src/server.rs`).

## Commands

```sh
cargo build                     # build everything
cargo run                       # local split-screen (bin `main`)
cargo run --bin server          # headless server
cargo run --bin client          # networked client
cargo test                      # unit tests
cargo fmt --check               # formatting check (CI-enforced)
cargo fmt                       # apply formatting
cargo clippy -- -D warnings     # lint (CI-enforced, warnings are errors)
git lfs pull                    # fetch binary assets (assets/export.glb) if missing
```

A `Makefile` wraps the common ones (`make build`, `make test`, `make fmt`, `make lint`, `make run`,
`make run-server`, `make run-client`) plus deployment helpers (`make docker-build`, `make docker-run`,
`make k8s-render`, `make k8s-deploy`). Prefer these over typing raw `cargo`/`docker`/`kubectl`
invocations so behavior stays consistent with CI.

Always run `cargo fmt` and `cargo clippy -- -D warnings` (or `make fmt lint test`) before considering
a change done — CI (`.github/workflows/rust.yml`) fails the build on either. CI also runs
`cargo llvm-cov` for coverage (not required locally).

Rust toolchain is pinned via `rust-toolchain.toml` (channel `1.97`, includes `clippy`+`rustfmt`)
so cargo will auto-fetch that exact toolchain.

Assets (`assets/export.glb`) are stored via **Git LFS** — see `.gitattributes`. If a fresh clone
is missing binary assets or the glTF fails to load, run `git lfs pull`.

## Architecture / Code Organization

```
src/
  main.rs      - local split-screen entry point (LocalPlugins)
  server.rs    - headless server entry point (ServerPlugins + ServerPlugin)
  client.rs    - networked client entry point (ClientPlugins + ClientPlugin)
  lib.rs       - just `pub mod plugins;`
  plugins/
    mod.rs     - defines the three top-level PluginGroups: LocalPlugins, ServerPlugin, ClientPlugin
    core.rs    - CorePlugin (ground + lights, local/client) / ServerCorePlugin (ground only, server)
    camera.rs  - follow-camera + split-screen viewport layout, driven by observers
    input.rs   - leafwing-input-manager PlayerActions enum + default keymaps (WASD vs arrows)
    physics.rs - Avian3d + bevy-tnua character controller wiring (local PhysicsPlugin vs networked NetworkPlugin)
    network.rs - lightyear NetworkPlugin: registers replicated components, prediction/interpolation/rollback rules
    player.rs  - Player component, PlayerControlScheme (tnua), spawn/movement/rotation logic, tests
    player/
      model.rs - swappable donut/cube visual model, color sync, model-swap input handling
```

`plugins/mod.rs` composes three distinct `PluginGroup`s depending on binary:

- `LocalPlugins` (used by `main.rs`): `core::CorePlugin`, `player::PlayerPlugin`, `physics::PhysicsPlugin`,
  `camera::CameraPlugin`, `input::InputPlugin`.
- `ServerPlugin` (used by `server.rs`): `core::ServerCorePlugin`, `physics::NetworkPlugin`, `network::NetworkPlugin`.
- `ClientPlugin` (used by `client.rs`): `core::CorePlugin`, `player::NetworkPlugin`, `physics::NetworkPlugin`,
  `network::NetworkPlugin`, `camera::CameraPlugin`.

Note the naming overload: several modules (`player`, `physics`) each define **both** a local-only
plugin (`PlayerPlugin`, `PhysicsPlugin`) and a networked variant confusingly also named `NetworkPlugin`.
When editing, check the module path (`player::NetworkPlugin` vs `network::NetworkPlugin` vs
`physics::NetworkPlugin`) — they are unrelated types with the same short name.

### Control/data flow

- **Local mode**: `player::spawn_players` spawns two `Player` bundles directly with hardcoded configs
  (position/color/keymap), full physics runs every frame via `PhysicsPlugin`.
- **Networked mode**: server spawns a `Player` on `Connected` (see `server.rs::on_client_connected`),
  attaching `Replicate`/`PredictionTarget`/`InterpolationTarget`. Client reacts to the replicated
  entity via an `Add<Player>` observer (`client.rs::on_player_added`) which inserts local-only
  components (`TnuaController`, `Collider`, `FrameInterpolate<...>`) — these are **not** replicated,
  only added client-side once the entity mirrors in. Client input is driven by an `ActionState<PlayerActions>`
  that lightyear replicates to the server; the same `apply_controls` system (in `player.rs`) runs on
  both client (predicted) and server, keyed off `With<Predicted>` on the client to avoid double-applying.
- `network.rs` is where all lightyear replication rules live: which components get prediction,
  interpolation, and the per-component `should_rollback` thresholds (e.g. position rollback only if
  predicted/server states diverge by >= 1.0 unit). If you add a new networked component, register it
  here.
- `FacingAngle` is a custom-replicated angle (not a `Transform`/`Rotation`) with a manual `Diffable<f32>`
  impl (linear diff-based correction) in `network.rs` — used instead of `Rotation` because rotation is
  axis-locked (`LockedAxes::ROTATION_LOCKED`) for the physics body; visual rotation is applied separately
  in `player::apply_visual_rotation` to the child model entity, not the physics body itself.
- Camera follow (`camera.rs`) only attaches to entities with `Predicted` (client) — i.e. your own
  player gets a camera, not other replicated/interpolated players. Viewport splitting recalculates on
  `WindowResized` and whenever a new camera/player is added, via observers (`add_camera_on_player_added`,
  `adjust_viewport_on_camera_added`) rather than a per-frame system.
- Player model swap (`player/model.rs`) is fully event-driven: `SwapModel` action toggles
  `CurrentPlayerModel`, an `Insert<CurrentPlayerModel>` observer despawns the old child model and spawns
  the new one (donut glTF vs procedural cube mesh), and a separate `Update` system
  (`sync_player_model_colors`) walks all descendant entities to keep materials in sync with `PlayerColor`
  after swaps.

## Conventions & Gotchas

- **Table-driven tests**: every test uses a local `TestCase` struct + array + loop, asserting the full
  result with `assert_eq!` (see `player.rs` and `camera.rs`). New test cases for existing behavior go
  into the existing table, not a new `#[test]` fn. `TestCase` fields hold inputs/expected outputs only;
  no transformation logic in the loop body.
- Tests live in a `#[cfg(test)] mod test` (singular, `player.rs`) or `mod tests` (plural, `camera.rs`)
  at the bottom of the same file as the code under test — no separate `tests/` integration dir exists
  currently.
- Pure/testable logic (`movement_direction`, `movement_rotation`) is factored out of the ECS system
  functions (`apply_controls`) specifically so it can be unit tested without spinning up a Bevy `App`;
  follow this pattern for new gameplay math.
- Bevy ECS style used throughout: observers (`add_observer`) for one-shot reactive logic on component
  add/insert, regular systems for per-frame/per-tick logic. `On<Add, T>` / `On<Insert, T>` triggers are
  used heavily instead of `Changed<T>` queries for setup-once logic.
- `#[allow(clippy::type_complexity)]` is used on systems with large tuple `Query` types (an accepted
  pattern here) rather than restructuring the query — follow this when adding similarly complex queries.
- Physics: colliders/mass are added on the physics root entity, not the visual model child; the visual
  model (`PlayerModel`) is purely cosmetic and reparented as a child on every model swap.
- `#[allow(deprecated)]` appears in `network.rs` on `NetworkPlugin::build` — lightyear's leafwing input
  plugin config API is mid-migration; don't "fix" this by silently changing config without checking
  lightyear's current API first.
- Dependency updates are grouped in `.github/dependabot.yml`: all `bevy*`/`avian*`/`iyes*`/`lightyear*`
  crates are bumped together as one group (monthly) since they need to stay version-compatible;
  rust-toolchain and GitHub Actions are separate monthly groups.
- Default binary is `main` (`default-run = "main"` in `Cargo.toml`), so bare `cargo run` launches local
  split-screen, not the client or server.

## Deployment (`--bin server`)

The dedicated server can be containerized and deployed to Kubernetes:

- `Dockerfile` is a two-stage build: `rust:1.97-slim-bookworm` compiles `--bin server` in release
  mode, then the binary is copied into `gcr.io/distroless/cc-debian12:nonroot` (uid/gid 65532,
  matching the Deployment's `securityContext`). The `assets/` directory is intentionally **not**
  copied into the image — the server's plugin group (`ServerPlugin` in `plugins/mod.rs`) never adds
  `player::PlayerPlugin`/`player::NetworkPlugin`, so `player/model.rs`'s glTF-loading observer never
  registers server-side and the donut model is never loaded at runtime.
- The compiled server binary only dynamically links `libc`/`libm`/`libgcc_s` (verified with `ldd`)
  even though the crate pulls in bevy's audio/window/render backends (ALSA, Wayland, X11, Vulkan) as
  compile-time dependencies — the linker's `--as-needed` behavior drops those `DT_NEEDED` entries
  since `MinimalPlugins` never calls into that dead code. If you add real server-side rendering/audio
  usage this may no longer hold; re-run `ldd` on the built binary before trusting the minimal
  distroless base still works.
- `k8s/deployment.yaml` + `k8s/service.yaml` + `k8s/kustomization.yaml`: no `Ingress` is defined —
  lightyear's `netcode` transport is raw UDP, and the cluster's ingress controller (Traefik, used
  elsewhere in the jan-xyz infra) only routes HTTP(S). The `Service` is `type: LoadBalancer` exposing
  UDP port 5000 directly. There are also no `readinessProbe`/`livenessProbe` entries — Kubernetes has
  no built-in UDP probe type and the server exposes no HTTP endpoint to probe.
- `.github/workflows/server-build-and-deploy.yml` builds/pushes to GitHub Container Registry
  (`ghcr.io/<owner>/walking-in-bevy-server`, auth via the built-in `GITHUB_TOKEN`, no registry
  secrets needed) and deploys via `kubectl apply -k k8s/`, gated on a `secrets.KUBECONFIG` (base64
  kubeconfig). It auto-bumps a `server-vX.Y.Z` git tag on every push to `main` that touches
  `src/**`, `Cargo.toml`/`Cargo.lock`, `Dockerfile`, or `k8s/**`, then commits the resolved image tag
  back into `k8s/kustomization.yaml` via `kustomize edit set image`. This mirrors the versioning
  pattern used by the `lilith`/`deckard` services in the separate `janxyz` monorepo, adapted for a
  single-service repo (no path-based job filtering needed).
- Local dev/testing: `make docker-build` builds the image, `make docker-run` runs it with UDP 5000
  published, `make k8s-render` runs `kubectl kustomize k8s/` to sanity-check the manifests without
  applying them, `make k8s-deploy` applies them to whatever cluster your current kubeconfig context
  points at.
