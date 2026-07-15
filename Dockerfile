# syntax=docker/dockerfile:1.7

FROM rust:1.97-slim-bookworm AS build

WORKDIR /src
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libasound2-dev \
        libudev-dev \
        libwayland-dev \
        libxkbcommon-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/src/target \
    cargo build --release --bin server \
    && cp target/release/server /out-server

FROM gcr.io/distroless/cc-debian12:nonroot

COPY --from=build /out-server /server

EXPOSE 5000/udp

ENTRYPOINT ["/server"]
