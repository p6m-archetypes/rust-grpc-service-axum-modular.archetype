FROM lukemathwalker/cargo-chef:latest-rust-1.68.0 AS chef
WORKDIR /build

FROM debian:bullseye-slim
WORKDIR /app
RUN apt update && apt-get install libssl-dev libssl1.1 -y
COPY [".platform/certificates", ".platform/certificates"]
RUN ls
COPY target/aarch64-unknown-linux-gnu/debug/platform-application-operator /app/
ENTRYPOINT ["/app/platform-application-operator"]
