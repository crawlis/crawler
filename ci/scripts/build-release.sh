#!/bin/bash

set -euo pipefail

if [ $TARGET == "x86_64-unknown-linux-musl" ]
then
  ./ci/scripts/build-musl-release.sh "$1"
else
  cargo build --target ${TARGET} --verbose --release
fi