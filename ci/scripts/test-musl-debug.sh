 
#!/bin/bash

# Usage: ./musl-test-debug.sh <MODULE_NAME>
#
# Most parts of this script are comming from:
# https://github.com/emk/rust-musl-builder/blob/master/examples/build-release
#
# Called by `.travis.yml` to build release binaries. We use
# ekidd/rust-musl-builder to make the Linux binaries so that we can run
# them unchanged on any distro, including tiny distros like Alpine (which
# is heavily used for Docker containers).

docker build -t "$1"-debug -f ./ci/dockerfiles/musl-tester-debug.Dockerfile .
docker run -it --name "$1" "$1"-debug
docker rm "$1"
docker rmi "$1"-debug
       