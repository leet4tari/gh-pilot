# syntax = docker/dockerfile:1.3

ARG RUST_VERSION=1.66
# rust source compile with cross platform build support
FROM --platform=$BUILDPLATFORM rust:$RUST_VERSION-bullseye as builder

ARG BUILDPLATFORM
ARG BUILDOS
ARG BUILDARCH
ARG BUILDVARIANT
ARG TARGETPLATFORM
ARG TARGETOS
ARG TARGETARCH
ARG TARGETVARIANT
ARG RUST_TOOLCHAIN
ARG VERSION=0.0.1
ARG CARGO_FLAGS="--release --locked"

COPY . .

RUN cargo build $CARGO_FLAGS

# Create runtime base minimal image for the target platform executables
FROM --platform=$TARGETPLATFORM bitnami/minideb:bullseye as runtime

ARG BUILDPLATFORM
ARG TARGETOS
ARG TARGETARCH
ARG TARGETVARIANT

ARG VERSION

COPY --from=builder target/release/ghp /usr/local/bin/
COPY --from=builder target/release/ghp-server /usr/local/bin/

ENTRYPOINT [ "ghp-server" ]
#CMD [ "--non-interactive-mode" ]
