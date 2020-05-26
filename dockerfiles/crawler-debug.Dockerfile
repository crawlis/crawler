# This is a development dockerfile optimized to :
#   - Reduce the build time: non-project binaries are cached
#   - Reduce the image space: the project is installed as a binary runnable from scratch image

ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

##################################################
#                                                #
# BUILDER                                        #
#                                                #
# Debian-based image for openssl compillation    #
#                                                #
##################################################
FROM ${BASE_IMAGE} as builder

RUN rustup self update
RUN rustup target add x86_64-unknown-linux-musl

# Create a new empty shell project to cache dependencies
RUN USER=root cargo new --bin --vcs none crawler
WORKDIR /home/rust/src/crawler
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --target x86_64-unknown-linux-musl
RUN rm src/*.rs && \
    rm -rf ./target/x86_64-unknown-linux-musl/debug/deps/crawler*

# Install the binary
COPY ./src ./src
RUN cargo build --target x86_64-unknown-linux-musl
RUN chmod +x ./target/x86_64-unknown-linux-musl/debug/crawler

##################################################
#                                                #
# SCRATCH                                        #
#                                                #
# Empty image to execute binary                  #
#                                                #
##################################################
FROM scratch

# Adding the binary
COPY --from=builder /home/rust/src/crawler/target/x86_64-unknown-linux-musl/debug/crawler .

# Adding SSL certificates
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
CMD ["./crawler"]