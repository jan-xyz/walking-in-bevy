.PHONY: build test fmt lint run run-server run-client docker-build docker-run k8s-render k8s-deploy clean

IMAGE ?= walking-in-bevy-server:latest

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

docker-build:
	docker build -t $(IMAGE) .

docker-run: docker-build
	docker run --rm -p 5000:5000/udp $(IMAGE)

k8s-render:
	kubectl kustomize k8s/

k8s-deploy:
	kubectl apply -k k8s/

clean:
	cargo clean
