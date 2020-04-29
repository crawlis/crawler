#!/bin/bash
set -x

cargo build --target $TARGET --verbose
cargo build --target $TARGET --verbose --release
mkdir -p target/executable
cp target/${TARGET}/debug/crawler target/executable/crawler-${TRAVIS_RUST_VERSION}-${TARGET}-debug
cp target/${TARGET}/release/crawler target/executable/crawler-${TRAVIS_RUST_VERSION}-${TARGET}