.PHONY: all build run-envoy run-sidecar run-service_a run-service_b

build:
	cargo build --release -p sidecar
	cargo build --release -p service_a
	cargo build --release -p service_b

run-envoy:
	envoy -c envoy.yaml

run-sidecar:
	cargo run --release -p sidecar

run-service_a:
	cargo run --release -p service_a

run-service_b:
	cargo run --release -p service_b
