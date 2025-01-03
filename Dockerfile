# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.83
ARG ALPINE_VERSION=3.21
ARG APP_NAME=rustwester

################################################################################
# Create a stage for building the application.
FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} AS build
ARG APP_NAME
ARG TARGET=x86_64-unknown-linux-musl

WORKDIR /app

# Install host build dependencies.
RUN apk update && apk upgrade --no-cache \
    && apk add --no-cache build-base clang file git libcrypto3 libssl3 lld \
        musl-dev openssl-dev openssl-libs-static pkgconfig \
    && rm -rf /var/cache/apk/*

# Copy Cargo.toml and Cargo.lock to cache dependencies.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && touch src/main.rs && cargo fetch --locked

COPY . .

RUN cargo build --locked --release --target ${TARGET} && cp ./target/${TARGET}/release/${APP_NAME} /bin/server

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
# The example below uses the alpine image as the foundation for running the app.
# By specifying the "3.18" tag, it will use version 3.18 of alpine. If
# reproducability is important, consider using a digest
# (e.g., alpine@sha256:664888ac9cfd28068e062c991ebcff4b4c7307dc8dd4df9e728bedde5c449d91).
FROM alpine:${ALPINE_VERSION} AS final
LABEL org.opencontainers.image.maintainer="Cristian Iordachescu <iordachescu1996@outlook.com>"
LABEL org.opencontainers.image.version="2.0.0"
LABEL org.opencontainers.image.title="Rustwester"
LABEL org.opencontainers.image.description="This is a Dockerfile for running rustwester. For more information visit run with --help."

RUN apk update && apk upgrade --no-cache && rm -rf /var/cache/apk/*

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# ENV LOG_FILE=/var/log/rustwester.log

# Expose the port that the application listens on.
EXPOSE 9999

# What the container should run when it is started.
CMD ["/bin/server"]
