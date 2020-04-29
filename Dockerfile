### NOTES ###
#
# This is a development dockerfile optimized to :
#   - Reduce the build time: non-project binaries are cached
#   - Reduce the image space: the project is installed as a binary runnable from scratch image
#

ARG RUST_VERSION=1.43.0
ARG LINUX=alpine

# Select build image
FROM rust:${RUST_VERSION}-${LINUX} AS builder

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

# Create a new empty shell project
RUN USER=root cargo new --bin crawler
WORKDIR /crawler

# Copy the manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build step to cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy the tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/crawler*
RUN cargo build --release

# Install binaries to run on scratch
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=builder /usr/local/cargo/bin/crawler .
USER 1000
CMD ["./crawler"]