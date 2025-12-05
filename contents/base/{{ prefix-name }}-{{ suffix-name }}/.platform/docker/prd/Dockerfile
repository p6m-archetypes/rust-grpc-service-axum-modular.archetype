FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json;

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

# Install non-cargo dependencies of the build dependencies here.

# Build dependencies - this is the caching Docker layer!
ARG PROFILE=release
ENV CARGO_REGISTRIES_P6M_RUN_CREDENTIAL_PROVIDER=cargo:token
RUN --mount=type=secret,id=p6m-run,env=CARGO_REGISTRIES_P6M_RUN_TOKEN \
    cargo chef cook --profile $PROFILE --recipe-path recipe.json;

# Build application
COPY . .
RUN cargo build --profile $PROFILE --bin platform-application-operator;

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Install OS packages that are runtime dependencies here.
RUN apt-get update && apt-get install --assume-yes libssl-dev ca-certificates

ARG PROFILE_DIR=release
COPY --from=builder /app/target/$PROFILE_DIR/platform-application-operator /usr/local/bin
ENTRYPOINT ["/usr/local/bin/platform-application-operator"]
