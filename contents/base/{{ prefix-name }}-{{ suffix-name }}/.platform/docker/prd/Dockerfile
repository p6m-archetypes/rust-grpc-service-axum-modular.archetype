FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json;

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Install non-cargo dependencies of the build dependencies here.
# rustfmt is required by tonic-reflection's build script
RUN rustup component add rustfmt

# Build dependencies - this is the caching Docker layer!
ARG PROFILE=release
ENV CARGO_REGISTRIES_YBOR_STUDIO_CREDENTIAL_PROVIDER=cargo:token
RUN --mount=type=secret,id=ybor-studio,env=CARGO_REGISTRIES_YBOR_STUDIO_TOKEN \
  cargo chef cook --profile $PROFILE --recipe-path recipe.json;

# Build application
COPY . .
RUN --mount=type=secret,id=ybor-studio,env=CARGO_REGISTRIES_YBOR_STUDIO_TOKEN \
  cargo build --profile $PROFILE --bin {{ project-name }};

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Install OS packages that are runtime dependencies here.
RUN apt-get update && apt-get install --assume-yes libssl-dev ca-certificates

ARG PROFILE_DIR=release
COPY --from=builder /app/target/$PROFILE_DIR/{{ project-name }} /usr/local/bin
ENTRYPOINT ["/usr/local/bin/{{ project-name }}"]
