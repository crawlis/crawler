### NOTES ###
#
# This is a development dockerfile optimized to :
#   - Reduce the build time: non-project binaries are cached
#   - Reduce the image space: the project is installed as a binary runnable from scratch image
#

ARG RUST_VERSION=stable

# Select build image
FROM clux/muslrust:stable as builder
RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 dockeruser
# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

# Create a new empty shell project

RUN USER=root cargo new --bin --vcs none crawler
WORKDIR /volume/crawler

# Copy the manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build step to cache dependencies
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm src/*.rs && \
    rm -rf ./target/x86_64-unknown-linux-musl/release/deps/crawler*

# Copy the tree
COPY ./src ./src

# Install binaries to run on scratch
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN mkdir /build-out && \
    cp ./target/x86_64-unknown-linux-musl/release/crawler /build-out/

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=builder /build-out/crawler .
USER 1000
CMD ["./crawler"]