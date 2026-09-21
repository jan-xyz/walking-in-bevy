# AGENTS.md

Guidance for AI agents working in this repository.

## Project Overview

A 3D multiplayer game built with the [Bevy](https://bevyengine.org/) game engine (Rust). Players
move around as a swappable Blender-exported donut (glTF) or procedural cube, with physics-based
movement/jumping and networked multiplayer via [lightyear](https://github.com/cBournhonesque/lightyear).

Cargo workspace with three crates:

- `shared/` (lib `walking-in-bevy-shared`) — code used by both sides: player components/movement,
  physics wiring, lightyear replication rules, level setup.
- `server/` (bin `server`, the workspace default) — headless authoritative server
  (`MinimalPlugins`, no rendering).
- `client/` (bin `client`) — networked client that connects to `server` over UDP (netcode) and renders.

Run `cargo run --bin server` then `cargo run --bin client` (one or more) for networked play;
`make run` runs the server. By default `client` connects to
the deployed server at `imperius.janxyz.de:5000` (DNS-resolved at startup, see
`client/src/main.rs::resolve_server_addr`); override with the `SERVER_ADDR` env var
(e.g. `SERVER_ADDR=127.0.0.1:5000`) to point at a local `server` instance instead. The server
itself always listens on `0.0.0.0:5000` (hardcoded in `server/src/main.rs`).

Dependency versions are centralized in the root `Cargo.toml` under `[workspace.dependencies]`;
member crates use `<dep>.workspace = true` (crate-specific cargo features, e.g. lightyear's
`netcode`/`udp`, are declared in the member's own `Cargo.toml` on top of the workspace baseline).

## Commands

```sh
cargo build                     # build everything
cargo run --bin server          # headless server (also `make run`)
cargo run --bin client          # networked client
cargo test                      # unit tests
cargo fmt --check               # formatting check (CI-enforced)
cargo fmt                       # apply formatting
cargo clippy -- -D warnings     # lint (CI-enforced, warnings are errors)
```

A `Makefile` wraps the common ones (`make build`, `make test`, `make fmt`, `make lint`, `make run`,
`make run-server`, `make run-client`) plus deployment helpers (`make docker-build`, `make docker-run`,
`make k8s-render`, `make k8s-deploy`). Prefer these over typing raw `cargo`/`docker`/`kubectl`
invocations so behavior stays consistent with CI.

Always run `cargo fmt` and `cargo clippy -- -D warnings` (or `make fmt lint test`) before considering
a change done — CI (`.github/workflows/rust.yml`) fails the build on either. CI also runs
`cargo llvm-cov` for coverage (not required locally).

Rust toolchain is pinned via `rust-toolchain.toml` (includes `clippy`+`rustfmt`)
so cargo will auto-fetch that exact toolchain.

Assets live in `client/assets/` (the client crate is the only consumer). Bevy resolves the asset
root from `CARGO_MANIFEST_DIR` (which `cargo run` sets to the crate being run), so keeping the
assets inside the client crate is what makes `cargo run --bin client` find them from any CWD.
`.gitattributes` configures Git LFS for media types, but no files are currently LFS-tracked.

## Architecture / Code Organization

```
shared/src/
  lib.rs       - module declarations
  core.rs      - LevelMeshPlugin (ground) + LevelAmbientPlugin (lights, client-only)
  physics.rs   - PhysicsPlugin: Avian3d + bevy-tnua character controller wiring
  network.rs   - lightyear NetworkPlugin: leafwing input plugin, avian replication mode,
                 which components are replicated/predicted
  player.rs    - Player/FacingAngle/PlayerColor/CurrentPlayerModel components, PlayerActions,
                 PlayerControlScheme (tnua), player_bundle, apply_controls, movement math, tests
client/src/
  main.rs      - entry point: ClientPlugins + ClientPlugin, connect_to_server, on_player_added,
                 predicted apply_controls
  camera.rs    - follow-camera + split-screen viewport layout, driven by observers, tests
  input.rs     - leafwing-input-manager InputPlugin + default WASD keymap
  model.rs     - swappable donut/cube visual model, color sync, model-swap input handling
server/src/
  main.rs      - entry point: ServerPlugins + ServerPlugin, start_server, on_client_connected
                 (spawns replicated Player bundles), on_new_client
```

Both binaries compose a `PluginGroup` from the shared plugins plus their own crate-local plugins:

- `ServerPlugin` (in `server/src/main.rs`): `shared::{core::LevelMeshPlugin, physics::PhysicsPlugin, network::NetworkPlugin}`.
- `ClientPlugin` (in `client/src/main.rs`): `shared::{core, physics, network}` plus
  `model::ModelPlugin` and `camera::CameraPlugin`.

### Control/data flow

- Server spawns a `Player` on `Connected` (see `server/src/main.rs::on_client_connected`) via
  `player_bundle`, attaching `Replicate`/`PredictionTarget`/`InterpolationTarget`/`ControlledBy`.
  Client reacts to the replicated entity via an `Add<Player>` observer
  (`client/src/main.rs::on_player_added`) which inserts local-only components
  (`TnuaController`, `Collider`, `FrameInterpolate`, input map) — these are **not** replicated,
  only added client-side once the entity mirrors in. Client input is driven by an
  `ActionState<PlayerActions>` that lightyear replicates to the server; the same `apply_controls`
  system (in `shared/src/player.rs`) runs on both server and client, keyed off `With<Predicted>`
  on the client to avoid double-applying.
- `shared/src/network.rs` is where all lightyear replication rules live (leafwing input plugin,
  avian replication mode, which components are replicated/predicted). If you add a new networked
  component, register it here.
- `FacingAngle` is a custom-replicated angle (not a `Transform`/`Rotation`) — used instead of
  `Rotation` because rotation is axis-locked (`LockedAxes::ROTATION_LOCKED`) for the physics
  body; visual rotation is applied separately in `client/src/model.rs::apply_visual_rotation`
  to the child model entity, not the physics body itself.
- Camera follow (`client/src/camera.rs`) only attaches to entities with `Predicted` (client) —
  i.e. your own player gets a camera, not other replicated/interpolated players. Viewport
  splitting recalculates on `WindowResized` and whenever a new camera/player is added, via
  observers (`add_camera_on_player_added`, `adjust_viewport_on_camera_added`) rather than a
  per-frame system.
- Player model swap (`client/src/model.rs`) is fully event-driven: `SwapModel` action toggles
  `CurrentPlayerModel`, an `Insert<CurrentPlayerModel>` observer despawns the old child model
  and spawns the new one (donut glTF vs procedural cube mesh), and a separate `Update` system
  (`sync_player_model_colors`) walks all descendant entities to keep materials in sync with
  `PlayerColor` after swaps.

## Conventions & Gotchas

- **Table-driven tests**: every test uses a local `TestCase` struct + array + loop, asserting the full
  result with `assert_eq!` (see `shared/src/player.rs` and `client/src/camera.rs`). New test cases
  for existing behavior go into the existing table, not a new `#[test]` fn. `TestCase` fields hold
  inputs/expected outputs only; no transformation logic in the loop body.
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
- `#[allow(deprecated)]` appears in `shared/src/network.rs` on `NetworkPlugin::build` — lightyear's
  leafwing input plugin config API is mid-migration; don't "fix" this by silently changing config
  without checking lightyear's current API first.
- Dependency updates are grouped in `.github/dependabot.yml`: all `bevy*`/`avian*`/`iyes*`/`lightyear*`
  crates are bumped together as one group (monthly) since they need to stay version-compatible;
  rust-toolchain and GitHub Actions are separate monthly groups.
- `make run` launches the server (`run: cargo run --bin server` in the `Makefile`); bare
  `cargo run` doesn't work from the workspace root (virtual manifest, multiple binaries).

## Deployment (`--bin server`)

The dedicated server can be containerized and deployed to Kubernetes:

- **No cross-compilation, deliberately.** GitHub Actions' `ubuntu-latest` runners are linux/x86_64,
  and the target cluster is linux/x86_64 too — builder arch and deploy arch already match. The CI
  workflow therefore runs a plain native `cargo build --release --bin server` on the runner (cached
  with `Swatinem/rust-cache@v2`, the standard/most-effective Rust build cache) and `Dockerfile` does
  **no compilation at all** — it's a single `COPY` of the prebuilt binary into
  `gcr.io/distroless/cc-debian12:nonroot` (uid/gid 65532, matching the Deployment's
  `securityContext`). This was tried the other way first (multi-stage Docker build compiling inside
  the container, `cross`/QEMU for arch parity) and abandoned: it added real complexity (emulated
  builds are slow and BuildKit's registry/GHA cache is far less effective than `Swatinem/rust-cache`
  for a dependency-heavy crate like this one) to solve a cross-compilation problem that doesn't
  actually exist here. If the cluster architecture ever changes, or CI moves to non-x86_64 runners,
  this assumption needs revisiting.
- The `client/assets/` directory is intentionally **not** copied into the image — the server crate never
  adds the client's `ModelPlugin`, so `client/src/model.rs`'s glTF-loading observer never registers
  server-side and the donut model is never loaded at runtime.
- The compiled server binary only dynamically links `libc`/`libm`/`libgcc_s` (verified with `ldd`)
  even though the crate pulls in bevy's audio/window/render backends (ALSA, Wayland, X11, Vulkan) as
  compile-time dependencies — the linker's `--as-needed` behavior drops those `DT_NEEDED` entries
  since `MinimalPlugins` never calls into that dead code. If you add real server-side rendering/audio
  usage this may no longer hold; re-run `ldd` on the built binary before trusting the minimal
  distroless base still works.
- The Docker build context is the `dist/` directory (just the single `server` binary), not the repo
  root — `make server-binary` (or the CI step of the same name) builds the release binary and copies
  it to `dist/server` before `docker build` ever runs. Building `dist/server` only produces a
  runnable Linux binary when run on a Linux x86_64 host; running `make docker-build` on macOS/Windows
  will build a container with a binary that won't execute (wrong OS/format) — do server container
  builds in CI or on a Linux x86_64 dev box.
- `k8s/deployment.yaml` + `k8s/service.yaml` + `k8s/kustomization.yaml`: no `Ingress` is defined —
  lightyear's `netcode` transport is raw UDP, and the cluster's ingress controller (Traefik, used
  elsewhere in the jan-xyz infra) only routes HTTP(S). The `Service` is `type: LoadBalancer` exposing
  UDP port 5000 directly. There are also no `readinessProbe`/`livenessProbe` entries — Kubernetes has
  no built-in UDP probe type and the server exposes no HTTP endpoint to probe. The kustomization
  targets the `janxyz` namespace (not a dedicated namespace for this repo) because the deploying
  service account's RBAC is scoped there, matching the `lilith`/`deckard` services deployed from the
  separate `janxyz` monorepo.
- `.github/workflows/server-build-and-deploy.yml` builds/pushes to GitHub Container Registry
  (`ghcr.io/<owner>/walking-in-bevy-server`, auth via the built-in `GITHUB_TOKEN`, no registry
  secrets needed) and deploys via `kubectl apply -k k8s/`, gated on a `secrets.KUBECONFIG` (base64
  kubeconfig). It auto-bumps a `server-vX.Y.Z` git tag on every push to `main` that touches
  `client/**`, `server/**`, `shared/**`, `Cargo.toml`/`Cargo.lock`, `Dockerfile`, or `k8s/**`, then
  commits the resolved image tag back into `k8s/kustomization.yaml` via `kustomize edit set image`.
  This mirrors the versioning pattern used by the `lilith`/`deckard` services in the separate
  `janxyz` monorepo, adapted for a single-service repo (no path-based job filtering needed).
- Local dev/testing: `make server-binary` builds the release binary into `dist/` (Linux x86_64 host
  only, see above), `make docker-build` builds the image, `make docker-run` runs it with UDP 5000
  published, `make k8s-render` runs `kubectl kustomize k8s/` to sanity-check the manifests without
  applying them, `make k8s-deploy` applies them to whatever cluster your current kubeconfig context
  points at.
