 
#!/bin/bash

# Usage: ./musl-build-debug.sh <MODULE_NAME>
#
# Most parts of this script are comming from:
# https://github.com/emk/rust-musl-builder/blob/master/examples/build-release
#
# Called by `.travis.yml` to build release binaries. We use
# ekidd/rust-musl-builder to make the Linux binaries so that we can run
# them unchanged on any distro, including tiny distros like Alpine (which
# is heavily used for Docker containers).

mkdir -p ./target/x86_64-unknown-linux-musl/release

docker build -t "$1"-debug -f ./ci/dockerfiles/musl-builder-debug.Dockerfile .
docker run -it --name "$1" "$1"-debug
docker cp "$1":/home/rust/src/target/x86_64-unknown-linux-musl/debug/"$1" ./target/x86_64-unknown-linux-musl/debug/"$1"
docker rm "$1"
docker rmi "$1"-debug
       