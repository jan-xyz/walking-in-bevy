.PHONY: build test fmt lint run run-server run-client server-binary docker-build docker-run k8s-render k8s-deploy clean

IMAGE ?= walking-in-bevy-server:latest
DIST := dist

build:
	cargo build

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

run:
	cargo run

run-server:
	cargo run --bin server

run-client:
	cargo run --bin client

# Produces a Linux x86_64 binary. Only run this on a Linux x86_64 host (like
# CI) — on other hosts/architectures the binary won't run in the container.
server-binary:
	cargo build --release --bin server
	mkdir -p $(DIST)
	cp target/release/server $(DIST)/server

docker-build: server-binary
	docker build -f Dockerfile -t $(IMAGE) $(DIST)

docker-run: docker-build
	docker run --rm -p 5000:5000/udp $(IMAGE)

k8s-render:
	kubectl kustomize k8s/

k8s-deploy:
	kubectl apply -k k8s/

clean:
	cargo clean
	rm -rf $(DIST)
