FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /build

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install --assume-yes libssl-dev ca-certificates
COPY target/aarch64-unknown-linux-gnu/debug/{{ project-name }} /app/
ENTRYPOINT ["/app/{{ project-name }}"]
