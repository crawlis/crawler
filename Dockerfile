ARG RUST_VERSION=1.43.0
ARG LINUX=alpine

FROM rust:${RUST_VERSION}-${LINUX} AS builder

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl


WORKDIR /app
COPY ./ ./
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=builder /usr/local/cargo/bin/crawler .
USER 1000
CMD ["./crawler"]