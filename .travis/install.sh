#!/bin/bash
set -x

export PATH="$PATH:$HOME/.cargo/bin"
rustup target add $TARGET || true