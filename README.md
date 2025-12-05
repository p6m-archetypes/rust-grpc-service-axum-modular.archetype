# Rust gRPC Service (Modular, Axum)

This archetype generates production-ready microservices using Rust, Tonic, and Axum. It creates modular, layered services with optional database persistence.

## Rendering

To generate a project from this archetype, install the [Archetect CLI](https://archetect.github.io/) and run:

```sh
archetect render git@github.com:p6m-archetypes/rust-grpc-service-axum-modular.archetype.git
```

## Core Capabilities

The archetype creates services featuring a modular architecture with cleanly separated layers:

- **Binary Crate**: CLI, configuration loading, and operational controls
- **Client Crate**: gRPC client library for integration tests and external consumers
- **Core Crate**: Business logic implementing the gRPC service trait
- **Server Crate**: Network server exposing the Core via HTTP/2
- **Persistence Crate** (optional): Database access with SeaORM and migrations

## Configuration Options

When generating a service, you specify:

- **Organization name**: Your organization identifier
- **Solution name**: The broader solution this service belongs to
- **Prefix/Suffix names**: Used to construct the service name (e.g., "example-service")
- **Persistence**: PostgreSQL, MySQL, MSSQL, or None

## Technology Foundation

Built on modern Rust ecosystem components:

- **[Tonic](https://github.com/hyperium/tonic)**: gRPC implementation with async/await
- **[Axum](https://github.com/tokio-rs/axum)**: Web framework for management endpoints
- **[SeaORM](https://www.sea-ql.org/SeaORM/)**: Async ORM with migrations (when persistence is enabled)
- **[Tokio](https://tokio.rs/)**: Async runtime
- **Protocol Buffers**: Contract-first API definitions

## Project Structure

```
example-service/
├── crates/
│   ├── example_service_bin/       # Binary with CLI and configuration
│   ├── example_service_client/    # Generated gRPC client library
│   ├── example_service_core/      # Business logic and gRPC trait impl
│   ├── example_service_server/    # HTTP/2 server with health checks
│   └── example_service_persistence/  # Database layer (optional)
├── specs/                         # Protocol Buffer definitions
└── .platform/                     # Docker and deployment configs
```

## Features

- **Layered Architecture**: Each layer can be embedded and tested independently
- **Configuration Management**: YAML config files with environment variable overrides
- **CLI Interface**: Built-in commands for migrations, config inspection, and server operation
- **Health Checks**: gRPC health service for Kubernetes probes
- **Reflection**: gRPC reflection for debugging with tools like grpcurl
- **Tracing**: Structured logging with configurable formats and filters

## Use Cases

This archetype is ideal for:

- Internal microservices requiring type-safe RPC
- Domain services in a platform architecture
- Services that may or may not need database persistence
- Teams adopting Rust for high-performance backend services

## Requirements

- [Archetect CLI](https://archetect.github.io/) 2.0.7+
- [Rust](https://rustup.rs/) toolchain
- [protoc](https://grpc.io/docs/protoc-installation/) Protocol Buffer compiler
- Docker (for integration tests with databases)
